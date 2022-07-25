<script lang="ts">
    import { onMount } from "svelte";
    import { LiteGraph, LGraphCanvas } from "litegraph.js";

    onMount(async () => {
        var graph = new LiteGraph.LGraph();
        var canvas = new LGraphCanvas("#mycanvas", graph);
        graph.start();
    });
</script>

<canvas id="mycanvas" style="border: 1px solid" class="w-full h-full" />
