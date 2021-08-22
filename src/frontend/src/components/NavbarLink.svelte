<script>
    import {Link} from "svelte-routing"
    export let to;
</script>

<Link {to} class="text-gray-400 no-underline hover:bg-gray-800 flex items-center pl-2 h-10 text-xl font-sans"><slot /></Link>