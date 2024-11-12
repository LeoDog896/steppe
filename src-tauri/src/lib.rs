#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use deno_runtime::deno_core::{ModuleSpecifier, FsModuleLoader};
use deno_runtime::deno_fs::RealFs;
use deno_runtime::deno_permissions::PermissionsContainer;
use deno_runtime::permissions::RuntimePermissionDescriptorParser;
use deno_runtime::worker::{MainWorker, WorkerOptions, WorkerServiceOptions};
use portable_pty::{native_pty_system, CommandBuilder, PtyPair, PtySize};
use std::fs::{create_dir_all, File};
use std::{
    io::{BufRead, BufReader, Read, Write}, path::Path, process::exit, sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    }, thread::{self}, rc::Rc, path::PathBuf
};

use tauri::{async_runtime::Mutex as AsyncMutex, State};

struct SubTerminal {
    pty_pair: Arc<AsyncMutex<PtyPair>>,
    writer: Arc<AsyncMutex<Box<dyn Write + Send>>>,
    reader: Arc<AsyncMutex<BufReader<Box<dyn Read + Send>>>>,
    has_terminal: AtomicBool,
}

struct AppState {
    pty_pair: Arc<AsyncMutex<PtyPair>>,
    writer: Arc<AsyncMutex<Box<dyn Write + Send>>>,
    reader: Arc<AsyncMutex<BufReader<Box<dyn Read + Send>>>>,
    has_terminal: AtomicBool,
}

#[tauri::command]
async fn async_create_shell(state: State<'_, AppState>) -> Result<(), String> {
    if state.has_terminal.load(Ordering::Acquire) {
        return Ok(());
    }

    #[cfg(target_os = "windows")]
    let mut cmd = CommandBuilder::new("powershell.exe");

    #[cfg(not(target_os = "windows"))]
    let mut cmd = {
        let path = std::env::var("SHELL").map_err(|_| "Could not grab preferred shell from $SHELL")?;
        CommandBuilder::new(path)
    };

    // add the $TERM env variable

    #[cfg(target_os = "windows")]
    cmd.env("TERM", "cygwin");

    #[cfg(not(target_os = "windows"))]
    cmd.env("TERM", "xterm-256color");

    let mut child = state
        .pty_pair
        .lock()
        .await
        .slave
        .spawn_command(cmd)
        .map_err(|err| err.to_string())?;

    thread::spawn(move || {
        let status = child.wait().unwrap();
        exit(status.exit_code() as i32)
    });

    state.has_terminal.store(true, Ordering::Release);

    Ok(())
}

#[tauri::command]
async fn async_write_to_pty(data: &str, state: State<'_, AppState>) -> Result<(), ()> {
    write!(state.writer.lock().await, "{}", data).map_err(|_| ())
}

#[tauri::command]
async fn async_read_from_pty(state: State<'_, AppState>) -> Result<Option<String>, ()> {
    let mut reader = state.reader.lock().await;
    let data = {
        // Read all available text
        let data = reader.fill_buf().map_err(|_| ())?;

        // Send te data to the webview if necessary
        if data.len() > 0 {
            std::str::from_utf8(data)
                .map(|v| Some(v.to_string()))
                .map_err(|_| ())?
        } else {
            None
        }
    };

    if let Some(data) = &data {
        reader.consume(data.len());
    }

    Ok(data)
}

#[tauri::command]
async fn async_resize_pty(rows: u16, cols: u16, state: State<'_, AppState>) -> Result<(), ()> {
    state
        .pty_pair
        .lock()
        .await
        .master
        .resize(PtySize {
            rows,
            cols,
            ..Default::default()
        })
        .map_err(|_| ())
}

fn get_config_dir() -> PathBuf {
    Path::new(&std::env::var("XDG_CONFIG_HOME").unwrap_or(
        Path::new(&std::env::var("HOME").unwrap()).join(".config").to_string_lossy().to_string()
    )).join("steppe")
}

fn get_config_path() -> PathBuf {
    get_config_dir().join("config.js")
}

fn write_default_config(path: &PathBuf) {
    if let Some(parent) = path.parent() {
        create_dir_all(parent).unwrap();
    }

    File::create_new(path).unwrap().write(b"export {}").unwrap();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let path = get_config_path();
    if !path.exists() {
        write_default_config(&path);
    }

    // deno boilerplate from https://github.com/denoland/deno/blob/main/runtime/examples/extension/main.rs
    let main_module = ModuleSpecifier::from_file_path(get_config_path()).unwrap();

    let fs = Arc::new(RealFs);

    let permission_desc_parser =
        Arc::new(RuntimePermissionDescriptorParser::new(fs.clone()));

    let mut worker = MainWorker::bootstrap_from_options(
        main_module.clone(),
        WorkerServiceOptions {
            module_loader: Rc::new(FsModuleLoader),
            permissions: PermissionsContainer::allow_all(permission_desc_parser),
            blob_store: Default::default(),
            broadcast_channel: Default::default(),
            feature_checker: Default::default(),
            node_services: Default::default(),
            npm_process_state_provider: Default::default(),
            root_cert_store_provider: Default::default(),
            shared_array_buffer_store: Default::default(),
            compiled_wasm_module_store: Default::default(),
            v8_code_cache: Default::default(),
            fs,
        },
        WorkerOptions {
            ..Default::default()
        },
    );

    let pty_system = native_pty_system();

    let pty_pair = pty_system
        .openpty(PtySize {
            rows: 24,
            cols: 80,
            pixel_width: 0,
            pixel_height: 0,
        })
        .unwrap();

    let reader = pty_pair.master.try_clone_reader().unwrap();
    let writer = pty_pair.master.take_writer().unwrap();

    tauri::Builder::default()
        .plugin(tauri_plugin_clipboard_manager::init())
        .manage(AppState {
            pty_pair: Arc::new(AsyncMutex::new(pty_pair)),
            writer: Arc::new(AsyncMutex::new(writer)),
            reader: Arc::new(AsyncMutex::new(BufReader::new(reader))),
            has_terminal: AtomicBool::new(false),
        })
        .invoke_handler(tauri::generate_handler![
            async_write_to_pty,
            async_resize_pty,
            async_create_shell,
            async_read_from_pty
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
