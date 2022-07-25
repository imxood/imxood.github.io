<script>
    export let boards;
    export let alignment;
    import Drawflow from "drawflow";
    import { onMount, onDestroy } from "svelte";
    import "drawflow/dist/drawflow.min.css";
    import Magnify from "svelte-material-icons/Magnify.svelte";
    import SearchModal from "./SearchModal.svelte";
    import { matchSorter } from "match-sorter";
    import { error } from "../error/state.js";
    import { updateAlignment } from "../../graphql/mutations.js";
    import { callAPI } from "../../io.js";

    let showSearch = false;
    let container;
    let editor;
    let loaded = false;

    const flatBoards = (boards) => {
        let objectives = [];
        boards.forEach((board) =>
            board.views.items.forEach((view) =>
                objectives.push(view.objectives.items)
            )
        );
        return objectives.flat();
    };

    const calculateProgress = (data) => {
        try {
            const val =
                ((data.currentValue - data.startValue) * 100) /
                (data.targetValue - data.startValue);
            if (val === Infinity) {
                return 0;
            } else {
                return val.toFixed(0);
            }
        } catch (e) {}
    };

    const generateOwnerInitials = (user) => {
        try {
            if (user !== undefined) {
                return (
                    ((user || {}).firstName || "").charAt(0) +
                    ((user || {}).lastName || "").charAt(0)
                );
            } else {
                return undefined;
            }
        } catch (e) {
            console.log(e);
            return "";
        }
    };

    const keyResultHTML = (keyResult) => {
        const name = keyResult.name;
        const owners = keyResult?.owners?.items?.map((v) => v.owner);
        const owner = owners && head(owners);
        const initials = owner && generateOwnerInitials(owner);
        const percentage = calculateProgress(keyResult);
        return `
      <ul class="">
        <li class="flex items-center gap-3 mb-3">
        <button title="${owner?.firstName} ${owner?.lastName}" class='${
            initials
                ? "text-white w-8 h-8 rounded-full flex items-center justify-center bg-tint-800 bg-gradient-to-r from-cyan-500 to-blue-500"
                : "rounded-full bg-tint-800 w-8 h-8"
        }'>${initials ? initials : ""}</button>
        <div>
        <p class="text-tint-300 text-sm" title="${name}">${name.substring(
            0,
            50
        )}${name.length > 50 ? "..." : ""}</p>
        <div class="flex gap-4 items-center">
        <div class="bg-neutural-000 min-h-[12px] min-w-[12rem]">
        <div style="width: ${percentage}%" class="bg-accent-green min-h-[12px]"></div>
        </div>
        <p class="">${percentage}%</p>
        </div>
        </div>
        </div>
        </div>
        </div>
        </li>
      </ul>
        `;
    };

    const createHTMLLoad = (objective) => {
        return objective.html;
        const objectiveId = objective.name;
        const data = objective.data;
        const o = flatBoards(boards);
        const obj = o.filter((obj) => obj.id === objectiveId);
        if (obj.length > 0) {
            const first = head(obj);
            const updatedAt = first?.keyResults?.items?.map(
                (kr) => kr.updatedAt
            );
            const sorted = updatedAt && head(sortByDate(updatedAt));
            const lastUpdate = sorted && sorted.substring(0, 10);
            return `
      <div  class="">
  <!-- ARCHIVED -->
      <h5 class="text-tint-300 font-bold text-base mb-2">${first.name}</h5>
          <p class="text-tint-600 mb-2">Updated ${lastUpdate}</p>
      <ul class="">
        ${first.keyResults.items.map(keyResultHTML).join("")}
      <ul>
      </div>
   `;
        } else {
            const archived = `
  <div  class="text-right">
        <span title="The current objective is archvied or deleted" class="w-4 h-4 bg-accent-rose text-white p-1 rounded-full font-bold px-2">Archived</span>
  </div>
  `;
            return objective.html.replace("<!-- ARCHIVED -->", archived);
        }
    };

    const updateNodes = (data) => {
        const modified = Object.keys(data).map((key) => ({
            ...data[key],
            html: createHTMLLoad(data[key]),
        }));
        const reduced = modified.reduce(
            (a, v, i) => ({ ...a, [i + 1]: v }),
            {}
        );
        return reduced;
    };

    const loadJSON = (alignment) => {
        let json = JSON.parse(alignment.data);
        try {
            let data = (((json || {}).drawflow || {}).Home || {}).data;
            json.drawflow.Home.data = updateNodes(data);
            editor && editor.import(json);
            loaded = true;
        } catch (e) {
            console.log("error message", e.message);
            $error = e.message;
        }
    };

    const saveData = async (e, d) => {
        const data = editor.export();
        alignment.data = JSON.stringify(data);
        try {
            return await callAPI(updateAlignment, {
                input: {
                    organizationId: window.user.profile,
                    id: alignment.id,
                    data: JSON.stringify(data),
                },
            });
        } catch (e) {
            console.log(e.message);
            $error = e.message;
        }
    };

    onMount(async () => {
        try {
            editor = new Drawflow(container);
            editor.reroute = true;
            editor.start();
            editor.zoom_max = 1.6;
            editor.zoom_min = 0.5;
            editor.zoom_value = 0.1;
            editor.zoom = alignment.lastZoomValue || 0.7;
            editor.force_first_input = true;
            editor.zoom_refresh();
            editor.on("nodeCreated", async () => await saveData());
            editor.on("nodeRemoved", async () => await saveData());
            editor.on("nodeMoved", async () => await saveData());
            editor.on("connectionCreated", async () => await saveData());
            editor.on("connectionRemoved", async () => await saveData());
        } catch (e) {
            console.log(e.message);
        }
    });

    $: alignment.data &&
        boards.length > 0 &&
        editor &&
        !loaded &&
        loadJSON(alignment);

    const handleZoomOut = () => {
        editor.zoom_out();
        alignment.lastZoomValue = editor.zoom_last_value;
    };
    const handleZoomIn = () => {
        editor.zoom_in();
        alignment.lastZoomValue = editor.zoom_last_value;
    };

    const sortByDate = (xs) => xs.sort((a, b) => new Date(b) - new Date(b));

    const head = (xs) => {
        try {
            return xs[0];
        } catch (e) {
            return null;
        }
    };

    const createHTML = (objectiveId) => {
        const o = flatBoards(boards);
        const obj = o.filter((obj) => obj.id === objectiveId);
        if (obj.length > 0) {
            const first = head(obj);
            const updatedAt = first?.keyResults?.items?.map(
                (kr) => kr.updatedAt
            );
            const sorted = updatedAt && head(sortByDate(updatedAt));
            const lastUpdate = sorted && sorted.substring(0, 10);
            console.log("lastUpdate", lastUpdate);
            return `
      <div  class="">
      <h5 class="text-tint-300 font-bold text-base mb-2">${first.name}</h5>
          <p class="text-tint-600 mb-2">Updated ${lastUpdate}</p>
      <ul class="">
        ${first.keyResults.items.map(keyResultHTML).join("")}
      <ul>
      </div>
   `;
        } else {
            console.log("return archived");
        }
    };

    const handleAddObjective = (e) => {
        const objective = e.detail;
        const objectiveId = e.detail.objectiveId;
        editor.addNode(
            objective.objectiveId,
            1,
            1,
            300,
            200,
            "github",
            {},
            createHTML(objective.objectiveId)
        );
    };

    const handleLoad = () => {
        console.log(editor.drawflow);
    };
