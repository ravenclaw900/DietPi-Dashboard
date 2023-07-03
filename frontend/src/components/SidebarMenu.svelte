<script lang="ts">
    import NavbarLink from "./NavbarLink.svelte";
    import { navigate } from "svelte-routing";

    export let menu: boolean;
    export let changePage: (page: string) => void;
</script>

<div
    class="bg-gray-900 dark:bg-black flex-grow w-1/6 2xl:w-10rem"
    class:menu-shrink={!menu}
    id="sidebarMenu"
>
    <div
        class="hidden justify-center items-center h-12 text-2xl whitespace-nowrap lg:flex bg-dplime-dark"
    >
        DietPi Dashboard
    </div>
    <button on:click={() => changePage("/")} class="w-full"
        ><NavbarLink class="i-fa-gauge-high">Statistics</NavbarLink></button
    >
    <button on:click={() => changePage("/process")} class="w-full"
        ><NavbarLink class="i-fa-microchip">Processes</NavbarLink></button
    >
    <button on:click={() => changePage("/service")} class="w-full"
        ><NavbarLink class="i-fa-list">Services</NavbarLink></button
    >
    <button on:click={() => changePage("/software")} class="w-full"
        ><NavbarLink class="i-fa-database">Software</NavbarLink></button
    >
    <button on:click={() => navigate("/terminal")} class="w-full">
        <NavbarLink class="i-fa-terminal">Terminal</NavbarLink>
    </button>
    <button on:click={() => changePage("/management")} class="w-full"
        ><NavbarLink class="i-fa-user">Management</NavbarLink></button
    >
    <button on:click={() => changePage("/browser")} class="w-full"
        ><NavbarLink class="i-fa-folder">File Browser</NavbarLink></button
    >
</div>

<style>
    #sidebarMenu {
        min-width: 10rem;
        max-width: 16.666667%;
        transition: width 1.5s, max-width 1.5s, min-width 1.5s;
    }

    #sidebarMenu.menu-shrink {
        width: 0px;
        max-width: 0px;
        min-width: 0px;
    }
</style>
