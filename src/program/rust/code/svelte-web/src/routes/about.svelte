<script>
    import { onMount, onDestroy } from "svelte";
    onMount(async () => {
        console.log("onMount -- about");
    });

    onDestroy(async () => {
        console.log("onDestroy -- about");
    });
</script>

<button class="btn">about</button>
