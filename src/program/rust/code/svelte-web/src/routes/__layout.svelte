<script>
    import "../app.pcss";
    import Titlebar from "./titlebar.svelte"
</script>

<Titlebar />

<nav>
    <a href="/">Home</a>
    <a href="/about">About</a>
    <a href="/settings">Settings</a>
</nav>

<slot />
