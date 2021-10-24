<script lang="ts">
    import { Router, Route } from "svelte-routing";
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
        currentpath?: string;
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
    let shown = false;
    let menu = window.innerWidth > 768;
    let update = "";

    const socketMessageListener = (e) => {
        socketData = JSON.parse(e.data);
        if (socketData.update != undefined) {
            update = socketData.update;
        }
    };
    const socketOpenListener = () => {
        console.log("Connected");
        shown = true;
        pollServer();
    };
    const socketErrorListener = (e) => {
        console.error(e);
    };
    const socketCloseListener = () => {
        if (socket) {
            console.log("Disconnected");
        }
        socket = new WebSocket(`ws://${window.location.hostname}:8080/ws`);
        socket.onopen = socketOpenListener;
        socket.onmessage = socketMessageListener;
        socket.onclose = socketCloseListener;
        socket.onerror = socketErrorListener;
    };

    function pollServer() {
        socket.send(JSON.stringify({ page: window.location.pathname }));
    }

    onMount(() => {
        socketCloseListener();
    });
</script>

<main class="min-h-screen flex overflow-x-hidden dark:text-white">
    <Router {url}>
        <div
            class="bg-gray-900 dark:bg-black flex-grow{menu ? '' : ' shrink'}"
            id="sidebarMenu"
        >
            <div
                class="hidden lg:flex whitespace-nowrap h-12 bg-dplime-dark text-black text-2xl items-center justify-center"
            >
                DietPi Dashboard
            </div>
            <span on:click={pollServer}
                ><NavbarLink icon={faTachometerAlt} to="/"
                    >Statistics</NavbarLink
                ></span
            >
            <span on:click={pollServer}
                ><NavbarLink icon={faMicrochip} to="process"
                    >Processes</NavbarLink
                ></span
            >
            <span on:click={pollServer}
                ><NavbarLink icon={faList} to="service">Services</NavbarLink
                ></span
            >
            <span on:click={pollServer}
                ><NavbarLink icon={faDatabase} to="software"
                    >Software</NavbarLink
                ></span
            >
            <NavbarLink icon={faTerminal} to="terminal">Terminal</NavbarLink>
            <span on:click={pollServer}
                ><NavbarLink icon={faUser} to="management"
                    >Management</NavbarLink
                ></span
            >
            <span on:click={pollServer}
                ><NavbarLink icon={faFolder} to="browser"
                    >File Browser</NavbarLink
                ></span
            >
        </div>
        <div class="w-5/6 flex flex-col flex-grow min-h-full">
            <header class="bg-dplime h-12 grid grid-cols-3 items-center">
                <span
                    on:click={() => (menu = !menu)}
                    class="justify-self-start text-black"
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
            </header>
            <div class="dark:bg-gray-900 bg-gray-100 flex-grow p-6">
                {#if shown}
                    <Route path="process"
                        ><Process {socketData} {socket} /></Route
                    >
                    <Route path="/"><Home {socketData} /></Route>
                    <Route path="software"
                        ><Software {socketData} {socket} /></Route
                    >
                    <Route path="terminal"><Terminal /></Route>
                    <Route path="management"
                        ><Management {socket} {socketData} /></Route
                    >
                    <Route path="browser"
                        ><FileBrowser {socket} {socketData} /></Route
                    >
                    <Route path="service"
                        ><Service {socket} {socketData} /></Route
                    >
                    <Route path=""><h3>Page not found</h3></Route>
                {:else}
                    <h3>Connecting to API...</h3>
                {/if}
            </div>
            <footer
                class="border-t bg-gray-200 dark:bg-gray-800 dark:border-gray-700 border-gray-300 h-16 flex flex-col justify-center items-center"
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
    </Router>
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
