<script lang="ts">
    import Card from "../components/Card.svelte";
    import Chart from "chart.js/auto";
    import type { ChartConfiguration } from "chart.js";
    import { onMount } from "svelte";
    import { tweened } from "svelte/motion";

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

    export let socketData;
    export let darkMode;
    let canvas;

    function unitCalc(used, total) {
        let unitTotal, unitUsed, unit;
        if (total > 1099512000000) {
            unitTotal = Math.round((total / 1099512000000) * 100) / 100;
            unitUsed = Math.round((used / 1099512000000) * 100) / 100;
            unit = "TiB";
        } else if (total > 1073742000) {
            unitTotal = Math.round((total / 1073742000) * 100) / 100;
            unitUsed = Math.round((used / 1073742000) * 100) / 100;
            unit = "GiB";
        } else if (total > 1048576) {
            unitTotal = Math.round((total / 1048576) * 100) / 100;
            unitUsed = Math.round((used / 1048576) * 100) / 100;
            unit = "MiB";
        } else if (total > 1024) {
            unitTotal = Math.round((total / 1024) * 100) / 100;
            unitUsed = Math.round((used / 1024) * 100) / 100;
            unit = "KiB";
        } else if (total < 1024) {
            unitTotal = total;
            unitUsed = used;
            unit = "B";
        }
        return [unitUsed, unitTotal, unit];
    }

    const chartData = {
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
                label: "Network (recieved)",
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

    let ramData, swapData, diskData;

    $: socketData.cpu != undefined &&
        (cpuAnimate.set(socketData.cpu),
        ramAnimate.set(socketData.ram.percent),
        swapAnimate.set(socketData.swap.percent),
        diskAnimate.set(socketData.disk.percent));

    $: socketData.ram &&
        ((ramData = unitCalc(socketData.ram.used, socketData.ram.total)),
        (swapData = unitCalc(socketData.swap.used, socketData.swap.total)),
        (chartData.datasets[2].hidden = socketData.swap.total == 0),
        (diskData = unitCalc(socketData.disk.used, socketData.disk.total)));

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
            let currenttime = new Date();
            chartData.labels.push(
                `${currenttime.getHours()}:${currenttime.getMinutes()}:${currenttime.getSeconds()}`
            );
            chartData.datasets[0].data.push(socketData.cpu);
            chartData.datasets[1].data.push(socketData.ram.used / 1048576);
            chartData.datasets[2].data.push(socketData.swap.used / 1048576);
            chartData.datasets[3].data.push(socketData.disk.used / 1048576);
            chartData.datasets[4].data.push(socketData.network.sent / 1048576);
            chartData.datasets[5].data.push(
                socketData.network.recieved / 1048576
            );
            chart.update();
        }, 2000);
    });
</script>

<main class="flex gap-5 flex-wrap min-h-full flex-col md:flex-row flex-grow">
    <Card header="System Diagnostics">
        <div id="chartWrapper">
            <canvas bind:this={canvas} />
        </div>
    </Card>
    {#if ramData != undefined}
        <Card header="System Stats">
            CPU:<span class="float-right">{socketData.cpu}/100%</span>
            <div class="bg-gray-200 dark:bg-gray-800 w-full h-3 my-1">
                <div class="bg-green-500 h-3" style="width:{$cpuAnimate}%" />
            </div>
            RAM:<span class="float-right"
                >{ramData[0]}/{ramData[1]}{ramData[2]}</span
            >
            <div class="bg-gray-200 dark:bg-gray-800 w-full h-3 my-1">
                <div class="bg-red-500 h-3" style="width:{$ramAnimate}%" />
            </div>
            Swap:<span class="float-right"
                >{swapData[0]}/{swapData[1]}{swapData[2]}</span
            >
            <div class="bg-gray-200 dark:bg-gray-800 w-full h-3 my-1">
                <div class="bg-blue-500 h-3" style="width:{$swapAnimate}%" />
            </div>
            Disk:<span class="float-right"
                >{diskData[0]}/{diskData[1]}{diskData[2]}</span
            >
            <div class="bg-gray-200 dark:bg-gray-800 w-full h-3 my-1">
                <div class="bg-yellow-500 h-3" style="width:{$diskAnimate}%" />
            </div>
        </Card>
    {:else}
        <h3>Getting data...</h3>
    {/if}
</main>

<style>
    #chartWrapper {
        max-width: 100%;
        min-width: 0;
        width: 85vw;
        max-height: 95%;
        min-height: 0;
        height: 70vh;
    }
</style>
