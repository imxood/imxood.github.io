<script>
    import "../app.css";
    import Titlebar from "$lib/titlebar.svelte";
    import Nav from "./nav.svelte";
</script>

<div class="w-full h-full bg-blue-300">
    <Titlebar />
    <Nav />
    <slot />
</div>
