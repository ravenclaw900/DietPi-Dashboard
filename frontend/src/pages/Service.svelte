<script lang="ts">
    import { serviceStore } from "../websocket";
</script>

<main>
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
        {#each $serviceStore.services as service}
            <tr
                class="mt-32 even:bg-white odd:bg-gray-200 dark:even:bg-black dark:odd:bg-gray-800  dark:border-gray-600 border-t-2 border-gray-300 border-opacity-50"
            >
                <td class="p-2">{service.name}</td>
                <td class="p-2">{service.status}</td>
                <td class="p-2">
                    {#if service.log !== ""}
                        <details>
                            <summary> Show log </summary>
                            {@html service.log}
                        </details>
                    {/if}</td
                >
                <td class="p-2">{service.start}</td>
                <td class="p-2 space-x-2">
                    {#if service.status === "inactive" || service.status === "failed"}
                        <button
                            on:click={() =>
                                serviceStore.send({ cmd: "start", args: [service.name] })}
                            title="Start"
                            class="btn rounded-sm p-0.5 i-fa-play text-2xl"
                        />
                    {:else}
                        <button
                            on:click={() =>
                                serviceStore.send({ cmd: "stop", args: [service.name] })}
                            title="Stop"
                            class="btn rounded-sm p-0.5 i-fa-square text-2xl"
                        /><button
                            on:click={() =>
                                serviceStore.send({
                                    cmd: "restart",
                                    args: [service.name],
                                })}
                            title="Restart"
                            class="btn rounded-sm p-0.5 i-fa-rotate-left text-2xl"
                        />
                    {/if}</td
                >
            </tr>
        {/each}
    </table>
</main>
