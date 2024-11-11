<script lang="ts">
    import { onDestroy, onMount } from "svelte";
    import { Terminal } from '@xterm/xterm'
    import { FitAddon } from '@xterm/addon-fit';
    import { ImageAddon } from '@xterm/addon-image';
    import { ClipboardAddon } from './ClipboardAddon';
    import { invoke } from "@tauri-apps/api/core";
    import FontFaceObserver from 'fontfaceobserver'
    import '@fontsource-variable/jetbrains-mono';

    let terminalElement: HTMLDivElement

    let term: Terminal
    let fitAddon: FitAddon
    let imageAddon: ImageAddon
    let clipboardAddon: ClipboardAddon

    async function fitTerminal() {
        fitAddon.fit();
        invoke<string>("async_resize_pty", {
            rows: term.rows,
            cols: term.cols,
        });
    }

    // Write data from pty into the terminal
    function writeToTerminal(data: string) {
        return new Promise<void>((r) => {
            term.write(data, () => r());
        });
    }

    // Write data from the terminal to the pty
    function writeToPty(data: string) {
        invoke("async_write_to_pty", {
            data,
        });
    }

    async function readFromPty() {
        const data = await invoke<string>("async_read_from_pty");

        if (data) {
            await writeToTerminal(data);
        }

        window.requestAnimationFrame(readFromPty);
    }

    onMount(async () => {
        const font = new FontFaceObserver('Jetbrains Mono Variable', {
            weight: 400
        });

        await font.load()
        
        term = new Terminal({
            fontFamily: "Jetbrains Mono Variable",
            theme: {
                background: "rgb(47, 47, 47)",
            },
        });

        fitAddon = new FitAddon();
        imageAddon = new ImageAddon();
        clipboardAddon = new ClipboardAddon();

        term.loadAddon(fitAddon);
        term.loadAddon(imageAddon);
        term.loadAddon(clipboardAddon);

        term.open(terminalElement);
        term.onData(writeToPty);

        fitAddon.fit();

        invoke("async_create_shell").catch((error: unknown) => {
            // on linux it seem to to "Operation not permitted (os error 1)", yet it still works.
            console.error("Error creating shell:", error);
        });

        window.requestAnimationFrame(readFromPty);
    })

    onDestroy(() => {
        fitAddon.dispose()
        imageAddon.dispose()
        clipboardAddon.dispose()
        term.dispose()
    })
</script>

<svelte:window on:resize={fitTerminal}></svelte:window>

<div bind:this={terminalElement}></div>

<style>
    div {
        width: 100vw;
        height: 100vh;
    }
</style>
