<script>
    import {socket, data} from "../store/store.js"

    let json = {page: window.location.pathname}
    setInterval(() => {
        $socket.send(JSON.stringify(json))
    }, 1000);
</script>

<main>
    {#if $data != undefined}
        <div class="bg-white p-2 rounded border-t-4 border-gray-300 w-max font-sans shadow">
            <h2 class="border-b-2 border-gray-200 h-auto pb-2 mb-2">
                System Stats
            </h2>
            CPU:<span class="float-right">{$data.cpu}/100%</span>
            <div class="bg-gray-200 w-80 h-3 my-1">
                <div class="bg-green-500 h-3" style="width:{$data.cpu}%"></div>
            </div>
        </div>
    {:else}
        <h3>Opening connection to API...</h3>
    {/if}
</main>