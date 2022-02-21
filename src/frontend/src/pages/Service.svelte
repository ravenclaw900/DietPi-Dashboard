<script lang="ts">
    import {
        faSquare,
        faPlay,
        faRedoAlt,
    } from "@fortawesome/free-solid-svg-icons";
    import Fa from "svelte-fa";

    interface serviceData {
        services?: services[];
    }

    interface services {
        name: string;
        status: string;
        log: string;
        start: string;
    }

    export let socketSend: (cmd: string, args: string[]) => void;
    export let socketData: serviceData;
</script>

<main>
    {#if socketData.services}
        <table
            class="border border-gray-300 dark:border-gray-700 w-full table-fixed break-words"
        >
            <tr class="table-header">
                <th>Name</th>
                <th>Status</th>
                <th>Error Log</th>
                <th>Start Time</th>
                <th>Actions</th>
            </tr>
            {#each socketData.services as service}
                <tr
                    class="mt-32 even:bg-white odd:bg-gray-200 dark:even:bg-black dark:odd:bg-gray-800  dark:border-gray-600 border-t-2 border-gray-300 border-opacity-50"
                >
                    <td class="p-2">{service.name}</td>
                    <td class="p-2">{service.status}</td>
                    <td class="p-2">
                        {#if service.log != ""}
                            <details>
                                <summary> Show log </summary>
                                {@html service.log}
                            </details>
                        {/if}</td
                    >
                    <td class="p-2">{service.start}</td>
                    <td class="p-2 space-x-2">
                        {#if service.status == "dead" || service.status == "failed"}
                            <span
                                on:click={() =>
                                    socketSend("start", [service.name])}
                                title="Start"
                                ><Fa
                                    icon={faPlay}
                                    class="btn rounded-sm p-0.5"
                                    size="lg"
                                /></span
                            >
                        {:else}
                            <span
                                on:click={() =>
                                    socketSend("stop", [service.name])}
                                title="Stop"
                                ><Fa
                                    icon={faSquare}
                                    class="btn p-0.5 rounded-sm p-0.5"
                                    size="lg"
                                /></span
                            ><span
                                on:click={() =>
                                    socketSend("restart", [service.name])}
                                title="Restart"
                                ><Fa
                                    icon={faRedoAlt}
                                    class="btn p-0.5 rounded-sm p-0.5"
                                    size="lg"
                                /></span
                            >
                        {/if}</td
                    >
                </tr>
            {/each}
        </table>
    {/if}
</main>
