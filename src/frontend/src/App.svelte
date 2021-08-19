<script>
    import { Router, Route, Link } from "svelte-routing";
    import Home from "./pages/Home.svelte";
    import Test from "./pages/Test.svelte";
    export let url = "";

    /**
    var socket;
    const socketMessageListener = (e) => {
        console.log(e.data);
    };
    const socketOpenListener = (e) => {
        console.log('Connected');
        socket.send(JSON.stringify({ message: "hello, server" }))
    };
    const socketErrorListener = (e) => {
        console.error(e);
    }
    const socketCloseListener = (e) => {
        if (socket) {
            console.log('Disconnected.');
        }
        socket = new WebSocket("ws://localhost:8080/ws");
        socket.onopen = socketOpenListener;
        socket.onmessage = socketMessageListener;
        socket.onclose = socketCloseListener;
        socket.onerror = socketErrorListener;
    };
    window.addEventListener('load', (event) => {
        socketCloseListener();
    });
    */
</script>
  
<style global>
  @tailwind base;
  @tailwind components;
  @tailwind utilities;
</style>
  
<main>
    <Router url="{url}">
        <div class="float-left bg-gray-900 h-96 w-1/6 mr-2 -ml-2 h-screen">
            <div class="h-12 bg-lime-500 text-2xl flex items-center justify-center">DietPi Dashboard</div>
            <Link to="/" class="text-gray-400 no-underline hover:bg-gray-800 flex items-center pl-2 h-10 text-xl font-sans">Home</Link>
            <Link to="test" class="text-gray-400 no-underline hover:bg-gray-800 flex items-center pl-2 h-10 text-xl font-sans">Test</Link>
        </div>
        <div class="bg-lime-400 h-12 -mt-2 -mr-2"></div>
        <div>
            <Route path="test" component="{Test}" /> 
            <Route path="/" component="{Home}"></Route>
        </div>
    </Router>
</main>