<script lang="ts">
    import Topbar from "$lib/Topbar.svelte";
  import Terminal from "../lib/Terminal.svelte";
</script>

<main>
  <Topbar />

  <Terminal />
</main>

<style lang="scss">
  main {
    width: 100%;
    height: 100%;
    display: flex;
    flex-direction: column;
  }
</style>