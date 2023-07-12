<script lang="ts">
    export let icon: string;
    export let navFunction: () => void;
</script>

<button on:click={navFunction} class="w-full">
    <div
        class="text-gray-400 no-underline hover:bg-gray-800 flex items-center h-10 text-xl font-sans whitespace-nowrap cursor-pointer"
    >
        <div class="mr-2 ml-1 {icon}" />
        <slot />
    </div>
</button>
