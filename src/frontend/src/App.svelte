<script>
    import { Router, Route } from "svelte-routing";
    import { onMount } from 'svelte';
    import Home from "./pages/Home.svelte";
    import Test from "./pages/Test.svelte";
    import NavbarLink from "./components/NavbarLink.svelte"
    let url = "";

    let socket
    let data = {}
    let shown = false
    const socketMessageListener = (e) => {
        data = JSON.parse(e.data)
    };
    const socketOpenListener = (e) => {
        console.log('Connected')
        shown = true
    };
    const socketErrorListener = (e) => {
        console.error(e)
    }
    const socketCloseListener = (e) => {
        if (socket) {
            console.log('Disconnected.');
        }
        socket = new WebSocket(`ws://${window.location.hostname}:8080/ws`)
        socket.onopen = socketOpenListener
        socket.onmessage = socketMessageListener
        socket.onclose = socketCloseListener
        socket.onerror = socketErrorListener
    };
    
    onMount(socketCloseListener);
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
            <NavbarLink to="/" class="text-gray-400 no-underline hover:bg-gray-800 flex items-center pl-2 h-10 text-xl font-sans">Home</NavbarLink>
            <NavbarLink to="test" class="text-gray-400 no-underline hover:bg-gray-800 flex items-center pl-2 h-10 text-xl font-sans">Test</NavbarLink>
        </div>
        <div class="w-5/6 flex flex-col flex-grow min-h-full">
            <header class="bg-lime-400 h-12">Test</header>
                <div class="bg-gray-100 flex-grow p-6">
                    {#if shown}
                        <Route path="test" component="{Test}" />
                        <Route path="/"><Home {socket} {data}/></Route>
                    {:else}
                        <h3>Connecting to API...</h3>
                    {/if}
                </div>
            <footer class="border-t bg-gray-200 border-gray-300 h-16 flex flex-col justify-center items-center">
                DietPi-Dashboard created by ravenclaw900
                <a href="https://github.com/ravenclaw900/DietPi-Dashboard" target="_blank"><img src="/assets/github.svg" class="h-7 hover:opacity-75" alt="GitHub mark"></a>
            </footer>
        </div>
    </Router>
</main>