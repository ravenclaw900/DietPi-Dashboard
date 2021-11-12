<script lang="ts">
    import { navigate, Route, Router } from "svelte-routing";
    import { onMount } from "svelte";
    import Home from "./pages/Home.svelte";
    import Process from "./pages/Process.svelte";
    import Software from "./pages/Software.svelte";
    import Terminal from "./pages/Terminal.svelte";
    import NavbarLink from "./components/NavbarLink.svelte";
    import Fa from "svelte-fa";
    import { faGithub } from "@fortawesome/free-brands-svg-icons";
    import {
        faTachometerAlt,
        faMicrochip,
        faDatabase,
        faTerminal,
        faUser,
        faBars,
        faList,
        faFolder,
        faSun,
        faMoon,
    } from "@fortawesome/free-solid-svg-icons";
    import Management from "./pages/Management.svelte";
    import FileBrowser from "./pages/FileBrowser.svelte";
    import Service from "./pages/Service.svelte";

    import logo from "./assets/dietpi.png";

    interface socketData {
        // Software page
        software?: software[];
        response?: string;
        // Process page
        processes?: processes[];
        // Services page
        services?: services[];
        // File browser page
        contents?: browser[];
        textdata?: string;
        // Global
        update?: string;
    }

    interface software {
        id: number;
        installed: boolean;
        name: string;
        description: string;
        dependencies: string;
        docs: string;
    }

    interface processes {
        pid: number;
        name: string;
        cpu: number;
        ram: number;
        status: string;
    }

    interface services {
        name: string;
        status: string;
        log: string;
        start: string;
    }

    interface browser {
        name: string;
        path: string;
        prettytype: string;
        maintype: string;
        subtype: string;
        size: number;
    }

    let url = "";

    let socket;
    let socketData: socketData = {};
    let binData = "";
    let shown = false;
    let menu = window.innerWidth > 768;
    let update = "";
    let darkMode = window.matchMedia("(prefers-color-scheme: dark)").matches;
    let darkIcon;
    let blur = false;
    let navPage = "";
    $: darkIcon = darkMode ? faMoon : faSun;

    const socketMessageListener = (e) => {
        if (typeof e.data === "string") {
            socketData = JSON.parse(e.data);
            binData = "";
        } else {
            socketData = {};
            binData = URL.createObjectURL(e.data);
        }
        if (socketData.update != undefined) {
            update = socketData.update;
        }
        if (navPage) {
            blur = false;
            navigate(navPage);
            navPage = "";
        }
    };
    const socketOpenListener = () => {
        console.log("Connected");
        shown = true;
        pollServer(window.location.pathname);
    };
    const socketErrorListener = (e) => {
        console.error(e);
    };
    const socketCloseListener = () => {
        if (socket) {
            console.log("Disconnected");
        }
        let proto = window.location.protocol == "https:" ? "wss" : "ws";
        socket = new WebSocket(
            `${proto}://${window.location.hostname}:${window.location.port}/ws`
        );
        socket.onopen = socketOpenListener;
        socket.onmessage = socketMessageListener;
        socket.onclose = socketCloseListener;
        socket.onerror = socketErrorListener;
    };

    function pollServer(page: string) {
        socket.send(JSON.stringify({ page }));
        navPage = page;
    }

    function changePage(page: string) {
        if (page != window.location.pathname) {
            blur = true;
            pollServer(page);
        }
        // Continued in socketMessageListener
    }

    onMount(() => {
        socketCloseListener();
    });
</script>

