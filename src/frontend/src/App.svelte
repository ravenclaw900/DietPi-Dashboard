<script>
    import { Router, Route, Link } from "svelte-routing";
    import Home from "./pages/Home.svelte";
    import Test from "./pages/Test.svelte";
    export let url = ""; //This property is necessary declare to avoid ignore the Router
</script>
  
<style>
main {
    text-align: center;
    padding: 1em;
    max-width: 240px;
    margin: 0 auto;
}
</style>
  
<main>
    <Router url="{url}">
        <nav>
            <Link to="/">Home</Link>
            <Link to="test">Test</Link>
        </nav>
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