<script lang="ts">
    import Card from "../components/Card.svelte";
    import { tweened } from "svelte/motion";
    import prettyBytes from "pretty-bytes";
    import uPlot from "uplot";
    import { onMount, onDestroy } from "svelte";

    import type { socketData } from "../types";

    export let socketData: Partial<socketData>;
    export let darkMode: boolean;
    export let tempUnit: "fahrenheit" | "celsius";

    let portrait = window.innerHeight > window.innerWidth;

    let chart: HTMLDivElement;

    const cpuAnimate = tweened(0, {
        duration: 200,
    });

    const ramAnimate = tweened(0, {
        duration: 200,
    });

    const swapAnimate = tweened(0, {
        duration: 200,
    });

    const diskAnimate = tweened(0, {
        duration: 200,
    });

    let ramData: (string | number)[],
        swapData: (string | number)[],
        diskData: (string | number)[];

    let data: uPlot.AlignedData = [[], [], [], [], [], [], [], []];

    $: socketData.cpu != undefined &&
        (cpuAnimate.set(socketData.cpu),
        ramAnimate.set(socketData.ram.percent),
        swapAnimate.set(socketData.swap.percent),
        diskAnimate.set(socketData.disk.percent));

    $: socketData.ram &&
        ((ramData = [
            prettyBytes(socketData.ram.used, { binary: true }),
            prettyBytes(socketData.ram.total, { binary: true }),
        ]),
        (swapData = [
            prettyBytes(socketData.swap.used, { binary: true }),
            prettyBytes(socketData.swap.total, { binary: true }),
        ]),
        (diskData = [
            prettyBytes(socketData.disk.used),
            prettyBytes(socketData.disk.total),
        ]));

    function getSize() {
        if (portrait) {
            return {
                width: Math.max(
                    (window.innerWidth / 100) * 70,
                    document.getElementById("chart").getBoundingClientRect()
                        .width - 20
                ),
                height: (window.innerHeight / 100) * 50,
            };
        } else {
            return {
                height: (window.innerHeight / 100) * 70,
                width:
                    document.getElementById("chart").getBoundingClientRect()
                        .width - 20,
            };
        }
    }

    function getTempMsg(temp: number) {
        if (temp >= 70) {
            return "WARNING: Reducing the life of your device";
        } else if (temp >= 60) {
            return "Running hot, not recommended";
        } else if (temp >= 50) {
            return "Running warm, but safe";
        } else if (temp >= 40) {
            return "Optimal temperature";
        } else if (temp >= 30) {
            return "Cool runnings";
        } else {
            return "Who put me in the freezer!";
        }
    }

    function getTempClass(temp: number) {
        if (temp >= 70) {
            return "font-semibold text-red-500";
        } else if (temp >= 60) {
            return "text-red-500";
        } else if (temp >= 50) {
            return "text-yellow-500";
        } else if (temp >= 40) {
            return "text-green-500";
        } else {
            return "text-blue-500";
        }
    }

    let uplot: uPlot;

    onMount(() => {
        let opts: uPlot.Options = {
            ...getSize(),
            series: [
                {},
                {
                    spanGaps: false,
                    label: "CPU",
                    stroke: "#10b981",
                    width: 3,
                    scale: "%",
                    value: (_: any, val: number) => val.toFixed(2) + "%",
                },
                {
                    spanGaps: false,
                    label: "RAM",
                    stroke: "#ef4444",
                    width: 3,
                    scale: "mb",
                    value: (_: any, val: number) =>
                        prettyBytes(val * 1000000, { binary: true }),
                },
                {
                    show: true,
                    spanGaps: false,
                    label: "Swap",
                    stroke: "#3b82f6",
                    width: 3,
                    scale: "mb",
                    value: (_: any, val: number) =>
                        prettyBytes(val * 1000000, { binary: true }),
                },
                {
                    spanGaps: false,
                    label: "Disk",
                    stroke: "#eab308",
                    width: 2,
                    scale: "mb",
                    value: (_: any, val: number) => prettyBytes(val * 1000000),
                },
                {
                    spanGaps: false,
                    label: "Network (sent)",
                    stroke: "#a855f7",
                    width: 3,
                    scale: "mb",
                    value: (_: any, val: number) => prettyBytes(val * 1000000),
                },
                {
                    spanGaps: false,
                    label: "Network (received)",
                    stroke: "#ec4899",
                    width: 3,
                    scale: "mb",
                    value: (_: uPlot, val: number) =>
                        prettyBytes(val * 1000000),
                },
            ],
            axes: [
                {
                    grid: { show: false },
                    stroke: () => (darkMode ? "#fff" : "#000"),
                },
                {
                    scale: "mb",
                    values: (_: any, vals: number[]) =>
                        vals.map((v: number) => +v.toFixed(2) + " MB"),
                    size: 75,
                    grid: { stroke: () => (darkMode ? "#4b5563" : "#ededed") },
                    stroke: () => (darkMode ? "#fff" : "#000"),
                },
                {
                    side: 1,
                    scale: "%",
                    values: (_: any, vals: number[]) =>
                        vals.map((v: number) => +v.toFixed(2) + "%"),
                    grid: { show: false },
                    stroke: "#10b981",
                },
                {
                    side: 1,
                    scale: "deg",
                    values: (_: any, vals: number[]) =>
                        vals.map(
                            (v: number) =>
                                +v + (tempUnit == "celsius" ? "ºC" : "ºF")
                        ),
                    grid: { show: false },
                    stroke: "#94A3B8",
                    size: 75,
                },
            ],
            scales: {
                "%": {
                    auto: false,
                    // Hide CPU axis when CPU series is disabled
                    range: (u: uPlot) =>
                        u.series[1].show ? [0, 100] : [null, null],
                },
            },
        };

        uplot = new uPlot(opts, data, chart);

        if (socketData.swap != undefined && socketData.swap.total == 0) {
            uplot.setSeries(3, { show: false });
        }
    });

    let handle1 = setInterval(() => {
        let dataPush = data as number[][];
        if (socketData.ram.used != undefined) {
            dataPush[0].push(Math.round(Date.now() / 1000));
            dataPush[1].push(socketData.cpu);
            dataPush[2].push(socketData.ram.used / 1000000);
            dataPush[3].push(socketData.swap.used / 1000000);
            dataPush[4].push(socketData.disk.used / 1000000);
            dataPush[5].push(socketData.network.sent / 1000000);
            dataPush[6].push(socketData.network.received / 1000000);
        }
        if (socketData.temp != undefined && socketData.temp.available) {
            if (uplot.series[7] == undefined) {
                uplot.addSeries({
                    spanGaps: false,
                    label: "CPU Temperature",
                    stroke: "#94A3B8",
                    width: 3,
                    scale: "deg",
                    value: (_: any, val: number) =>
                        val + (tempUnit == "celsius" ? "ºC" : "ºF"),
                });
            }
            if (tempUnit == "celsius") {
                dataPush[7].push(socketData.temp.celsius);
            } else if (tempUnit == "fahrenheit") {
                dataPush[7].push(socketData.temp.fahrenheit);
            }
        }
        uplot.setData(data);
    }, 2000);

    onDestroy(() => {
        uplot = undefined;
        clearInterval(handle1);
    });
