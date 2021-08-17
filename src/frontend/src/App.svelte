<script>
    import { Router, Route, Link } from "svelte-routing";
    import Home from "./pages/Home.svelte";
    import Test from "./pages/Test.svelte";
    export let url = "";
</script>
  
<style global>
  @tailwind base;
  @tailwind components;
  @tailwind utilities;
</style>
  
<main>
    <Router url="{url}">
        <div class="float-left bg-black h-96 w-1/6 mr-2 -ml-2 h-screen">
            <div class="h-12 bg-gray-500 text-2xl flex items-center justify-center">DietPi Dashboard</div>
            <Link to="/" class="text-gray-400 no-underline hover:bg-gray-800 h-10 text-xl flex items-center justify-center font-sans">Home</Link>
            <Link to="test" class="text-gray-400 no-underline hover:bg-gray-800 h-10 text-xl flex items-center justify-center font-sans">Test</Link>
        </div>
        <div class="bg-gray-400 h-12 -mt-2 -mr-2"></div>
        <div>
            <Route path="test" component="{Test}" /> 
            <Route path="/" component="{Home}"></Route>
        </div>
    </Router>
</main>