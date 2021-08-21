<script>
    import { Router, Route, Link } from "svelte-routing";
    import Home from "./pages/Home.svelte";
    import Test from "./pages/Test.svelte";
    export let url = "";

    let socket;
    const socketMessageListener = (e) => {
        console.log(e.data)
    }
    const socketOpenListener = (e) => {
        console.log('Connected')
    }
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
    window.addEventListener('load', (event) => {
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
            <Link to="/" class="text-gray-400 no-underline hover:bg-gray-800 flex items-center pl-2 h-10 text-xl font-sans">Home</Link>
            <Link to="test" class="text-gray-400 no-underline hover:bg-gray-800 flex items-center pl-2 h-10 text-xl font-sans">Test</Link>
        </div>
        <div class="w-5/6 flex flex-col flex-grow min-h-full">
            <header class="bg-lime-400 h-12">Test</header>
            <div class="bg-gray-100 flex-grow">
                <Route path="test" component="{Test}" /> 
                <Route path="/" component="{Home}" />
            </div>
            <footer class="border-t bg-gray-200 border-gray-300 h-10">DietPi Dashboard</footer>
        </div>
    </Router>
</main>