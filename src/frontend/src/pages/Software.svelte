<script lang="ts">
    import Fa from "svelte-fa";
    import { faCircleNotch } from "@fortawesome/free-solid-svg-icons";

    export let socketData: softwareData;
    export let socketSend: (cmd: string, args: string[]) => void;

    interface softwareData {
        uninstalled?: software[];
        installed?: software[];
        response?: string;
    }

    interface software {
        id: number;
        name: string;
        description: string;
        dependencies: string;
        docs: string;
    }

    let installTemp: boolean[] = [];
    let installArray: number[] = [];
    let nameList = "";
    let installTable = false;
    let needInstallTemp = true;
    let running = false;

    const installTempCreate = () => {
        if (needInstallTemp) {
            installTemp = [];
            for (
                let i = 0;
                i <
                socketData[installTable ? "installed" : "uninstalled"].length;
                i++
            ) {
                installTemp[
                    socketData[installTable ? "installed" : "uninstalled"][i].id
                ] = false;
            }
        }
        needInstallTemp = false;
        running = false;
    };

    function checkButton() {
        installArray = [];
        for (const i of socketData[
            installTable ? "installed" : "uninstalled"
        ]) {
            if (installTemp[i.id] == true) {
                installArray = [...installArray, i.id];
            }
        }
    }

    function getNameList() {
        nameList = "";
        for (let i = 0; i < installArray.length; i++) {
            if (i == 0) {
                nameList += " (";
            } else {
                nameList += ", ";
            }
            nameList += socketData[
                installTable ? "installed" : "uninstalled"
            ].find((o) => o.id == installArray[i]).name;
            if (i == installArray.length - 1) {
                nameList += ")";
            }
        }
    }

    function sendSoftware() {
        socketSend(
            installTable ? "uninstall" : "install",
            installArray.map((val) => {
                return val.toString();
            })
        );
        running = true;
    }

    // Runs once data is received or table is changed
    $: socketData.uninstalled && installTempCreate();
    $: (installTable == true || installTable == false) &&
        ((needInstallTemp = true), installTempCreate());

    // Runs every time installTemp array is changed
    $: socketData.uninstalled && installTemp && (checkButton(), getNameList());
</script>

<main>
    {#if socketData.uninstalled}
        <div class="border-b-2 border-gray-500">
            <button
                class="border-1 border-b-0 border-gray-500 p-1 focus:outline-none{installTable
                    ? ''
                    : ' bg-gray-200 dark:bg-gray-700'}"
                on:click={() => (installTable = false)}>Not installed</button
            >
            <button
                class="border-1 border-b-0 border-gray-500 p-1 focus:outline-none{installTable
                    ? ' bg-gray-200 dark:bg-gray-700'
                    : ''}"
                on:click={() => (installTable = true)}>Installed</button
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
            {#each socketData[installTable ? "installed" : "uninstalled"] as software}
                {#if software.id != -1}
                    <tr
                        class="mt-32 even:bg-white odd:bg-gray-200 dark:even:bg-black dark:odd:bg-gray-800  dark:border-gray-600 border-t-2 border-gray-300 border-opacity-50"
                    >
                        <td class="p-2">{software.id}</td>
                        <td class="p-2"
                            ><input
                                type="checkbox"
                                on:click={() =>
                                    (installTemp[software.id] =
                                        !installTemp[software.id])}
                                bind:checked={installTemp[software.id]}
                                disabled={running}
                            /></td
                        >
                        <td class="p-2">{software.name}</td>
                        <td class="p-2">{software.description}</td>
                        <td class="p-2">{software.dependencies}</td>
                        <td class="p-2">
                            {#if software.docs.substring(0, 5) == "https"}
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
    {/if}
    <div class="flex justify-center my-2">
        <button
            on:click={sendSoftware}
            class="rounded border {installTable
                ? 'bg-red-500 border-red-600 hover:bg-red-700'
                : 'bg-green-500 border-green-600 hover:bg-green-700'} p-2 disabled:opacity-50"
            disabled={installArray.length == 0 || running}
        >
            {#if running}
                <Fa icon={faCircleNotch} class="animate-spin" />
            {/if}
            {installTable ? "Uni" : "I"}nstall{nameList}
        </button>
    </div>
    {#if socketData.response != ""}
        <textarea
            class="w-full bg-gray-200 h-72 rounded dark:bg-gray-800"
            value={socketData.response}
            disabled
        />
    {/if}
</main>
