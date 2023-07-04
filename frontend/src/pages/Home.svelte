<script lang="ts">
    import Card from "../components/Card.svelte";
    import Graph from "../components/Graph.svelte";
    import prettyBytes from "pretty-bytes";
    import "uplot/dist/uPlot.min.css";

    import { statisticsStore } from "../websocket";

    export let darkMode: boolean;
    export let tempUnit: "fahrenheit" | "celsius";

    let portrait = window.innerHeight > window.innerWidth;

    function getTempMsg(temp: number) {
        if (
            (tempUnit === "celsius" && temp >= 70) ||
            (tempUnit === "fahrenheit" && temp >= 158)
        ) {
            return "WARNING: Reducing the life of your device";
        } else if (
            (tempUnit === "celsius" && temp >= 60) ||
            (tempUnit === "fahrenheit" && temp >= 140)
        ) {
            return "Running hot, not recommended";
        } else if (
            (tempUnit === "celsius" && temp >= 50) ||
            (tempUnit === "fahrenheit" && temp >= 122)
        ) {
            return "Running warm, but safe";
        } else if (
            (tempUnit === "celsius" && temp >= 40) ||
            (tempUnit === "fahrenheit" && temp >= 104)
        ) {
            return "Optimal temperature";
        } else if (
            (tempUnit === "celsius" && temp >= 30) ||
            (tempUnit === "fahrenheit" && temp >= 86)
        ) {
            return "Cool runnings";
        } else {
            return "Who put me in the freezer!";
        }
    }

    function getTempClass(temp: number) {
        if (
            (tempUnit === "celsius" && temp >= 70) ||
            (tempUnit === "fahrenheit" && temp >= 158)
        ) {
            return "text-red-500 font-semibold";
        } else if (
            (tempUnit === "celsius" && temp >= 60) ||
            (tempUnit === "fahrenheit" && temp >= 140)
        ) {
            return "text-red-500";
        } else if (
            (tempUnit === "celsius" && temp >= 50) ||
            (tempUnit === "fahrenheit" && temp >= 122)
        ) {
            return "text-yellow-500";
        } else if (
            (tempUnit === "celsius" && temp >= 40) ||
            (tempUnit === "fahrenheit" && temp >= 104)
        ) {
            return "text-green-500";
        } else if (
            (tempUnit === "celsius" && temp >= 30) ||
            (tempUnit === "fahrenheit" && temp >= 86)
        ) {
            return "text-blue-500";
        } else {
            return "text-blue-500 font-semibold";
        }
    }
</script>

<svelte:window on:resize={() => (portrait = window.innerHeight > window.innerWidth)} />

<main
    class="flex gap-5 flex-wrap min-h-full flex-col flex-grow"
    class:md:flex-row={!portrait}
>
    <Card header="System Diagnostics">
        <Graph {darkMode} {tempUnit} {portrait} />
    </Card>
    <Card header="System Stats">
        {#if $statisticsStore.temp.temp !== null}
            <div class="text-center">
                <span class={getTempClass($statisticsStore.temp.temp)}>
                    {$statisticsStore.temp.temp}{tempUnit === "celsius"
                        ? "ºC"
                        : "ºF"}</span
                >: {getTempMsg($statisticsStore.temp.temp)}
            </div>
            CPU:<span class="float-right">{$statisticsStore.cpu}/100%</span>
            <div class="bg-gray-200 dark:bg-gray-800 w-full h-3 my-1">
                <div
                    class="bg-green-500 h-3 transition-width-200"
                    style="width:{$statisticsStore.cpu}%"
                />
            </div>
            RAM:<span class="float-right"
                >{prettyBytes($statisticsStore.ram.used, { binary: true })}/{prettyBytes(
                    $statisticsStore.ram.total,
                    { binary: true }
                )}</span
            >
            <div class="bg-gray-200 dark:bg-gray-800 w-full h-3 my-1">
                <div
                    class="bg-red-500 h-3 transition-width-200"
                    style="width:{$statisticsStore.ram.percent}%"
                />
            </div>
            Swap:<span class="float-right"
                >{prettyBytes($statisticsStore.swap.used, { binary: true })}/{prettyBytes(
                    $statisticsStore.swap.total,
                    { binary: true }
                )}</span
            >
            <div class="bg-gray-200 dark:bg-gray-800 w-full h-3 my-1">
                <div
                    class="bg-blue-500 h-3 transition-width-200"
                    style="width:{$statisticsStore.swap.percent}%"
                />
            </div>
            Disk:<span class="float-right"
                >{prettyBytes($statisticsStore.disk.used)}/{prettyBytes(
                    $statisticsStore.disk.total
                )}</span
            >
            <div class="bg-gray-200 dark:bg-gray-800 w-full h-3 my-1">
                <div
                    class="bg-yellow-500 h-3 transition-width-200"
                    style="width:{$statisticsStore.disk.percent}%"
                />
            </div>
        {/if}
    </Card>
</main>
