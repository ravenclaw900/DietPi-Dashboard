<script lang="ts">
    import { Terminal } from "xterm";
    import { AttachAddon } from "xterm-addon-attach";
    import { FitAddon, type ITerminalDimensions } from "xterm-addon-fit";

    import { onDestroy } from "svelte";

    import "xterm/css/xterm.css";

    export let loginDialog: boolean;
    export let node: string;

    let termDiv: HTMLDivElement;

    let proto = window.location.protocol == "https:" ? "wss" : "ws";
    const socket = new WebSocket(`${proto}://${node}/ws/term`);

    const attachAddon = new AttachAddon(socket);

    const fitAddon = new FitAddon();

    let terminal = new Terminal();
    terminal.loadAddon(attachAddon);
    terminal.loadAddon(fitAddon);

    const sendSize = (e: ITerminalDimensions) => {
        let size = JSON.stringify({ cols: e.cols, rows: e.rows + 1 });
        socket.send(`size${size}`);
    };

    terminal.onResize((e) => sendSize(e));

    window.onresize = () => {
        fitAddon.fit();
    };

    socket.onopen = () => {
        let obj = JSON.parse(localStorage.getItem("tokens"));
        if (obj != null && obj[node] != null) {
            socket.send(`token${obj[node]}`);
        }
        terminal.open(termDiv);
        fitAddon.fit();
        sendSize({ cols: terminal.cols, rows: terminal.rows });
    };

    onDestroy(() => socket.close(1000));
</script>

<div bind:this={termDiv} class="h-full{loginDialog ? ' hidden' : ''}" />
