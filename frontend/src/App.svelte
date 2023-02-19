<script lang="ts">
    import { navigate, Route, Router } from "svelte-routing";
    import { fade, slide } from "svelte/transition";
    import { cmp } from "semver-compare-multi";
    import Home from "./pages/home/Main.svelte";
    import Process from "./pages/Process.svelte";
    import Software from "./pages/Software.svelte";
    import Terminal from "./pages/Terminal.svelte";
    import NavbarLink from "./components/NavbarLink.svelte";
    import Fa from "svelte-fa";
    import FaLayers from "svelte-fa/src/fa-layers.svelte";
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
        faCog,
        faCircle,
    } from "@fortawesome/free-solid-svg-icons";
    import Management from "./pages/Management.svelte";
    import FileBrowser from "./pages/FileBrowser.svelte";
    import Service from "./pages/Service.svelte";

    import logo from "./assets/dietpi.png";
    import github from "./assets/github-mark.svg";
    import { type socketData, MessageKind } from "./types";

    let socket: WebSocket;
    let socketData: socketData;
    let nodes: string[] = [];
    let shown = false;
    let darkMode = false;
    let blur = false;
    let login = false;
    let loginDialog = false;
    let notificationsShown = false;
    let settingsShown = false;
    let passwordMessage = false;
    let notify = false;
    let reopenSocket = true;
    let menu = window.innerWidth > 768;
    let dpUpdate = "";
    let tempUnit: "fahrenheit" | "celsius";
    let navPage = "";
    let token = "";
    let password = "";
    let frontendVersion = "__PACKAGE_VERSION__";
    let backendVersion = "";
    let updateAvailable = "";
    let node = `${window.location.hostname}:${window.location.port}`;

    $: node && (((shown = false), (reopenSocket = false)), connectSocket(node));
    $: notify =
        dpUpdate != "" ||
        cmp(frontendVersion, backendVersion) != 0 ||
        updateAvailable != "";

    // Get dark mode
    if (localStorage.getItem("darkMode") != null) {
        darkMode = JSON.parse(localStorage.getItem("darkMode"));
    } else {
        darkMode = window.matchMedia("(prefers-color-scheme: dark)").matches;
    }

    const updateCheck = () => {
        if (
            localStorage.getItem("update-check") == null ||
            JSON.parse(localStorage.getItem("update-check")).lastChecked +
                86400 <
                Math.round(Date.now() / 1000)
        ) {
            fetch(
                "https://api.github.com/repos/ravenclaw900/DietPi-Dashboard/releases/latest"
            ).then((response) =>
                response.text().then((body) => {
                    let version = JSON.parse(body).name.substring(1);
                    if (cmp(version, backendVersion) > 0) {
                        updateAvailable = version;
                    }
                    localStorage.setItem(
                        "update-check",
                        JSON.stringify({
                            version,
                            lastChecked: Math.round(Date.now() / 1000),
                        })
                    );
                })
            );
        } else if (localStorage.getItem("update-check") != null) {
            let version = JSON.parse(
                localStorage.getItem("update-check")
            ).version;
            if (cmp(version, backendVersion) > 0) {
                updateAvailable = version;
            }
        }
    };

    const socketMessageListener = (e: MessageEvent) => {
        socketData = JSON.parse(e.data);
        if (socketData.kind == MessageKind.Global) {
            dpUpdate = socketData.update;
            login = socketData.login;
            if (socketData.nodes) {
                nodes = socketData.nodes;
            }
            backendVersion = socketData.version;
            tempUnit = socketData.temp_unit;
            // Get token
            if (login) {
                let obj = JSON.parse(localStorage.getItem("tokens"));
                if (obj == null || obj[node] == null) {
                    // Login
                    loginDialog = true;
                } else {
                    // Or use stored token
                    token = obj[node];
                    socket.send(JSON.stringify({ token }));
                    pollServer(window.location.pathname);
                }
            } else {
                // Remove legacy "token" setting
                localStorage.removeItem("token");
                localStorage.removeItem("tokens");
                token = "";
                pollServer(window.location.pathname);
            }
            if (socketData.update_check) {
                updateCheck();
            }
        }
        if (socketData.kind == MessageKind.Reauth) {
            loginDialog = true;
        }
        if (navPage) {
            blur = false;
            navigate(navPage);
            navPage = "";
        }
    };
    const socketOpenListener = () => {
        console.info("Connected");
        shown = true;
        loginDialog = false;
    };
    const socketErrorListener = (e: ErrorEvent) => {
        console.error(e);
    };
    const socketCloseListener = (e: CloseEvent) => {
        console.info("Disconnected, reconnecting:", reopenSocket);
        if (reopenSocket) {
            setTimeout(() => connectSocket(node), 1000);
        } else {
            reopenSocket = true;
        }
    };

    function pollServer(page: string) {
        if (page != "/terminal") {
            // Terminal doesn't work if sent
            socket.send(
                JSON.stringify({
                    page,
                })
            );
        }
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
            (response) => {
                password = "";
                if (response.status == 401) {
                    passwordMessage = true;
                    setTimeout(() => (passwordMessage = false), 2000);
                    return;
                }
                response.text().then((body) => {
                    token = body;
                    let obj =
                        localStorage.getItem("tokens") == null
                            ? {}
                            : JSON.parse(localStorage.getItem("tokens"));
                    obj[node] = body;
                    localStorage.setItem("tokens", JSON.stringify(obj));
                    loginDialog = false;
                    socket.send(JSON.stringify({ token }));
                    pollServer(window.location.pathname);
                });
            }
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
        } else {
            reopenSocket = true;
        }
        let proto = window.location.protocol == "https:" ? "wss" : "ws";
        socket = new WebSocket(`${proto}://${url}/ws`);
        socket.onopen = socketOpenListener;
        socket.onmessage = socketMessageListener;
        socket.onclose = socketCloseListener;
        socket.onerror = socketErrorListener;
    }
