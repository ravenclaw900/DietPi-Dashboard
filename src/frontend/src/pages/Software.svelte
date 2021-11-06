<script lang="ts">
    import Fa from "svelte-fa";
    import { faCircleNotch } from "@fortawesome/free-solid-svg-icons";

    export let socketData: softwareData;
    export let socket;

    interface softwareData {
        software?: software[];
        response?: string;
    }

    interface software {
        id: number;
        installed: boolean;
        name: string;
        description: string;
        dependencies: string;
        docs: string;
    }

    let installTemp = [];
    let installArray = [];
    let nameList = "";
    let uninstall;
    let needInstallTemp = true;
    let running = false;

    const installTempCreate = () => {
        if (needInstallTemp) {
            for (let i = 0; i < socketData.software.length; i++) {
                installTemp[socketData.software[i].id] =
                    socketData.software[i].installed;
            }
        }
        needInstallTemp = false;
        running = false;
    };

    function checkButton() {
        uninstall = undefined;
        installArray = [];
        for (const i of socketData.software) {
            if (i.installed != installTemp[i.id]) {
                if (
                    uninstall !== undefined &&
                    uninstall !== !installTemp[i.id]
                ) {
                    alert(
                        "ERROR: cannot install and uninstall at the same time"
                    );
                    installTemp[i.id] = !installTemp[i.id];
                    return;
                }
                uninstall = !installTemp[i.id];
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
            nameList += socketData.software[installArray[i]].name;
            if (i == installArray.length - 1) {
                nameList += ")";
            }
        }
    }

    function sendSoftware() {
        socket.send(
            JSON.stringify({
                cmd: uninstall == true ? "uninstall" : "install",
                args: installArray.map((val) => {
                    return val.toString();
                }),
            })
        );
        running = true;
    }

    $: socketData.software && installTempCreate();

    $: socketData.software && installTemp && (checkButton(), getNameList());
</script>

<main>
    {#if socketData.software != undefined}
        <table
            class="border border-gray-300 dark:border-gray-700 w-full table-fixed break-words"
        >
            <tr class="table-header">
                <th>ID</th>
                <th>Installed</th>
                <th>Name</th>
                <th>Description</th>
                <th>Dependencies</th>
                <th>Documentation link</th>
            </tr>
            {#each socketData.software as software}
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
                            {#if software.docs.substr(0, 5) == "https"}
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
                class="rounded border {uninstall == true
                    ? 'bg-red-500 border-red-600 hover:bg-red-700'
                    : 'bg-green-500 border-green-600 hover:bg-green-700'} p-2 disabled:opacity-50"
                disabled={installArray.length == 0 || running}
            >
                {#if running}
                    <Fa icon={faCircleNotch} class="animate-spin" />
                {/if}
                {uninstall == true ? "Uni" : "I"}nstall{nameList}
            </button>
        </div>
        {#if socketData.response != ""}
            <textarea
                class="w-full bg-gray-200 h-72 rounded"
                value={socketData.response}
                disabled
            />
        {/if}
    {:else}
        <h3>Getting data...</h3>
    {/if}
</main>