</script>

<svelte:window
    on:resize={() => {
        portrait = window.innerHeight > window.innerWidth;
        uplot.setSize(getSize());
    }}
/>

<main
    class="flex gap-5 flex-wrap min-h-full flex-col flex-grow"
    class:md:flex-row={!portrait}
>
    <Card header="System Diagnostics" id="chart">
        <div bind:this={chart} />
    </Card>
    <Card header="System Stats">
        {#if ramData != undefined}
            {#if socketData.temp.available}
                <div class="text-center">
                    <span class={getTempClass(socketData.temp.celsius)}>
                        {socketData.temp.celsius}ºC/{socketData.temp
                            .fahrenheit}ºF</span
                    >: {getTempMsg(socketData.temp.celsius)}
                </div>
            {/if}
            CPU:<span class="float-right">{socketData.cpu}/100%</span>
            <div class="bg-gray-200 dark:bg-gray-800 w-full h-3 my-1">
                <div class="bg-green-500 h-3" style="width:{$cpuAnimate}%" />
            </div>
            RAM:<span class="float-right">{ramData[0]}/{ramData[1]}</span>
            <div class="bg-gray-200 dark:bg-gray-800 w-full h-3 my-1">
                <div class="bg-red-500 h-3" style="width:{$ramAnimate}%" />
            </div>
            Swap:<span class="float-right">{swapData[0]}/{swapData[1]}</span>
            <div class="bg-gray-200 dark:bg-gray-800 w-full h-3 my-1">
                <div class="bg-blue-500 h-3" style="width:{$swapAnimate}%" />
            </div>
            Disk:<span class="float-right">{diskData[0]}/{diskData[1]}</span>
            <div class="bg-gray-200 dark:bg-gray-800 w-full h-3 my-1">
                <div class="bg-yellow-500 h-3" style="width:{$diskAnimate}%" />
            </div>
        {/if}
    </Card>
</main>

<style>
    @import "uplot/dist/uPlot.min.css";
</style>
