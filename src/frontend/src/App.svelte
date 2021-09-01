<script lang="ts">
    import { Router, Route } from "svelte-routing";
    import { onMount } from 'svelte';
    import Home from "./pages/Home.svelte";
    import Process from "./pages/Process.svelte";
    import Software from "./pages/Software.svelte";
    import NavbarLink from "./components/NavbarLink.svelte"
    import Fa from 'svelte-fa'
    import { faGithub } from '@fortawesome/free-brands-svg-icons'
    import { faTachometerAlt, faMicrochip, faDatabase } from "@fortawesome/free-solid-svg-icons"
    let url = "";

    let socket
    let socketData = {}
    let shown = false
    const socketMessageListener = (e) => {
        socketData = JSON.parse(e.data)
    };
    const socketOpenListener = () => {
        console.log('Connected')
        shown = true
    };
    const socketErrorListener = (e) => {
        console.error(e)
    }
    const socketCloseListener = () => {
        if (socket) {
            console.log('Disconnected.');
        }
        socket = new WebSocket(`ws://${window.location.hostname}:8080/ws`)
        socket.onopen = socketOpenListener
        socket.onmessage = socketMessageListener
        socket.onclose = socketCloseListener
        socket.onerror = socketErrorListener
    };

    function pollServer() {
        socket.send(JSON.stringify({page: window.location.pathname}))
    }

    $: shown && pollServer()

    onMount(() => {
        socketCloseListener()
    });
</script>
  
<style global>
  @tailwind base;
  @tailwind components;
  @tailwind utilities;
</style>
  
<main class="min-h-screen -m-2 flex">
    <Router url="{url}">
        <div class="bg-gray-900 w-1/6 flex-grow">
            <div class="h-12 bg-lime-500 text-2xl flex items-center justify-center">DietPi Dashboard</div>
            <span on:click={pollServer}><NavbarLink icon={faTachometerAlt} to="/">Statistics</NavbarLink></span>
            <span on:click={pollServer}><NavbarLink icon={faMicrochip} to="process">Processes</NavbarLink></span>
            <span on:click={pollServer}><NavbarLink icon={faDatabase} to="software">Software</NavbarLink></span>
        </div>
        <div class="w-5/6 flex flex-col flex-grow min-h-full">
            <header class="bg-lime-400 h-12 flex justify-center items-center">
                <a href="https://dietpi.com" target="_blank"><img src="/assets/dietpi.png" alt="DietPi logo" class="h-10"></a>
            </header>
                <div class="bg-gray-100 flex-grow p-6">
                    {#if shown}
                        <Route path="process"><Process {socketData} /></Route>
                        <Route path="/"><Home {socketData}/></Route>
                        <Route path="software"><Software {socketData} {socket} /></Route>
                    {:else}
                        <h3>Connecting to API...</h3>
                    {/if}
                </div>
            <footer class="border-t bg-gray-200 border-gray-300 h-16 flex flex-col justify-center items-center">
                DietPi-Dashboard created by ravenclaw900
                <a href="https://github.com/ravenclaw900/DietPi-Dashboard" target="_blank"><Fa icon={faGithub} class="hover:opacity-75" size="2x" /></a>
            </footer>
        </div>
    </Router>
</main>