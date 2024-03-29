<script lang="ts">
    import { navigate, Route, Router } from "svelte-routing";
    import { fade, slide } from "svelte/transition";
    import { cmp } from "semver-compare-multi";
    import Home from "./pages/Home.svelte";
    import Process from "./pages/Process.svelte";
    import Software from "./pages/Software.svelte";
    import Terminal from "./pages/Terminal.svelte";
    import SidebarMenu from "./components/SidebarMenu.svelte";
    import Management from "./pages/Management.svelte";
    import FileBrowser from "./pages/FileBrowser.svelte";
    import Service from "./pages/Service.svelte";

    import logo from "./assets/dietpi.png";
    import github from "./assets/github-mark.svg";
    import { socket } from "./websocket";

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
    let menu = window.innerWidth > 768;
    let dpUpdate = "";
    let tempUnit: "fahrenheit" | "celsius";
    let navPage = "";
    let token = "";
    let password = "";
    let frontendVersion = __PACKAGE_VERSION__;
    let backendVersion = "";
    let updateAvailable = "";
    let node = window.location.host;
    let tokens: Record<string, string> = JSON.parse(
        localStorage.getItem("tokens") ?? "{}"
    );

    $: $socket && (onSocketMessage(), (shown = true));
    $: node !== window.location.host && socket.reopen(node);
    $: notify =
        dpUpdate !== "" ||
        cmp(frontendVersion, backendVersion) !== 0 ||
        updateAvailable !== "";

    // Get dark mode
    let darkModeTemp = localStorage.getItem("darkMode");
    if (darkModeTemp !== null) {
        darkMode = JSON.parse(darkModeTemp);
    } else {
        darkMode = window.matchMedia("(prefers-color-scheme: dark)").matches;
    }

    const updateCheck = () => {
        let updateCheckTemp = localStorage.getItem("update-check");
        if (
            updateCheckTemp === null ||
            JSON.parse(updateCheckTemp).lastChecked + 86400 <
                Math.round(Date.now() / 1000)
        ) {
            fetch(
                "https://api.github.com/repos/ravenclaw900/DietPi-Dashboard/releases/latest"
            ).then(response =>
                response.text().then(body => {
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
        } else if (updateCheckTemp !== null) {
            let version = JSON.parse(updateCheckTemp).version;
            if (cmp(version, backendVersion) > 0) {
                updateAvailable = version;
            }
        }
    };

    function onSocketMessage() {
        if ($socket) {
            if ($socket.dataKind === "GLOBAL") {
                dpUpdate = $socket.update;
                login = $socket.login;
                if ($socket.nodes) {
                    nodes = $socket.nodes;
                }
                backendVersion = $socket.version;
                tempUnit = $socket.temp_unit;
                // Get token
                if (login) {
                    if (tokens[node] === undefined) {
                        // Login
                        loginDialog = true;
                    } else {
                        // Or use stored token
                        token = tokens[node];
                        socket.send({ token });
                        loginDialog = false;
                        pollServer(window.location.pathname);
                    }
                } else {
                    // Remove legacy "token" setting
                    localStorage.removeItem("token");
                    localStorage.removeItem("tokens");
                    token = "";
                    pollServer(window.location.pathname);
                }
                if ($socket.update_check) {
                    updateCheck();
                }
            }
            if ($socket.dataKind === "REAUTH") {
                loginDialog = true;
            }
            if (navPage) {
                blur = false;
                navigate(navPage);
                navPage = "";
            }
        }
    }

    function pollServer(page: string) {
        if (page !== "/terminal") {
            // Terminal doesn't work if sent
            socket.send({
                page,
            });
        }
    }

    function changePage(page: string) {
        if (page !== window.location.pathname) {
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
        fetch(`${window.location.protocol}//${node}/login`, options).then(response => {
            password = "";
            if (response.status === 401) {
                passwordMessage = true;
                setTimeout(() => (passwordMessage = false), 2000);
                return;
            }
            response.text().then(body => {
                token = body;
                tokens[node] = body;
                localStorage.setItem("tokens", JSON.stringify(tokens));
                loginDialog = false;
                socket.send({ token });
                pollServer(window.location.pathname);
            });
        });
    }
</script>

<main
    class="min-h-screen flex"
    class:lt-sm:overflow-x-hidden={menu}
    class:dark={darkMode}
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
                    <h6 class="text-red-500" transition:fade>Incorrect password</h6>
                {/if}
            </div>
        </div>
    {/if}
    <SidebarMenu {menu} {changePage} />
    <div class="flex flex-col flex-grow w-5/6 min-h-full">
        <header class="grid grid-cols-3 items-center h-12 bg-dplime">
            <button
                on:click={() => (menu = !menu)}
                class="justify-self-start p-1 ml-1 btn i-fa-bars text-4xl"
            />
            <a href="https://dietpi.com" class="justify-self-center" target="_blank"
                ><img src={logo} alt="DietPi logo" class="h-10" /></a
            >
            <div class="flex justify-around">
                {#if nodes.length !== 0}
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
                    <button
                        class="md:hidden i-fa-gear text-2xl flex-shrink-0"
                        on:click={() => (settingsShown = !settingsShown)}
                    />
                {/if}
                {#if notify}
                    <button
                        on:click={() => (notificationsShown = !notificationsShown)}
                        class="flex"
                    >
                        <div class="i-fa-envelope text-2xl" />
                        <div
                            class="i-svg-spinners-pulse-multiple text-red-600 -ml-2 place-self-end"
                        />
                    </button>
                {:else}
                    <button
                        on:click={() => (notificationsShown = !notificationsShown)}
                        class="i-fa-envelope text-2xl flex-shrink-0"
                    />
                {/if}
                <button
                    class="text-2xl flex-shrink-0 {darkMode ? 'i-fa-moon' : 'i-fa-sun'}"
                    on:click={() => (
                        (darkMode = !darkMode),
                        localStorage.setItem("darkMode", darkMode.toString())
                    )}
                />
            </div>
        </header>
        {#if notificationsShown}
            <div class="p-2 bg-gray-50 dark:bg-gray-800 dark:text-white" transition:slide>
                <div class="min-h-10">
                    <table class="w-full">
                        {#if dpUpdate}
                            <tr class="border-b border-gray-300 border-gray-600"
                                >DietPi update available: {dpUpdate}</tr
                            >
                        {/if}
                        {#if cmp(frontendVersion, backendVersion) !== 0}
                            <tr class="border-b border-gray-300 border-gray-600"
                                >Warning: Current node is running a version of
                                DietPi-Dashboard {cmp(frontendVersion, backendVersion) < 0
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
            class="dark:bg-gray-900 bg-gray-100 flex-grow p-4 md:p-6 dark:text-white"
            class:blur-2={blur}
        >
            {#if shown && $socket !== null}
                <Router>
                    <Route path="process"><Process /></Route>
                    <Route path="/"><Home {darkMode} {tempUnit} /></Route>
                    <Route path="software"><Software /></Route>
                    <Route path="terminal"><Terminal {node} {token} /></Route>
                    <Route path="management"><Management /></Route>
                    <Route path="browser"><FileBrowser {node} {login} {token} /></Route>
                    <Route path="service"><Service /></Route>
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
            <a href="https://github.com/ravenclaw900/DietPi-Dashboard" target="_blank"
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
