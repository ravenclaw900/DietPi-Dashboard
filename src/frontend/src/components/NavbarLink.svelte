<script>
    import { Link } from "svelte-routing";
    import Fa from "svelte-fa";
    export let to;
    export let icon;
</script>

<Link
    {to}
    class="text-gray-400 no-underline hover:bg-gray-800 flex items-center h-10 text-xl font-sans whitespace-nowrap"
    ><Fa {icon} class="mr-2 ml-1" size="sm" /><slot /></Link
>
