<script lang="ts">
    import Card from "../components/Card.svelte";
    import { tweened } from "svelte/motion";
    import prettyBytes from "pretty-bytes";
    import uPlot from "uplot";
    import "uplot/dist/uPlot.min.css";
    import { onMount, onDestroy } from "svelte";

    import type { statisticsPage } from "../types";

    export let socketData: statisticsPage;
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

    $: cpuAnimate.set(socketData.cpu);
    $: ramAnimate.set(socketData.ram.percent);
    $: swapAnimate.set(socketData.swap.percent);
    $: diskAnimate.set(socketData.disk.percent);

    $: ramData = [
        prettyBytes(socketData.ram.used, { binary: true }),
        prettyBytes(socketData.ram.total, { binary: true }),
    ];
    $: swapData = [
        prettyBytes(socketData.swap.used, { binary: true }),
        prettyBytes(socketData.swap.total, { binary: true }),
    ];
    $: diskData = [prettyBytes(socketData.disk.used), prettyBytes(socketData.disk.total)];

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
            return "really-hot";
        } else if (temp >= 60) {
            return "hot";
        } else if (temp >= 50) {
            return "warm";
        } else if (temp >= 40) {
            return "normal";
        } else if (temp >= 30) {
            return "cold";
        } else {
            return "really-cold";
        }
    }

    function resizeUplot(uplot: uPlot, entry: Element) {
        uplot.setSize({
            width: Math.min(
                entry.clientWidth - 10,
                (window.innerWidth / 100) * (portrait ? 70 : 50)
            ),
            height: Math.min(
                entry.clientHeight - 20,
                (window.innerHeight / 100) * (portrait ? 50 : 70)
            ),
        });
    }

    let uplot: uPlot | null;

    onMount(() => {
        let opts: uPlot.Options = {
            width: 100,
            height: 100,
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
                    value: (_: uPlot, val: number) => prettyBytes(val * 1000000),
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
                            (v: number) => +v + (tempUnit === "celsius" ? "ºC" : "ºF")
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
                    range: (u: uPlot) => (u.series[1].show ? [0, 100] : [null, null]),
                },
            },
        };

        uplot = new uPlot(opts, data, chart);

        if (socketData.swap.total === 0) {
            uplot.setSeries(3, { show: false });
        }

        let observer = new ResizeObserver((entries, _) =>
            resizeUplot(uplot as uPlot, entries[0].target)
        );

        // Guaranteed to exist
        observer.observe(document.getElementById("chart") as HTMLElement);
    });

    let handle1 = setInterval(() => {
        let dataPush = data as number[][];
        dataPush[0].push(Math.round(Date.now() / 1000));
        dataPush[1].push(socketData.cpu);
        dataPush[2].push(socketData.ram.used / 1000000);
        dataPush[3].push(socketData.swap.used / 1000000);
        dataPush[4].push(socketData.disk.used / 1000000);
        dataPush[5].push(socketData.network.sent / 1000000);
        dataPush[6].push(socketData.network.received / 1000000);
        if (uplot !== null) {
            if (socketData.temp.available) {
                if (uplot.series[7] === undefined) {
                    uplot.addSeries({
                        spanGaps: false,
                        label: "CPU Temperature",
                        stroke: "#94A3B8",
                        width: 3,
                        scale: "deg",
                        value: (_: any, val: number) =>
                            val + (tempUnit === "celsius" ? "ºC" : "ºF"),
                    });
                }
                if (tempUnit === "celsius") {
                    dataPush[7].push(socketData.temp.celsius);
                } else if (tempUnit === "fahrenheit") {
                    dataPush[7].push(socketData.temp.fahrenheit);
                }
            }
            uplot.setData(data);
        }
    }, 2000);

    onDestroy(() => {
        uplot = null;
        clearInterval(handle1);
    });
</script>

<svelte:window
    on:resize={() => {
        portrait = window.innerHeight > window.innerWidth;
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
        {#if socketData.temp.available}
            <div class="text-center">
                <span class={getTempClass(socketData.temp.celsius)}>
                    {socketData.temp.celsius}ºC/{socketData.temp.fahrenheit}ºF</span
                >: {getTempMsg(socketData.temp.celsius)}
            </div>
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
    /* This is slightly annoying, but global safelist doesn't seem to work, so we'll have to live with this */
    .really-hot {
        @apply text-red-500 font-semibold;
    }

    .hot {
        @apply text-red-500;
    }

    .warm {
        @apply text-yellow-500;
    }

    .normal {
        @apply text-green-500;
    }

    .cold {
        @apply text-blue-500;
    }

    .really-cold {
        @apply text-blue-500 font-semibold;
    }
</style>
