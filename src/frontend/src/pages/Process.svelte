<script>
    export let socketData

    $: console.log(socketData)
</script>

<main>
    {#if socketData.processes != undefined}
        <table class="border bg-gray-200 border-gray-300 w-full">
            <tr>
                <th>PID</th>
                <th>Name</th>
                <th>CPU Usage</th>
                <th>RAM Usage</th>
            </tr>
            {#each socketData.processes as process}
                <tr class="mt-32 border-t-2 border-gray-300 border-opacity-50">
                    <td class="p-2">{process.pid}</td>
                    <td class="p-2">{process.name}</td>
                    <td class="p-2">{process.cpu}</td>
                    <td class="p-2">{process.ram}</td>
                </tr>
            {/each}
        </table>
    {:else}
        <h3>Getting data...</h3>
    {/if}
</main>
