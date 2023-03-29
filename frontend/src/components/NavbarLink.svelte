<script lang="ts">
    //export let class1: string;
</script>

<div
    class="text-gray-400 no-underline hover:bg-gray-800 flex items-center h-10 text-xl font-sans whitespace-nowrap cursor-pointer"
>
    <div class="mr-2 ml-1 {$$props.class}" />
    <slot />
</div>
