<script lang="ts">
    import uPlot from "uplot";
    import prettyBytes from "pretty-bytes";

    import { statisticsStore } from "../websocket";

    export let darkMode: boolean;
    export let portrait: boolean;
    export let tempUnit: "fahrenheit" | "celsius";

    // When new data comes in, add it to the graph and set data to itself for reactivity
    $: $statisticsStore, updateData(), (data = data);

    // Values on a log graph can't equal zero, so set them to 1
    function zeroToOne(value: number) {
        return value == 0 ? 1 : value;
    }

    function updateData() {
        let pushData = data as number[][];
        pushData[0].push(Math.round(Date.now() / 1000));
        pushData[1].push($statisticsStore.cpu);
        pushData[2].push(zeroToOne($statisticsStore.ram.used));
        pushData[3].push(zeroToOne($statisticsStore.swap.used));
        pushData[4].push(zeroToOne($statisticsStore.network.sent));
        pushData[5].push(zeroToOne($statisticsStore.network.received));
        if ($statisticsStore.temp.temp !== null) {
            pushData[6].push($statisticsStore.temp.temp);
        } else {
            pushData[6].push(0);
        }
    }

    function resizeGraph(graph: uPlot, entry: Element) {
        graph.setSize({
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

    // Timestamp, CPU, RAM, Swap, Net (sent), Net (received), CPU temperature
    let data: uPlot.AlignedData = [[], [], [], [], [], [], []];

    let series = [
        {
            label: "CPU",
            stroke: "#10b981",
            width: 3,
            scale: "%",
            value: (_: uPlot, val: number | null) =>
                (val ?? $statisticsStore.cpu).toFixed(2) + "%",
        },
        {
            label: "RAM",
            stroke: "#ef4444",
            width: 3,
            scale: "bytes",
            value: (_: uPlot, val: number | null) =>
                prettyBytes(val ?? $statisticsStore.ram.used, { binary: true }),
        },
        {
            show: true,
            label: "Swap",
            stroke: "#3b82f6",
            width: 3,
            scale: "bytes",
            value: (_: uPlot, val: number | null) =>
                prettyBytes(val ?? $statisticsStore.swap.used, { binary: true }),
        },
        {
            label: "Network (sent)",
            stroke: "#a855f7",
            width: 3,
            scale: "bytes",
            value: (_: uPlot, val: number | null) =>
                prettyBytes(val ?? $statisticsStore.network.sent),
        },
        {
            label: "Network (received)",
            stroke: "#ec4899",
            width: 3,
            scale: "bytes",
            value: (_: uPlot, val: number | null) =>
                prettyBytes(val ?? $statisticsStore.network.received),
        },
    ];

    const graphOpts: uPlot.Options = {
        width: 100,
        height: 100,
        series: [{}, ...series],
        cursor: {
            drag: {
                setScale: false,
            },
        },
        // Following the official examples, there don't need to be any numbers in the rest of the object
        // @ts-ignore
        select: {
            show: false,
        },
        axes: [
            {
                grid: { show: false },
                stroke: () => (darkMode ? "#fff" : "#000"),
            },
            {
                scale: "bytes",
                values: (_: uPlot, vals: (number | null)[]) =>
                    vals.map((v: number | null) => (v === null ? null : prettyBytes(v))),
                size: 75,
                grid: { stroke: () => (darkMode ? "#4b5563" : "#ededed") },
                stroke: () => (darkMode ? "#fff" : "#000"),
            },
            {
                side: 1,
                scale: "%",
                values: (_: uPlot, vals: number[]) =>
                    vals.map((v: number) => v.toFixed(2) + "%"),
                grid: { show: false },
                stroke: "#10b981",
                size: 75,
            },
            {
                side: 1,
                scale: "deg",
                values: (_: uPlot, vals: number[]) =>
                    vals.map((v: number) => v + (tempUnit === "celsius" ? "ºC" : "ºF")),
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
            bytes: {
                // Use log scale so all values are visible
                distr: 3,
            },
        },
    };

    function createGraph(
        node: HTMLDivElement,
        // darkMode is a paramater solely for the graph to be redrawn when dark mode is toggled
        params: { data: uPlot.AlignedData; darkMode: boolean }
    ) {
        let graph = new uPlot(graphOpts, params.data, node);

        // Svelte's on:resize event doesn't seem to work here, so we use this instead
        let observer = new ResizeObserver(() => {
            resizeGraph(graph, node);
        });

        observer.observe(node);

        return {
            update(newParams: { data: uPlot.AlignedData }) {
                // Can't set these in creation method because websocket data might not have been sent yet
                if (
                    graph.series[6] === undefined &&
                    $statisticsStore.temp.temp !== null
                ) {
                    graph.addSeries({
                        label: "CPU Temperature",
                        stroke: "#94A3B8",
                        width: 3,
                        scale: "deg",
                        value: (_: any, val: number | null) =>
                            (val ?? $statisticsStore.temp.temp) +
                            (tempUnit === "celsius" ? "ºC" : "ºF"),
                    });
                }
                if ($statisticsStore.swap.total === 0) {
                    graph.setSeries(3, { show: false });
                }

                // Slight caveat of having data reset every time dark mode is toggled, but probably minimal performance impact
                graph.setData(newParams.data);
            },
            destroy() {
                graph.destroy();
                observer.disconnect();
            },
        };
    }
</script>

<div use:createGraph={{ data, darkMode }} />
