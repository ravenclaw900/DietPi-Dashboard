<script lang="ts">
    import { softwareStore } from "../websocket";

    let installTemp: boolean[] = [];
    let installArray: number[] = [];
    let installTable = false;
    let needInstallTemp = true;
    let running = false;
    let nameList = "";

    // Runs once data is received or table is changed
    $: $softwareStore.uninstalled && installTempCreate();
    $: $softwareStore.uninstalled &&
        installTable !== undefined &&
        ((needInstallTemp = true), installTempCreate());

    // Runs every time installTemp array is changed
    $: $softwareStore.uninstalled && installTemp && (checkButton(), getNameList());

    const installTempCreate = () => {
        if (needInstallTemp) {
            installTemp = [];
            for (
                let i = 0;
                i < $softwareStore[installTable ? "installed" : "uninstalled"].length;
                i++
            ) {
                installTemp[
                    $softwareStore[installTable ? "installed" : "uninstalled"][i].id
                ] = false;
            }
        }
        needInstallTemp = false;
        running = false;
    };

    function checkButton() {
        installArray = [];
        for (const i of $softwareStore[installTable ? "installed" : "uninstalled"]) {
            if (installTemp[i.id] === true) {
                installArray = [...installArray, i.id];
            }
        }
    }

    function getNameList() {
        nameList = "";
        for (let i = 0; i < installArray.length; i++) {
            if (i === 0) {
                nameList += " (";
            } else {
                nameList += ", ";
            }
            nameList +=
                $softwareStore[installTable ? "installed" : "uninstalled"].find(
                    o => o.id === installArray[i]
                )?.name ?? "";
            if (i === installArray.length - 1) {
                nameList += ")";
            }
        }
    }

    function sendSoftware() {
        softwareStore.send({
            cmd: installTable ? "uninstall" : "install",
            args: installArray.map(val => {
                return val.toString();
            }),
        });
        running = true;
    }
</script>

<main>
    <div class="border-b-2 border-gray-500">
        <button
            class="border-1 border-b-0 border-gray-500 p-1 focus:outline-none"
            on:click={() => (installTable = false)}
            class:bg-gray-200={!installTable}
            class:dark:bg-gray-700={!installTable}>Not installed</button
        >
        <button
            class="border-1 border-b-0 border-gray-500 p-1 focus:outline-none"
            on:click={() => (installTable = true)}
            class:bg-gray-200={installTable}
            class:dark:bg-gray-700={installTable}>Installed</button
        >
    </div>
    <table
        class="border border-gray-300 dark:border-gray-700 w-full table-fixed break-words"
    >
        <tr class="table-header">
            <th>ID</th>
            <th>{installTable ? "Uninstall" : "Install"}</th>
            <th>Name</th>
            <th>Description</th>
            <th>Dependencies</th>
            <th>Documentation link</th>
        </tr>
        {#each $softwareStore[installTable ? "installed" : "uninstalled"] as software}
            {#if software.id !== -1}
                <tr
                    class="mt-32 even:bg-white odd:bg-gray-200 dark:even:bg-black dark:odd:bg-gray-800  dark:border-gray-600 border-t-2 border-gray-300 border-opacity-50"
                >
                    <td class="p-2">{software.id}</td>
                    <td class="p-2"
                        ><input
                            type="checkbox"
                            on:click={() =>
                                (installTemp[software.id] = !installTemp[software.id])}
                            bind:checked={installTemp[software.id]}
                            disabled={running}
                        /></td
                    >
                    <td class="p-2">{software.name}</td>
                    <td class="p-2">{software.description}</td>
                    <td class="p-2">{software.dependencies}</td>
                    <td class="p-2">
                        {#if software.docs.substring(0, 5) === "https"}
                            <a
                                href={software.docs}
                                class="text-blue-500 underline"
                                target="_blank"
                            >
                                {software.docs}
                            </a>
                        {:else}
                            {software.docs}
                        {/if}
                    </td>
                </tr>
            {/if}
        {/each}
    </table>
    <div class="flex justify-center my-2">
        <button
            on:click={sendSoftware}
            class="rounded border {installTable
                ? 'bg-red-500 border-red-600 hover:bg-red-700'
                : 'bg-green-500 border-green-600 hover:bg-green-700'} p-2 disabled:opacity-50"
            disabled={installArray.length === 0 || running}
        >
            {#if running}
                <div class="i-svg-spinners-270-ring inline-block" />
            {/if}
            {installTable ? "Uni" : "I"}nstall{nameList}
        </button>
    </div>
    {#if $softwareStore.response}
        <textarea
            class="w-full bg-gray-200 h-72 rounded dark:bg-gray-800"
            value={$softwareStore.response}
            disabled
        />
    {/if}
</main>
