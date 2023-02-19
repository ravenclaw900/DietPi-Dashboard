<script lang="ts">
    import Card from "../../components/Card.svelte";
    import prettyBytes from "pretty-bytes";
    import uPlot from "uplot";
    import type { statisticsPage } from "../../types";

    export let darkMode: boolean;
    export let tempUnit: "fahrenheit" | "celsius";
    export let socketData: statisticsPage;

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

    let data: uPlot.AlignedData = [[], [], [], [], [], [], [], []];

    function initChart(el: HTMLDivElement) {
        let plot = new uPlot(opts, data, el);

        let handle1 = setInterval(() => {
            let dataPush = data as number[][];
            if (socketData.ram != undefined) {
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
    }
</script>

<Card header="System Diagnostics" id="chart">
    <div use:initChart />
</Card>

<style>
    @import "uplot/dist/uPlot.min.css";
</style>
