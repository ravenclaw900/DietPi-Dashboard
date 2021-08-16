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
        <div class="float-left bg-blue-400 h-full">
            <Link to="/">Home</Link>
            <Link to="test">Test</Link>
        </div>
        <div>
            <Route path="test" component="{Test}" /> 
            <!--for now the router just support case sensitive,
            one workaround colud be add two time the route
            Example.
            <Route path="About" component="{About}" /> 
            -->
            <Route path="/" component="{Home}"></Route>
        </div>
    </Router>
</main>