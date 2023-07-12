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
    <NavbarLink icon="i-fa-gauge-high" navFunction={() => changePage("/")}
        >Statistics</NavbarLink
    >
    <NavbarLink icon="i-fa-microchip" navFunction={() => changePage("/process")}
        >Processes</NavbarLink
    >
    <NavbarLink icon="i-fa-list" navFunction={() => changePage("/service")}
        >Services</NavbarLink
    >
    <NavbarLink icon="i-fa-database" navFunction={() => changePage("/software")}
        >Software</NavbarLink
    >
    <NavbarLink icon="i-fa-terminal" navFunction={() => navigate("/terminal")}
        >Terminal</NavbarLink
    >
    <NavbarLink icon="i-fa-user" navFunction={() => changePage("/management")}
        >Management</NavbarLink
    >
    <NavbarLink icon="i-fa-folder" navFunction={() => changePage("/browser")}
        >File Browser</NavbarLink
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
