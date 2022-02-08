<script lang="ts">
    import Card from "../components/Card.svelte";
    import Chart from "chart.js/auto";
    import type { ChartConfiguration, ChartData } from "chart.js";
    import { onMount } from "svelte";
    import { tweened } from "svelte/motion";
    import prettyBytes from "pretty-bytes";

    interface statData {
        cpu?: number;
        ram?: usage;
        swap?: usage;
        disk?: usage;
        network?: net;
    }

    interface usage {
        used: number;
        total: number;
        percent: number;
    }

    interface net {
        sent: number;
        received: number;
    }

    export let socketData: statData;
    export let darkMode: boolean;
    let canvas: HTMLCanvasElement;

    let portrait: boolean;
    $: portrait = window.innerHeight > window.innerWidth;

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

    const chartData: ChartData = {
        labels: [],
        datasets: [
            {
                label: "CPU",
                backgroundColor: "#10B981",
                borderColor: "#10B981",
                data: [],
                yAxisID: "cpuScale",
                hidden: false,
            },
            {
                label: "RAM",
                backgroundColor: "#EF4444",
                borderColor: "#EF4444",
                data: [],
                yAxisID: "usageScale",
                hidden: false,
            },
            {
                label: "Swap",
                backgroundColor: "#3B82F6",
                borderColor: "#3B82F6",
                data: [],
                yAxisID: "usageScale",
                hidden: false,
            },
            {
                label: "Disk",
                backgroundColor: "#F59E0B",
                borderColor: "#F59E0B",
                data: [],
                yAxisID: "usageScale",
                hidden: false,
            },
            {
                label: "Network (sent)",
                backgroundColor: "#8B5CF6",
                borderColor: "#8B5CF6",
                data: [],
                yAxisID: "usageScale",
                hidden: false,
            },
            {
                label: "Network (received)",
                backgroundColor: "#EC4899",
                borderColor: "#EC4899",
                data: [],
                yAxisID: "usageScale",
                hidden: false,
            },
        ],
    };

    const config: ChartConfiguration = {
        type: "line",
        data: chartData,
        options: {
            scales: {
                cpuScale: {
                    position: "right",
                    type: "linear",
                    ticks: {
                        callback: (value) => {
                            return value + "%";
                        },
                    },
                    grid: {
                        color: darkMode ? "#4B5563" : "#D1D5DB",
                    },
                },
                usageScale: {
                    position: "left",
                    type: "linear",
                    ticks: {
                        callback: (value) => {
                            return value + "MiB";
                        },
                    },
                    grid: {
                        color: darkMode ? "#4B5563" : "#D1D5DB",
                    },
                },
                x: {
                    grid: {
                        color: darkMode ? "#4B5563" : "#D1D5DB",
                    },
                },
            },
            responsive: true,
            maintainAspectRatio: false,
        },
    };

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
        (chartData.datasets[2].hidden = socketData.swap.total == 0),
        (diskData = [
            prettyBytes(socketData.disk.used),
            prettyBytes(socketData.disk.total),
        ]));

    onMount(() => {
        let chart = new Chart(canvas.getContext("2d"), config);

        setInterval(() => {
            chart.options.scales.cpuScale.grid.color = darkMode
                ? "#4B5563"
                : "#D1D5DB";
            chart.options.scales.usageScale.grid.color = darkMode
                ? "#4B5563"
                : "#D1D5DB";
            chart.options.scales.x.grid.color = darkMode
                ? "#4B5563"
                : "#D1D5DB";
            if (socketData.ram.used != undefined) {
                let currenttime = new Date();
                chartData.labels.push(
                    `${currenttime.getHours()}:${currenttime.getMinutes()}:${currenttime.getSeconds()}`
                );
                chartData.datasets[0].data.push(socketData.cpu);
                chartData.datasets[1].data.push(socketData.ram.used / 1048576);
                chartData.datasets[2].data.push(socketData.swap.used / 1048576);
                chartData.datasets[3].data.push(socketData.disk.used / 1048576);
                chartData.datasets[4].data.push(
                    socketData.network.sent / 1048576
                );
                chartData.datasets[5].data.push(
                    socketData.network.received / 1048576
                );
            }
            chart.update();
        }, 2000);
    });
</script>

<main
    class="flex gap-5 flex-wrap min-h-full flex-col flex-grow"
    class:md:flex-row={portrait}
>
    <Card header="System Diagnostics">
        <div
            id="chartWrapper"
            class={portrait
                ? "w-70vw min-w-full h-50vh"
                : "max-w-full h-70vh max-h-95%"}
        >
            <canvas bind:this={canvas} />
        </div>
    </Card>
    {#if ramData != undefined}
        <Card header="System Stats">
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
        </Card>
    {/if}
</main>
