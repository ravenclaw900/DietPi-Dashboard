<script lang="ts">
    import Card from "../components/Card.svelte";
    import prettyMilliseconds from "pretty-ms";
    import { fade } from "svelte/transition";

    interface hostData {
        hostname?: String;
        uptime?: number;
        arch?: string;
        kernel?: string;
        dp_version?: string;
        packages?: number;
        upgrades?: number;
        nic?: string;
        ip?: string;
    }

    export let socketSend: (cmd: string, args: string[]) => void;
    export let socketData: hostData;

    let uptime: string;
    let dialog = false;
    let msg = "";

    $: socketData.uptime &&
        ((uptime = prettyMilliseconds(socketData.uptime * 60000, {
            verbose: true,
        })),
        (dialog = false));

    function sendData(data: string) {
        socketSend(data, []);
        // Give backend an extra second to loop again
        setTimeout(() => {
            dialog = true;
        }, 1000);
        if (data == "reboot") {
            msg = "Waiting for device to finish...";
        } else if (data == "poweroff") {
            msg = "You can close this page";
        }
    }
</script>

<main class="flex gap-5 flex-wrap min-h-full flex-grow flex-col">
    {#if dialog}
        <div
            class="fixed inset-0 bg-gray-600 bg-opacity-50 h-screen w-screen flex items-center justify-center"
            transition:fade
        >
            <div
                class="bg-white dark:bg-black w-1/2 h-1/3 rounded-md flex items-center justify-center text-xl"
            >
                {msg}
            </div>
        </div>
    {/if}
    <Card header="System Information">
        <table
            class="border border-gray-100 dark:border-gray-900 h-full w-full"
        >
            <tr
                class="even:bg-white odd:bg-gray-200 dark:even:bg-black dark:odd:bg-gray-800"
            >
                <td class="p-1 font-semibold">Hostname:</td>
                <td class="p-1">{socketData.hostname}</td>
            </tr>
            <tr
                class="even:bg-white odd:bg-gray-200 dark:even:bg-black dark:odd:bg-gray-800"
            >
                <td class="p-1 font-semibold">Network Interface:</td>
                <td class="p-1">{socketData.nic}</td>
            </tr>
            <tr
                class="even:bg-white odd:bg-gray-200 dark:even:bg-black dark:odd:bg-gray-800"
            >
                <td class="p-1 font-semibold">IP Address:</td>
                <td class="p-1">{socketData.ip}</td>
            </tr>
            <tr
                class="even:bg-white odd:bg-gray-200 dark:even:bg-black dark:odd:bg-gray-800"
            >
                <td class="p-1 font-semibold">Uptime:</td>
                <td class="p-1">{uptime}</td>
            </tr>
            <tr
                class="even:bg-white odd:bg-gray-200 dark:even:bg-black dark:odd:bg-gray-800"
            >
                <td class="p-1 font-semibold">Kernel:</td>
                <td class="p-1">{socketData.kernel}</td>
            </tr>
            <tr
                class="even:bg-white odd:bg-gray-200 dark:even:bg-black dark:odd:bg-gray-800"
            >
                <td class="p-1 font-semibold">Architecture:</td>
                <td class="p-1">{socketData.arch}</td>
            </tr>
            <tr
                class="even:bg-white odd:bg-gray-200 dark:even:bg-black dark:odd:bg-gray-800"
            >
                <td class="p-1 font-semibold">Version:</td>
                <td class="p-1">{socketData.dp_version}</td>
            </tr>
            <tr
                class="even:bg-white odd:bg-gray-200 dark:even:bg-black dark:odd:bg-gray-800"
            >
                <td class="p-1 font-semibold">Installed Packages:</td>
                <td class="p-1"
                    >{socketData.packages}
                    {#if socketData.upgrades !== 0}({socketData.upgrades} upgradable){/if}
                </td>
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
</main>