</script>

<main
    class="min-h-screen flex{menu ? ' <sm:overflow-x-hidden' : ''}{darkMode
        ? ' dark'
        : ''}"
>
    {#if loginDialog}
        <div
            class="flex fixed inset-0 z-20 justify-center items-center w-screen h-screen bg-gray-600/50"
            transition:fade
        >
            <div
                class="flex z-40 flex-col gap-5 justify-center items-center w-1/2 h-1/3 text-xl bg-white rounded-md dark:bg-black dark:text-white"
            >
                <h6>Please login:</h6>
                <form
                    class="flex flex-col gap-5 items-center"
                    on:submit|preventDefault={getToken}
                >
                    <input
                        type="password"
                        class="bg-gray-100 rounded border border-gray-400 outline-none dark:border-gray-700 focus:bg-gray-200 dark:bg-gray-900 dark:focus:bg-gray-800"
                        bind:value={password}
                    />
                    <button
                        type="submit"
                        class="p-2 rounded border border-gray-500 hover:bg-gray-100 dark:hover:bg-gray-900 focus:outline-none active:bg-gray-200 dark:active:bg-gray-800"
                        >Login</button
                    >
                </form>
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
            class="hidden justify-center items-center h-12 text-2xl whitespace-nowrap lg:flex bg-dplime-dark"
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
    <div class="flex flex-col flex-grow w-5/6 min-h-full">
        <header class="grid grid-cols-3 items-center h-12 bg-dplime">
            <span on:click={() => (menu = !menu)} class="justify-self-start"
                ><Fa icon={faBars} class="p-1 ml-1 btn" size="3x" /></span
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
                        >{#if notify}
                            <FaLayers size="lg">
                                <Fa icon={faEnvelope} />
                                <Fa
                                    icon={faCircle}
                                    scale={0.5}
                                    translateX={0.25}
                                    translateY={0.25}
                                    color="tomato"
                                    class="animate-pulse"
                                />
                            </FaLayers>
                        {:else}
                            <Fa icon={faEnvelope} size="lg" />
                        {/if}
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
            <div
                class="p-2 bg-gray-50 dark:bg-gray-800 dark:text-white"
                transition:slide
            >
                <div class="min-h-10">
                    <table class="w-full">
                        {#if dpUpdate}
                            <tr class="border-b border-gray-300 border-gray-600"
                                >DietPi update available: {dpUpdate}</tr
                            >
                        {/if}
                        {#if cmp(frontendVersion, backendVersion) != 0}
                            <tr class="border-b border-gray-300 border-gray-600"
                                >Warning: Current node is running a version of
                                DietPi-Dashboard {cmp(
                                    frontendVersion,
                                    backendVersion
                                ) < 0
                                    ? "greater"
                                    : "lower"} than the main node (main: {frontendVersion},
                                node: {backendVersion})</tr
                            >
                        {/if}
                        {#if updateAvailable}
                            <tr class="border-b border-gray-300 border-gray-600"
                                >DietPi-Dashboard update available: {updateAvailable}</tr
                            >
                        {/if}
                    </table>
                </div>
            </div>
        {/if}
        {#if settingsShown}
            <div class="p-2 bg-gray-50 dark:bg-gray-800" transition:slide>
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
                <Router>
                    {#if socketData.kind == MessageKind.Process}
                        <Route path="process"
                            ><Process {socketData} {socketSend} /></Route
                        >
                    {/if}
                    {#if socketData.kind == MessageKind.Statistics}
                        <Route path="/"
                            ><Home {socketData} {darkMode} {tempUnit} /></Route
                        >
                    {/if}
                    {#if socketData.kind == MessageKind.Software}
                        <Route path="software"
                            ><Software {socketData} {socketSend} /></Route
                        >
                    {/if}
                    <Route path="terminal"><Terminal {node} {token} /></Route>
                    {#if socketData.kind == MessageKind.Management}
                        <Route path="management"
                            ><Management {socketSend} {socketData} /></Route
                        >
                    {/if}
                    {#if socketData.kind == MessageKind.Browser}
                        <Route path="browser"
                            ><FileBrowser
                                {socketSend}
                                {socketData}
                                {node}
                                {login}
                            /></Route
                        >
                    {/if}
                    {#if socketData.kind == MessageKind.Service}
                        <Route path="service"
                            ><Service {socketSend} {socketData} /></Route
                        >
                    {/if}
                    <Route path=""><h3>Page not found</h3></Route>
                </Router>
            {:else}
                <h3>Connecting to API...</h3>
            {/if}
        </div>
        <footer
            class="flex flex-col justify-center items-center bg-gray-200 border-t border-gray-300 dark:bg-gray-800 dark:border-gray-700 min-h-16 dark:text-white"
        >
            <div>
                DietPi-Dashboard <a
                    class="text-blue-500 dark:text-blue-600"
                    href="https://github.com/ravenclaw900/DietPi-Dashboard/releases/tag/v{frontendVersion}"
                    target="_blank">v{frontendVersion}</a
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
                ><img
                    src={github}
                    width="30px"
                    class="hover:opacity-75 dark:hover:opacity-60"
                    alt="GitHub icon"
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
