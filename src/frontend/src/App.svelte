<script lang="ts">
    import { navigate, Route, Router } from "svelte-routing";
    import { fade, slide } from "svelte/transition";
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
        faEnvelope,
        faEnvelopeOpenText,
        faCog,
    } from "@fortawesome/free-solid-svg-icons";
    import Management from "./pages/Management.svelte";
    import FileBrowser from "./pages/FileBrowser.svelte";
    import Service from "./pages/Service.svelte";

    import logo from "./assets/dietpi.png";

    interface socketData {
        // Statistics page
        cpu: number;
        ram: usage;
        swap: usage;
        disk: usage;
        network: net;
        // Software page
        uninstalled: software[];
        installed: software[];
        response: string;
        // Process page
        processes: processes[];
        // Services page
        services: services[];
        // File browser page
        contents: browser[];
        textdata: string;
        // Management page
        hostname: String;
        uptime: number;
        arch: string;
        kernel: string;
        version: string;
        packages: number;
        upgrades: number;
        nic: string;
        ip: string;
        // Global
        update: string;
        login: boolean;
        error: boolean;
        nodes: string[];
    }

    interface software {
        id: number;
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

    interface usage {
        used: number;
        total: number;
        percent: number;
    }

    interface net {
        sent: number;
        received: number;
    }

    let url = "";

    let socket: WebSocket;
    let socketData: Partial<socketData> = {};
    let binData = "";
    let shown = false;
    let menu = window.innerWidth > 768;
    let update = "";
    let darkMode = false;
    let blur = false;
    let navPage = "";
    let token = "";
    let password = "";
    let login = false;
    let loginDialog = false;
    let nodes: string[] = [];
    let node = `${window.location.hostname}:${window.location.port}`;
    let notificationsShown = false;
    let settingsShown = false;
    let passwordMessage = false;

    // Get dark mode
    if (localStorage.getItem("darkMode") != null) {
        darkMode = JSON.parse(localStorage.getItem("darkMode"));
    } else {
        darkMode = window.matchMedia("(prefers-color-scheme: dark)").matches;
    }

    const socketMessageListener = (e: MessageEvent) => {
        if (typeof e.data === "string") {
            socketData = JSON.parse(e.data);
            binData = "";
        } else {
            socketData = {};
            binData = URL.createObjectURL(e.data);
        }
        if (socketData.update != undefined) {
            update = socketData.update;
            login = socketData.login;
            if (socketData.nodes) {
                nodes = socketData.nodes;
            }
            // Get token
            if (login) {
                let obj = JSON.parse(localStorage.getItem("tokens"));
                if (obj == null || obj[node] == null) {
                    // Login
                    loginDialog = true;
                } else {
                    // Or use stored token
                    token = obj[node];
                    pollServer(window.location.pathname);
                }
            } else {
                localStorage.removeItem("token");
                localStorage.removeItem("tokens");
                pollServer(window.location.pathname);
            }
        }
        if (socketData.error == true) {
            loginDialog = true;
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
    };
    const socketErrorListener = (e: ErrorEvent) => {
        console.error(e);
        connectSocket(node);
    };
    const socketCloseListener = () => {
        console.log("Disconnected");
    };

    function pollServer(page: string) {
        let json: string;
        if (login) {
            json = JSON.stringify({
                page,
                token,
            });
        } else {
            json = JSON.stringify({
                page,
            });
        }
        socket.send(json);
    }

    function changePage(page: string) {
        if (page != window.location.pathname) {
            blur = true;
            pollServer(page);
            navPage = page;
        }
        // Continued in socketMessageListener
    }

    function getToken() {
        const options = {
            method: "POST",
            body: password,
        };
        fetch(`${window.location.protocol}//${node}/login/`, options).then(
            (response) =>
                response.text().then((body) => {
                    password = "";
                    if (body == "Unauthorized") {
                        passwordMessage = true;
                        setTimeout(() => (passwordMessage = false), 2000);
                    } else {
                        token = body;
                        let obj =
                            localStorage.getItem("tokens") == null
                                ? {}
                                : JSON.parse(localStorage.getItem("tokens"));
                        obj[node] = body;
                        localStorage.setItem("tokens", JSON.stringify(obj));
                        loginDialog = false;
                        pollServer(window.location.pathname);
                    }
                })
        );
    }

    function socketSend(cmd: string, args: string[]) {
        let json;
        if (login) {
            json = JSON.stringify({
                cmd,
                args,
                token,
            });
        } else {
            json = JSON.stringify({
                cmd,
                args,
            });
        }
        socket.send(json);
    }

    function connectSocket(url: string) {
        if (socket) {
            socket.close();
        }
        let proto = window.location.protocol == "https:" ? "wss" : "ws";
        socket = new WebSocket(`${proto}://${url}/ws`);
        socket.onopen = socketOpenListener;
        socket.onmessage = socketMessageListener;
        socket.onclose = socketCloseListener;
        socket.onerror = socketErrorListener;
    }

    $: node && ((shown = false), connectSocket(node));
</script>

<main class="min-h-screen flex overflow-x-hidden{darkMode ? ' dark' : ''}">
    {#if loginDialog}
        <div
            class="fixed inset-0 bg-gray-600/50 h-screen w-screen flex items-center justify-center"
            transition:fade
        >
            <div
                class="bg-white dark:bg-black w-1/2 h-1/3 rounded-md flex items-center flex-col justify-center text-xl z-40 gap-5 dark:text-white"
            >
                <h6>Please login:</h6>
                <input
                    type="password"
                    class="outline-none bg-gray-100 border border-gray-400 dark:border-gray-700 rounded focus:bg-gray-200 dark:bg-gray-900 dark:focus:bg-gray-800"
                    bind:value={password}
                />
                <button
                    on:click={getToken}
                    class="border-gray-500 hover:bg-gray-100 dark:hover:bg-gray-900 focus:outline-none border p-2 rounded active:bg-gray-200 dark:active:bg-gray-800"
                    >Login</button
                >
                {#if passwordMessage}
                    <h6 class="text-red-500" transition:fade>
                        Incorrect password
                    </h6>
                {/if}
            </div>
        </div>
    {/if}
    <div
        class="bg-gray-900 dark:bg-black flex-grow{menu
            ? ''
            : ' shrink'} w-1/6 2xl:w-10rem"
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
            <div class="flex justify-around">
                {#if nodes.length != 0}
                    <select bind:value={node} class="hidden md:inline-block">
                        <option
                            value={`${window.location.hostname}:${window.location.port}`}
                            >{`${window.location.hostname}:${window.location.port}`}
                        </option>
                        {#each nodes as node}
                            <option value={node}>
                                {node}
                            </option>
                        {/each}
                    </select>
                {/if}
                <div>
                    <span
                        class="cursor-pointer"
                        on:click={() =>
                            (notificationsShown = !notificationsShown)}
                        ><Fa
                            icon={update ? faEnvelopeOpenText : faEnvelope}
                            size="lg"
                        />
                    </span>
                </div>
                {#if nodes.length != 0}
                    <span
                        class="cursor-pointer md:hidden"
                        on:click={() => (settingsShown = !settingsShown)}
                        ><Fa icon={faCog} size="lg" />
                    </span>
                {/if}
                <span
                    class="cursor-pointer"
                    on:click={() => (
                        (darkMode = !darkMode),
                        localStorage.setItem("darkMode", darkMode.toString())
                    )}><Fa icon={darkMode ? faMoon : faSun} size="lg" /></span
                >
            </div>
        </header>
        {#if notificationsShown}
            <div class="bg-gray-50 dark:bg-gray-800 p-2" transition:slide>
                <div class="min-h-10">
                    <table class="w-full">
                        {#if update}
                            <tr class="border-b border-gray-300 border-gray-600"
                                >DietPi update available: {update}</tr
                            >
                        {/if}
                    </table>
                </div>
            </div>
        {/if}
        {#if settingsShown}
            <div class="bg-gray-50 dark:bg-gray-800 p-2" transition:slide>
                <div class="min-h-10">
                    <table class="w-full">
                        <select bind:value={node} class="w-full">
                            <option
                                value={`${window.location.hostname}:${window.location.port}`}
                                >{`${window.location.hostname}:${window.location.port}`}
                            </option>
                            {#each nodes as node}
                                <option value={node}>
                                    {node}
                                </option>
                            {/each}
                        </select>
                    </table>
                </div>
            </div>
        {/if}
        <div
            class="dark:bg-gray-900 bg-gray-100 flex-grow p-4 md:p-6 dark:text-white{blur
                ? ' children:blur-2 children:filter'
                : ''}"
        >
            {#if shown}
                <Router {url}>
                    <Route path="process"
                        ><Process {socketData} {socketSend} /></Route
                    >
                    <Route path="/"><Home {socketData} {darkMode} /></Route>
                    <Route path="software"
                        ><Software {socketData} {socketSend} /></Route
                    >
                    <Route path="terminal"
                        ><Terminal {loginDialog} {node} /></Route
                    >
                    <Route path="management"
                        ><Management {socketSend} {socketData} /></Route
                    >
                    <Route path="browser"
                        ><FileBrowser
                            {socketSend}
                            {socketData}
                            {binData}
                        /></Route
                    >
                    <Route path="service"
                        ><Service {socketSend} {socketData} /></Route
                    >
                    <Route path=""><h3>Page not found</h3></Route>
                </Router>
            {:else}
                <h3>Connecting to API...</h3>
            {/if}
        </div>
        <footer
            class="border-t bg-gray-200 dark:bg-gray-800 dark:border-gray-700 border-gray-300 min-h-16 flex flex-col justify-center items-center dark:text-white"
        >
            <div>
                DietPi-Dashboard <a
                    class="text-blue-500 dark:text-blue-600"
                    href="https://github.com/ravenclaw900/DietPi-Dashboard/releases/tag/v{'__PACKAGE_VERSION__'}"
                    target="_blank">v{"__PACKAGE_VERSION__"}</a
                >
                created by ravenclaw900.
                <a
                    class="text-blue-500 dark:text-blue-600"
                    href="https://dietpi.com/docs/software/system_stats/#dietpi-dashboard"
                    target="_blank">More Info</a
                >
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
        transition: width 1.5s, max-width 1.5s, min-width 1.5s;
    }

    #sidebarMenu.shrink {
        width: 0px;
        max-width: 0px;
        min-width: 0px;
    }
</style>
