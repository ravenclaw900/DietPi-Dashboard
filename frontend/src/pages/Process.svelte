<script lang="ts">
    import prettyBytes from "pretty-bytes";

    import { processStore } from "../websocket";

    import type { processItem } from "../types";

    $: processes = $processStore.processes;
    // Sorts when $processStore, sortBy, or reverse updates
    $: $processStore.processes, sortBy, reverse, sortTable(sortBy);

    let sortBy: keyof processItem = "pid";
    let reverse = false;

    $: console.log(reverse);

    function sortTable(sortValue: keyof processItem) {
        processes.sort((a, b) => {
            if (a[sortValue] > b[sortValue]) {
                return reverse ? -1 : 1;
            } else if (a[sortValue] < b[sortValue]) {
                return reverse ? 1 : -1;
            }
            return 0;
        });
        processes = processes;
    }

    function resortTable(sortValue: keyof processItem) {
        if (sortValue != sortBy) {
            reverse = false;
        } else {
            reverse = !reverse;
        }
        sortBy = sortValue;
    }
</script>

<main>
    <table
        class="border border-gray-300 dark:border-gray-700 w-full table-fixed break-words min-w-50"
    >
        <tr class="table-header">
            <th
                class="cursor-pointer"
                on:click={() => resortTable("pid")}
                on:keypress={() => resortTable("pid")}
                >PID
                {#if sortBy == "pid"}
                    <div
                        class="inline-block {reverse
                            ? 'i-fa6-solid-sort-down'
                            : 'i-fa6-solid-sort-up'}"
                    />
                {/if}
            </th>
            <th
                class="cursor-pointer"
                on:click={() => resortTable("name")}
                on:keypress={() => resortTable("name")}
                >Name
                {#if sortBy == "name"}
                    <div
                        class="inline-block {reverse
                            ? 'i-fa6-solid-sort-down'
                            : 'i-fa6-solid-sort-up'}"
                    />
                {/if}
            </th>
            <th
                class="cursor-pointer"
                on:click={() => resortTable("status")}
                on:keypress={() => resortTable("status")}
                >Status
                {#if sortBy == "status"}
                    <div
                        class="inline-block {reverse
                            ? 'i-fa6-solid-sort-down'
                            : 'i-fa6-solid-sort-up'}"
                    />
                {/if}
            </th>
            <th
                class="cursor-pointer"
                on:click={() => resortTable("cpu")}
                on:keypress={() => resortTable("cpu")}
                >CPU Usage
                {#if sortBy == "cpu"}
                    <div
                        class="inline-block {reverse
                            ? 'i-fa6-solid-sort-down'
                            : 'i-fa6-solid-sort-up'}"
                    />
                {/if}
            </th>
            <th
                class="cursor-pointer"
                on:click={() => resortTable("ram")}
                on:keypress={() => resortTable("ram")}
                >RAM Usage
                {#if sortBy == "ram"}
                    <div
                        class="inline-block {reverse
                            ? 'i-fa6-solid-sort-down'
                            : 'i-fa6-solid-sort-up'}"
                    />
                {/if}
            </th>
            <th>Actions</th>
        </tr>
        {#each processes as process}
            <tr
                class="mt-32 even:bg-white odd:bg-gray-200 dark:even:bg-black dark:odd:bg-gray-800  dark:border-gray-600 border-t-2 border-gray-300 border-opacity-50"
            >
                <td class="p-2">{process.pid}</td>
                <td class="p-2">{process.name}</td>
                <td class="p-2">{process.status}</td>
                <td class="p-2">{process.cpu}%</td>
                <td class="p-2"
                    >{prettyBytes(process.ram, {
                        binary: true,
                        maximumFractionDigits: 0,
                    })}</td
                >
                <td class="p-2 space-x-2">
                    {#if process.name != "dietpi-dashboar"}
                        <button
                            class="rounded-sm p-0.5 btn i-fa6-solid-ban text-2xl"
                            on:click={() =>
                                processStore.send({
                                    cmd: "terminate",
                                    args: [process.pid.toString()],
                                })}
                            title="Terminate"
                        />
                        <button
                            class="rounded-sm p-0.5 btn i-fa6-solid-skull text-2xl"
                            on:click={() =>
                                processStore.send({
                                    cmd: "kill",
                                    args: [process.pid.toString()],
                                })}
                            title="Kill"
                        />
                        {#if process.status != "stopped"}
                            <button
                                class="rounded-sm p-0.5 btn i-fa6-solid-pause text-2xl"
                                on:click={() =>
                                    processStore.send({
                                        cmd: "suspend",
                                        args: [process.pid.toString()],
                                    })}
                                title="Suspend"
                            />
                        {:else}
                            <button
                                class="rounded-sm p-0.5 btn i-fa6-solid-play text-2xl"
                                on:click={() =>
                                    processStore.send({
                                        cmd: "resume",
                                        args: [process.pid.toString()],
                                    })}
                                title="Resume"
                            />
                        {/if}
                    {/if}
                </td>
            </tr>
        {/each}
    </table>
</main>
