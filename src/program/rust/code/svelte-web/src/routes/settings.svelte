<script lang="ts">
    import Drawflow from "drawflow";

    import styleDrawflow from "drawflow/dist/drawflow.min.css";

    import { onMount } from "svelte";

    let editor_ele: HTMLElement;

    onMount(async () => {
        const editor = new Drawflow(editor_ele);
        editor.start();
    });
</script>

<button class="btn">settings</button>
<div class="drawflow max-h-[90%]" bind:this={editor_ele} />
