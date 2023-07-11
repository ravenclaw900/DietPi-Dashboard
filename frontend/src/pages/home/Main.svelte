<script lang="ts">
    import GraphCard from "./GraphCard.svelte";
    import StatsCard from "./StatsCard.svelte";

    export let darkMode: boolean;
    export let tempUnit: "fahrenheit" | "celsius";

    let portrait = window.innerHeight > window.innerWidth;
</script>

<svelte:window on:resize={() => (portrait = window.innerHeight > window.innerWidth)} />

<main class="flex gap-5 min-h-full flex-col" class:md:flex-row={!portrait}>
    <GraphCard {darkMode} {tempUnit} {portrait} />
    <StatsCard {tempUnit} />
</main>
