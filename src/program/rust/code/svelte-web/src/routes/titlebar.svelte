<script>
    import { appWindow } from "@tauri-apps/api/window";

    async function minimize() {
        console.log("minimize");
        await appWindow.minimize();
    }
    
    async function maximize() {
        console.log("toggleMaximize");
        await appWindow.toggleMaximize();
    }

    async function close() {
        console.log("close");
        await appWindow.close();
    }
</script>

<div data-tauri-drag-region class="flex flex-row-reverse bg-blue-500">
    <img
        class="px-2 py-1 h-6 hover:bg-slate-300"
        src="mdi_close.svg"
        alt="close"
        on:click={close}
    />
    <img
        class="px-2 py-1 h-6 hover:bg-slate-300"
        src="mdi_window-maximize.svg"
        alt="maximize"
        on:click={maximize}
    />
    <img
        class="px-2 py-1 h-6 hover:bg-slate-300"
        src="mdi_window-minimize.svg"
        alt="minimize"
        on:click={minimize}
    />
</div>
