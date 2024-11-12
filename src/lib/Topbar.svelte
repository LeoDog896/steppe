<script lang="ts">
    // Adapted from https://v2.tauri.app/learn/window-customization/
    import { getCurrentWindow } from '@tauri-apps/api/window';

    const appWindow = getCurrentWindow();
</script>

<nav data-tauri-drag-region class="titlebar">
  <button class="titlebar-button" id="titlebar-minimize" onclick={appWindow.minimize}>
    <img
      src="https://api.iconify.design/mdi:window-minimize.svg"
      alt="minimize"
    />
  </button>
  <button class="titlebar-button" id="titlebar-maximize" onclick={appWindow.toggleMaximize}>
    <img
      src="https://api.iconify.design/mdi:window-maximize.svg"
      alt="maximize"
    />
  </button>
  <button class="titlebar-button" id="titlebar-close" onclick={appWindow.close}>
    <img src="https://api.iconify.design/mdi:close.svg" alt="close" />
  </button>
</nav>

<style lang="scss">
    .titlebar {
        height: 2rem;
        background: gray;
        user-select: none;
        display: flex;
        justify-content: flex-end;
        align-items: center;
        width: 100%;
    }

    .titlebar-button {
        display: inline-flex;
        justify-content: center;
        align-items: center;
        width: 30px;
        height: 30px;
        user-select: none;
        -webkit-user-select: none;
        background: none;
        border: none;
    }

    .titlebar-button:hover {
        background: #5bbec3;
    }
</style>
