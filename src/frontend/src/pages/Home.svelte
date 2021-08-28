<script>
    import Card from "../components/Card.svelte"
    import Chart from "chart.js/auto"
    import { onMount } from "svelte"

    export let socketData
    let canvas

    function unitCalc(used, total) {
        let unitTotal, unitUsed, unit
        if (total > 1073742000) {
            unitTotal = Math.round(total / 1073742000 * 100) / 100
            unitUsed = Math.round(used / 1073742000 * 100) / 100
            unit = "GiB"
        } else if (total > 1048576) {
            unitTotal = Math.round(total / 1048576 * 100) / 100
            unitUsed = Math.round(used / 1048576 * 100) / 100
            unit = "MiB"
        } else if (total > 1024) {
            unitTotal = Math.round(total / 1024 * 100) / 100
            unitUsed = Math.round(used / 1024 * 100) / 100
            unit = "KiB"
        } else if (total < 1024) {
            unitTotal = total
            unitUsed = used
            unit = "B"
        }
        return [unitUsed, unitTotal, unit]
    }

    let ramData, swapData

    $: socketData.ram && (
        ramData = unitCalc(socketData.ram.used, socketData.ram.total),
        swapData = unitCalc(socketData.swap.used, socketData.swap.total)
    )

    const chartData = {
        labels: [],
        datasets: [
            {
                label: 'CPU',
                backgroundColor: '#10B981',
                borderColor: '#10B981',
                data: [],
                yAxisID: "cpuScale"
            },
            {
                label: 'RAM',
                backgroundColor: '#EF4444',
                borderColor: '#EF4444',
                data: [],
                yAxisID: "memScale"
            },
            {
                label: 'Swap',
                backgroundColor: '#3B82F6',
                borderColor: '#3B82F6',
                data: [],
                yAxisID: "memScale"
            }
    ]
    };

    const config = {
        type: 'line',
        data: chartData,
        options: {
            scales: {
                cpuScale: {
                    position: "right",
                    type: "linear",
                    ticks: {
                        callback: (value) => {
                            return value + "%"
                        }
                    }
                },
                memScale: {
                    position: "left",
                    type: "linear",
                    ticks: {
                        callback: (value) => {
                            return value + "MiB"
                        }
                    }
                }
            },
            responsive: true, 
            maintainAspectRatio: false
        }
    };

    onMount(() => {
        let chart = new Chart(
            canvas.getContext("2d"),
            config
        );

        setInterval(() => {
            let currenttime = new Date()
            chartData.labels.push(`${currenttime.getHours()}:${currenttime.getMinutes()}:${currenttime.getSeconds()}`);
            chartData.datasets[0].data.push(socketData.cpu)
            chartData.datasets[1].data.push(socketData.ram.used / 1048576)
            chartData.datasets[2].data.push(socketData.swap.used / 1048576)
            chart.update()
        }, 2000);
    })
</script>

<main class="flex gap-5 flex-wrap min-h-full flex-grow">
        <Card header="System Diagnostics">
            <div style="min-height: 90%; height:30vh; width:40vw">
                <canvas bind:this={canvas}/>
            </div>
        </Card>
        {#if ramData != undefined}
            <Card header="System Stats">
                CPU:<span class="float-right">{socketData.cpu}/100%</span>
                <div class="bg-gray-200 w-full h-3 my-1">
                    <div class="bg-green-500 h-3" style="width:{socketData.cpu}%"></div>
                </div>
                RAM:<span class="float-right">{ramData[0]}/{ramData[1]}{ramData[2]}</span>
                <div class="bg-gray-200 w-full h-3 my-1">
                    <div class="bg-red-500 h-3" style="width:{socketData.ram.percent}%"></div>
                </div>
                Swap:<span class="float-right">{swapData[0]}/{swapData[1]}{swapData[2]}</span>
                <div class="bg-gray-200 w-full h-3 my-1">
                    <div class="bg-blue-500 h-3" style="width:{socketData.swap.percent}%"></div>
                </div>
            </Card>
        {:else}
            <h3>Getting data...</h3>
        {/if}
</main>
