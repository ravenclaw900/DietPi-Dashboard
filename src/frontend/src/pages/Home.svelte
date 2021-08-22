<script>
    export let socket
    export let data

    console.log(socket)

    let json = {page: window.location.pathname}
    setInterval(() => {
        socket.send(JSON.stringify(json))
    }, 1000);

    function unitCalc(used, total) {
        let unitTotal, unitUsed, unit
        if (total > 1073742000) {
            unitTotal = Math.round(total / 1073742000 * 100) / 100
            unitUsed = Math.round(used / 1073742000 * 100) / 100
            unit = "GiB"
        } else if (total > 1048576) {
            unitTotal = Math.round(total / 1048576 * 100) / 100
            unitUsed = Math.round(used / 1048576 * 100) / 100
            unit = "MiB"
        } else if (total > 1024) {
            unitTotal = Math.round(total / 1024 * 100) / 100
            unitUsed = Math.round(used / 1024 * 100) / 100
            unit = "KiB"
        } else if (total < 1024) {
            unitTotal = total
            unitUsed = used
            unit = "B"
        }
        return [unitUsed, unitTotal, unit]
    }

    let ramData, swapData

    $: data.ram && (
        ramData = unitCalc(data.ram.used, data.ram.total),
        swapData = unitCalc(data.swap.used, data.swap.total)
    )
</script>

<main class="flex flex-wrap">
    {#if ramData != undefined}
        <div class="bg-white p-2 rounded border-t-4 border-gray-300 w-max font-sans shadow">
            <h2 class="border-b-2 border-gray-200 h-auto pb-2 mb-2">
                System Stats
            </h2>
            CPU:<span class="float-right">{data.cpu}/100%</span>
            <div class="bg-gray-200 w-80 h-3 my-1">
                <div class="bg-green-500 h-3" style="width:{data.cpu}%"></div>
            </div>
            RAM:<span class="float-right">{ramData[0]}/{ramData[1]}{ramData[2]}</span>
            <div class="bg-gray-200 w-80 h-3 my-1">
                <div class="bg-red-500 h-3" style="width:{data.ram.percent}%"></div>
            </div>
            Swap:<span class="float-right">{swapData[0]}/{swapData[1]}{swapData[2]}</span>
            <div class="bg-gray-200 w-80 h-3 my-1">
                <div class="bg-blue-500 h-3" style="width:{data.swap.percent}%"></div>
            </div>
        </div>
    {/if}
</main>