</script>

{#if showSearch}
    <SearchModal
        bind:showSearch
        {boards}
        on:add-objective={handleAddObjective}
    />
{/if}

<div class="drawflow max-h-[90%]" bind:this={container} />
<div
    class="absolute bottom-10 right-10 text-base text-tint-300 shadow-zoombtn right-0 py-2.5 px-6 flex items-center gap-5 justify-center bg-white rounded s-o2zHGApCCFOx"
>
    <button on:click={handleZoomOut} class="zoom s-o2zHGApCCFOx" id="zoomMinus">
        <svg width="1em" height="1em" viewBox="0 0 24 24">
            <path d="M19,13H5V11H19V13Z" fill="currentColor" /></svg
        ><!--<Minus>-->
    </button>
    <p id="zoomVal" class="s-o2zHGApCCFOx">
        {(alignment?.lastZoomValue
            ? alignment.lastZoomValue * 100
            : 70
        ).toFixed(0)}%
    </p>
    <button on:click={handleZoomIn} class="zoom s-o2zHGApCCFOx" id="zoomPlus">
        <svg width="1em" height="1em" viewBox="0 0 24 24">
            <path
                d="M19,13H13V19H11V13H5V11H11V5H13V11H19V13Z"
                fill="currentColor"
            /></svg
        ><!--<Plus>-->
    </button>
</div>
<div
    class="absolute bottom-10 right-[250px] text-base text-tint-300 shadow-zoombtn right-0 py-2.5 px-6 flex items-center gap-5 justify-center bg-white rounded s-o2zHGApCCFOx"
>
    <button on:click={handleLoad}>Print State</button>
    <button on:click={saveData}>Save State</button>
    <button
        on:click={() => (showSearch = !showSearch)}
        class="flex gap-2 w-[150px] text-tint-500 items-center"
        type=""
        placeholder="Quick search..."
    >
        <!-- Tailwind style search for key results / objectives -->
        <Magnify width={20} height={20} />
        Quick search...
    </button>
</div>

<style>
    #drawflow {
        position: relative;
        border: 1px solid green;
    }

    :global(.drawflow) {
        user-select: none;
    }

    :global(.drawflow_content_node) {
        /*border: 1px solid green !important;*/
        min-width: 400px;
    }
    :global(.drawflow_content_node > * > .title) {
        font-family: "Roboto";
        font-style: normal;
        font-weight: 600;
        font-size: 16px;
        line-height: 121.19%;
        color: #5e6370;
        margin-bottom: 0.5rem;
    }
    :global(.drawflow_content_node > * > .updated) {
        font-family: "Roboto";
        font-style: normal;
        font-weight: 400;
        font-size: 14px;
        line-height: 121.19%;
        color: #a3a6ad;
        margin-bottom: 0.5rem;
    }

    :global(.drawflow .drawflow-node) {
        background: white !important;
        border: none !important;
        /*border: 1px solid red !important;*/
        min-width: 450px;
        background: #ffffff;
        box-shadow: 0px 10px 20px rgba(0, 0, 0, 0.15);
        border-radius: 3px;
    }

    :global(.drawflow .drawflow-node .input) {
        width: 1.5rem;
        height: 1.5rem;
        background: #ffffff;
        /* Primary/Tint/800 */
        border: 2px solid #e8e9eb;
        margin-left: 0px;
    }
    :global(.drawflow .drawflow-node .output) {
        background: #ffffff;
        /* Primary/Tint/800 */
        border: 2px solid #e8e9eb;
        width: 1.5rem;
        height: 1.5rem;
        margin-left: 0px;
    }
    :global(.bar-zoom) {
        float: right;
        position: absolute;
        bottom: 10px;
        right: 10px;
        display: flex;
        font-size: 24px;
        color: white;
        padding: 5px 10px;
        background: #555555;
        border-radius: 4px;
        border-right: 1px solid var(--border-color);
        z-index: 5;
    }
    :global(.bar-zoom svg) {
        cursor: pointer;
        padding-left: 10px;
    }
    :global(.bar-zoom svg:nth-child(1)) {
        padding-left: 0px;
    }
    :global(.drawflow-delete) {
        background-color: white;
        border: 1px solid #d1d2d6;
        color: #5e6370;
        text-align: center;
        font-size: 16px;
    }
    :global(.drawflow .connection .arrow) {
        transform: translate(-10005px, -9999px);
    }
</style>
