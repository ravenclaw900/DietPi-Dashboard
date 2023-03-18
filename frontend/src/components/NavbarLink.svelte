<script lang="ts">
    import Fa from "svelte-fa";
    import type { IconDefinition } from "@fortawesome/free-solid-svg-icons";
    export let icon: IconDefinition;
</script>

<div
    class="text-gray-400 no-underline hover:bg-gray-800 flex items-center h-10 text-xl font-sans whitespace-nowrap cursor-pointer"
>
    <Fa {icon} class="mr-2 ml-1" size="sm" /><slot />
</div>
