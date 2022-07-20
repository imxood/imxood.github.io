<script>
    import "../app.css";
    import Titlebar from "./titlebar.svelte";
</script>

<div class="w-full h-full bg-blue-300">
    <Titlebar />
    <div class="" />
    <nav>
        <a href="/">Home</a>
        <a href="/about">About</a>
        <a href="/settings">Settings</a>
    </nav>

    <slot />
</div>
