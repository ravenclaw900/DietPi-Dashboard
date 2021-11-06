<script lang="ts">
    import Fa from "svelte-fa";
    import {
        faSort,
        faSortUp,
        faSortDown,
        faBan,
        faSkull,
        faPause,
        faPlay,
    } from "@fortawesome/free-solid-svg-icons";

    export let socketData: processData;
    export let socket;

    interface processData {
        processes?: processes[];
    }

    interface processes {
        pid: number;
        name: string;
        cpu: number;
        ram: number;
        status: string;
    }

    let pidSort = true;
    let pidIcon = faSortUp;
    let nameSort = false;
    let nameIcon = faSort;
    let cpuSort = false;
    let cpuIcon = faSort;
    let ramSort = false;
    let ramIcon = faSort;
    let statusSort = false;
    let statusIcon = faSort;
    let reverse = false;

    $: cpuSort && socketData.processes && sortCPU(reverse);
    $: pidSort && socketData.processes && sortPid(reverse);
    $: nameSort && socketData.processes && sortName(reverse);
    $: ramSort && socketData.processes && sortRAM(reverse);
    $: statusSort && socketData.processes && sortStatus(reverse);

    function sortCPU(reverse) {
        socketData.processes.sort((a, b) => {
            if (a.cpu < b.cpu) {
                return reverse ? -1 : 1;
            } else if (a.cpu > b.cpu) {
                return reverse ? 1 : -1;
            } else {
                return 0;
            }
        });
        socketData = socketData;
    }

    function setCPU() {
        if (cpuSort == true) {
            reverse = !reverse;
            if (cpuIcon == faSortUp) {
                cpuIcon = faSortDown;
            } else {
                cpuIcon = faSortUp;
            }
        } else {
            cpuSort = true;
            pidSort = false;
            nameSort = false;
            ramSort = false;
            reverse = false;
            cpuIcon = faSortUp;
            ramIcon = faSort;
            pidIcon = faSort;
            nameIcon = faSort;
            statusSort = false;
            statusIcon = faSort;
        }
    }

    function sortName(reverse) {
        socketData.processes.sort((a, b) => {
            if (a.name < b.name) {
                return reverse ? 1 : -1;
            } else if (a.name > b.name) {
                return reverse ? -1 : 1;
            } else {
                return 0;
            }
        });
        socketData = socketData;
    }

    function setName() {
        if (nameSort == true) {
            reverse = !reverse;
            if (nameIcon == faSortUp) {
                nameIcon = faSortDown;
            } else {
                nameIcon = faSortUp;
            }
        } else {
            pidSort = false;
            cpuSort = false;
            ramSort = false;
            nameSort = true;
            reverse = false;
            nameIcon = faSortUp;
            pidIcon = faSort;
            cpuIcon = faSort;
            ramIcon = faSort;
            statusSort = false;
            statusIcon = faSort;
        }
    }

    function sortPid(reverse) {
        socketData.processes.sort((a, b) => {
            if (a.pid < b.pid) {
                return reverse ? 1 : -1;
            } else if (a.pid > b.pid) {
                return reverse ? -1 : 1;
            } else {
                return 0;
            }
        });
        socketData = socketData;
    }

    function setPid() {
        if (pidSort == true) {
            reverse = !reverse;
            if (pidIcon == faSortUp) {
                pidIcon = faSortDown;
            } else {
                pidIcon = faSortUp;
            }
        } else {
            cpuSort = false;
            ramSort = false;
            nameSort = false;
            pidSort = true;
            reverse = false;
            pidIcon = faSortUp;
            cpuIcon = faSort;
            ramIcon = faSort;
            nameIcon = faSort;
            statusSort = false;
            statusIcon = faSort;
        }
    }

    function sortRAM(reverse) {
        socketData.processes.sort((a, b) => {
            if (a.ram < b.ram) {
                return reverse ? -1 : 1;
            } else if (a.ram > b.ram) {
                return reverse ? 1 : -1;
            } else {
                return 0;
            }
        });
        socketData = socketData;
    }

    function setRAM() {
        if (ramSort == true) {
            reverse = !reverse;
            if (ramIcon == faSortUp) {
                ramIcon = faSortDown;
            } else {
                ramIcon = faSortUp;
            }
        } else {
            pidSort = false;
            cpuSort = false;
            nameSort = false;
            ramSort = true;
            reverse = false;
            ramIcon = faSortUp;
            cpuIcon = faSort;
            nameIcon = faSort;
            pidIcon = faSort;
            statusSort = false;
            statusIcon = faSort;
        }
    }

    function sortStatus(reverse) {
        socketData.processes.sort((a, b) => {
            if (a.status < b.status) {
                return reverse ? -1 : 1;
            } else if (a.status > b.status) {
                return reverse ? 1 : -1;
            } else {
                return 0;
            }
        });
        socketData = socketData;
    }

    function setStatus() {
        if (statusSort == true) {
            reverse = !reverse;
            if (statusIcon == faSortUp) {
                statusIcon = faSortDown;
            } else {
                statusIcon = faSortUp;
            }
        } else {
            pidSort = false;
            cpuSort = false;
            nameSort = false;
            statusSort = true;
            reverse = false;
            ramIcon = faSort;
            cpuIcon = faSort;
            nameIcon = faSort;
            pidIcon = faSort;
            statusIcon = faSortUp;
        }
    }

    function sendSignal(signal, pid) {
        socket.send(JSON.stringify({ cmd: signal, args: [pid.toString()] }));
    }
