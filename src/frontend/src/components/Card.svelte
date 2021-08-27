<script>
    export let header;
</script>

<div class="bg-white p-2 rounded border-t-4 border-gray-300 flex-1 font-sans shadow">
    <h2 class="border-b-2 border-gray-200 h-auto pb-2 mb-2">
        {header}
    </h2>
    <slot />
</div>