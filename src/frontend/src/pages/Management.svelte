<script>
    import Card from "../components/Card.svelte";

    export let socket;
    export let socketData;

    let uptime;
    let uptimeSet = false;

    $: socketData.uptime != undefined &&
        !uptimeSet &&
        ((uptime = new Date(socketData.uptime * 1000)), (uptimeSet = true));

    function sendData(data) {
        socket.send(JSON.stringify({ do: data }));
        window.location.reload();
    }

    setInterval(() => {
        uptime.setSeconds(uptime.getSeconds() + 1);
        uptime = uptime;
    }, 1000);
</script>

<main class="flex gap-5 flex-wrap min-h-full flex-grow flex-col">
    {#if socketData.hostname != undefined}
        <Card header="System Information">
            <table class="border border-gray-100 h-full w-full">
                <tr class="even:bg-white odd:bg-gray-200">
                    <td class="p-1 font-semibold">Hostname:</td>
                    <td class="p-1">{socketData.hostname}</td>
                </tr>
                <tr class="even:bg-white odd:bg-gray-200">
                    <td class="p-1 font-semibold">Uptime</td>
                    <td class="p-1">{uptime.toISOString().substr(11, 8)}</td>
                </tr>
                <tr class="even:bg-white odd:bg-gray-200">
                    <td class="p-1 font-semibold">Base OS:</td>
                    <td class="p-1">{socketData.platform}</td>
                </tr>
                <tr class="even:bg-white odd:bg-gray-200">
                    <td class="p-1 font-semibold">Kernel:</td>
                    <td class="p-1">{socketData.kernel}</td>
                </tr>
                <tr class="even:bg-white odd:bg-gray-200">
                    <td class="p-1 font-semibold">Architecture:</td>
                    <td class="p-1">{socketData.arch}</td>
                </tr>
                <tr class="even:bg-white odd:bg-gray-200">
                    <td class="p-1 font-semibold">Network Interface:</td>
                    <td class="p-1">{socketData.interface}</td>
                </tr>
                <tr class="even:bg-white odd:bg-gray-200">
                    <td class="p-1 font-semibold">IP Address:</td>
                    <td class="p-1">{socketData.ip}</td>
                </tr>
            </table>
        </Card>
        <Card header="Management">
            <div class="flex gap-x-5">
                <button
                    on:click={() => sendData("poweroff")}
                    class="bg-red-500 border border-red-700 rounded-sm hover:bg-red-700 text-white text-md flex-grow p-2"
                    >Shutdown System</button
                >
                <button
                    on:click={() => sendData("reboot")}
                    class="bg-yellow-500 border border-yellow-600 rounded-sm hover:bg-yellow-600 text-white text-md flex-grow p-2"
                    >Restart System</button
                >
            </div>
        </Card>
    {:else}
        <h3>Getting data...</h3>
    {/if}
</main>
