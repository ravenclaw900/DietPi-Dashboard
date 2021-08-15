<script>
    import { Router, Route } from 'svelte-routing';
    import Home from '../pages/Home.svelte';
    import Test from '../pages/Test.svelte';
  
    export let url = '';
</script>
  
<Router {url}>
    <div>
        <Route path="test" component={Test} />
        <Route path="/" component={Home} />
    </div>
</Router>