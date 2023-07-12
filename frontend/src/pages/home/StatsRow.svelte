<script lang="ts">
    export let name: string;
    export let totalPretty: string;
    export let valuePretty: string;
    export let totalValueRatio: number;
    export let colorClass: string;
</script>

<div class="flex justify-between">
    <span>{name}:</span> <span>{valuePretty}/{totalPretty}</span>
</div>
<div class="bg-gray-200 dark:bg-gray-800 w-full h-3 my-1">
    <div class="{colorClass} h-3 transition-width-200" style="width:{totalValueRatio}%" />
</div>