<main class="min-h-screen flex overflow-x-hidden{darkMode ? ' dark' : ''}">
    <div
        class="bg-gray-900 dark:bg-black flex-grow{menu ? '' : ' shrink'}"
        id="sidebarMenu"
    >
        <div
            class="hidden lg:flex whitespace-nowrap h-12 bg-dplime-dark text-2xl items-center justify-center"
        >
            DietPi Dashboard
        </div>
        <span on:click={() => changePage("/")}
            ><NavbarLink icon={faTachometerAlt}>Statistics</NavbarLink></span
        >
        <span on:click={() => changePage("/process")}
            ><NavbarLink icon={faMicrochip}>Processes</NavbarLink></span
        >
        <span on:click={() => changePage("/service")}
            ><NavbarLink icon={faList}>Services</NavbarLink></span
        >
        <span on:click={() => changePage("/software")}
            ><NavbarLink icon={faDatabase}>Software</NavbarLink></span
        >
        <span on:click={() => navigate("/terminal")}>
            <NavbarLink icon={faTerminal}>Terminal</NavbarLink>
        </span>
        <span on:click={() => changePage("/management")}
            ><NavbarLink icon={faUser}>Management</NavbarLink></span
        >
        <span on:click={() => changePage("/browser")}
            ><NavbarLink icon={faFolder}>File Browser</NavbarLink></span
        >
    </div>
    <div class="w-5/6 flex flex-col flex-grow min-h-full">
        <header class="bg-dplime h-12 grid grid-cols-3 items-center">
            <span on:click={() => (menu = !menu)} class="justify-self-start"
                ><Fa icon={faBars} class="btn ml-1 p-1" size="3x" /></span
            >
            <a
                href="https://dietpi.com"
                class="justify-self-center"
                target="_blank"
                ><img src={logo} alt="DietPi logo" class="h-10" /></a
            >
            {#if update != ""}
                <span class="text-red-500 justify-self-center"
                    >DietPi update avalible: {update}</span
                >
            {/if}
            <span
                class="cursor-pointer justify-self-end mr-2"
                on:click={() => (darkMode = !darkMode)}
                ><Fa icon={darkIcon} size="lg" /></span
            >
        </header>
        <div
            class="dark:bg-gray-900 bg-gray-100 flex-grow p-6 dark:text-white{blur
                ? ' children:blur-2 children:filter'
                : ''}"
        >
            {#if shown}
                <Router {url}>
                    <Route path="process"
                        ><Process {socketData} {socket} /></Route
                    >
                    <Route path="/"><Home {socketData} {darkMode} /></Route>
                    <Route path="software"
                        ><Software {socketData} {socket} /></Route
                    >
                    <Route path="terminal"><Terminal /></Route>
                    <Route path="management"
                        ><Management {socket} {socketData} /></Route
                    >
                    <Route path="browser"
                        ><FileBrowser {socket} {socketData} {binData} /></Route
                    >
                    <Route path="service"
                        ><Service {socket} {socketData} /></Route
                    >
                    <Route path=""><h3>Page not found</h3></Route>
                </Router>
            {:else}
                <h3>Connecting to API...</h3>
            {/if}
        </div>
        <footer
            class="border-t bg-gray-200 dark:bg-gray-800 dark:border-gray-700 border-gray-300 h-16 flex flex-col justify-center items-center dark:text-white"
        >
            <div>
                DietPi-Dashboard <a
                    class="text-blue-500 dark:text-blue-600"
                    href="https://github.com/ravenclaw900/DietPi-Dashboard/releases/tag/v{'__PACKAGE_VERSION__'}"
                    target="_blank">v{"__PACKAGE_VERSION__"}</a
                > created by ravenclaw900
            </div>
            <a
                href="https://github.com/ravenclaw900/DietPi-Dashboard"
                target="_blank"
                ><Fa
                    icon={faGithub}
                    class="hover:opacity-75 dark:hover:opacity-60"
                    size="2x"
                /></a
            >
        </footer>
    </div>
</main>

<style global>
    #sidebarMenu {
        min-width: 10rem;
        max-width: 16.666667%;
        width: 16.666667%;
        transition: width 1.5s, max-width 1.5s, min-width 1.5s;
    }

    #sidebarMenu.shrink {
        width: 0px;
        max-width: 0px;
        min-width: 0px;
    }
</style>
