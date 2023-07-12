<script lang="ts">
    import prettyBytes from "pretty-bytes";

    import Card from "../../shared-components/Card.svelte";
    import StatsRow from "./StatsRow.svelte";

    import { statisticsStore } from "../../websocket";

    export let tempUnit: "fahrenheit" | "celsius";

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

<Card header="System Stats">
    {#if $statisticsStore.temp.temp !== null}
        <div class="text-center">
            <span class={getTempClass($statisticsStore.temp.temp)}
                >{$statisticsStore.temp.temp}°{tempUnit == "celsius" ? "C" : "F"}</span
            >: {getTempMsg($statisticsStore.temp.temp)}
        </div>
    {/if}
    <StatsRow
        name="CPU"
        valuePretty={$statisticsStore.cpu.toString()}
        totalPretty="100%"
        totalValueRatio={$statisticsStore.cpu}
        colorClass="bg-green-500"
    />
    <StatsRow
        name="RAM"
        valuePretty={prettyBytes($statisticsStore.ram.used, { binary: true })}
        totalPretty={prettyBytes($statisticsStore.ram.total, { binary: true })}
        totalValueRatio={$statisticsStore.ram.percent}
        colorClass="bg-red-500"
    />
    <StatsRow
        name="Swap"
        valuePretty={prettyBytes($statisticsStore.swap.used, { binary: true })}
        totalPretty={prettyBytes($statisticsStore.swap.total, { binary: true })}
        totalValueRatio={$statisticsStore.swap.percent}
        colorClass="bg-blue-500"
    />
    <StatsRow
        name="Disk"
        valuePretty={prettyBytes($statisticsStore.disk.used)}
        totalPretty={prettyBytes($statisticsStore.disk.total)}
        totalValueRatio={$statisticsStore.disk.percent}
        colorClass="bg-yellow-500"
    />
</Card>
