<script>
    import Fa from "svelte-fa";
    import {
        faSort,
        faSortUp,
        faSortDown,
    } from "@fortawesome/free-solid-svg-icons";

    export let socketData;
    let pidSort = true;
    let pidIcon = faSortUp;
    let nameSort = false;
    let nameIcon = faSort;
    let cpuSort = false;
    let cpuIcon = faSort;
    let ramSort = false;
    let ramIcon = faSort;
    let reverse = false;

    $: cpuSort && socketData.processes && sortCPU(reverse);
    $: pidSort && socketData.processes && sortPid(reverse);
    $: nameSort && socketData.processes && sortName(reverse);
    $: ramSort && socketData.processes && sortRAM(reverse);

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
        }
    }
</script>

<main>
    {#if socketData.processes != undefined}
        <table class="border border-gray-300 w-full">
            <tr class="bg-dplime">
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
            </tr>
            {#each socketData.processes as process}
                <tr
                    class="mt-32 even:bg-white odd:bg-gray-200 border-t-2 border-gray-300 border-opacity-50"
                >
                    <td class="p-2">{process.pid}</td>
                    <td class="p-2">{process.name}</td>
                    <td class="p-2">{process.cpu}%</td>
                    <td class="p-2">{process.ram}MiB</td>
                </tr>
            {/each}
        </table>
    {:else}
        <h3>Getting data...</h3>
    {/if}
</main>