</script>

<main>
    {#if socketData.processes != undefined}
        <table
            class="border border-gray-300 dark:border-gray-700 w-full table-fixed break-words overflow-x-scroll"
        >
            <tr class="bg-dplime" style="color:#000">
                <th
                    >PID<span on:click={setPid}
                        ><Fa
                            icon={pidIcon}
                            class="float-right cursor-pointer"
                        /></span
                    ></th
                >
                <th
                    >Name<span on:click={setName}
                        ><Fa
                            icon={nameIcon}
                            class="float-right cursor-pointer"
                        /></span
                    ></th
                >
                <th
                    >Status<span on:click={setStatus}
                        ><Fa
                            icon={statusIcon}
                            class="float-right cursor-pointer"
                        /></span
                    ></th
                >
                <th
                    >CPU Usage<span on:click={setCPU}
                        ><Fa
                            icon={cpuIcon}
                            class="float-right cursor-pointer"
                        /></span
                    ></th
                >
                <th
                    >RAM Usage<span on:click={setRAM}
                        ><Fa
                            icon={ramIcon}
                            class="float-right cursor-pointer"
                        /></span
                    ></th
                >
                <th>Actions</th>
            </tr>
            {#each socketData.processes as process}
                <tr
                    class="mt-32 even:bg-white odd:bg-gray-200 dark:even:bg-black dark:odd:bg-gray-800  dark:border-gray-600 border-t-2 border-gray-300 border-opacity-50"
                >
                    <td class="p-2">{process.pid}</td>
                    <td class="p-2">{process.name}</td>
                    <td class="p-2">{process.status}</td>
                    <td class="p-2">{process.cpu}%</td>
                    <td class="p-2">{process.ram}MiB</td>
                    <td class="p-2 space-x-2">
                        {#if process.name != "dietpi-dashboard"}
                            <span
                                on:click={() =>
                                    sendSignal("terminate", process.pid)}
                                title="Terminate"
                                ><Fa
                                    icon={faBan}
                                    class="btn rounded-sm p-0.5"
                                    size="lg"
                                /></span
                            >
                            <span
                                on:click={() => sendSignal("kill", process.pid)}
                                title="Kill"
                                ><Fa
                                    icon={faSkull}
                                    class="btn rounded-sm p-0.5"
                                    size="lg"
                                /></span
                            >
                            {#if process.status != "stopped"}
                                <span
                                    on:click={() =>
                                        sendSignal("suspend", process.pid)}
                                    title="Suspend"
                                    ><Fa
                                        icon={faPause}
                                        class="btn rounded-sm p-0.5"
                                        size="lg"
                                    /></span
                                >
                            {:else}
                                <span
                                    on:click={() =>
                                        sendSignal("resume", process.pid)}
                                    title="Resume"
                                    ><Fa
                                        icon={faPlay}
                                        class="btn rounded-sm p-0.5"
                                        size="lg"
                                    /></span
                                >
                            {/if}
                        {/if}
                    </td>
                </tr>
            {/each}
        </table>
    {:else}
        <h3>Getting data...</h3>
    {/if}
</main>
