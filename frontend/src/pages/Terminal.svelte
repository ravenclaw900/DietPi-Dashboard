<script lang="ts">
    import { Terminal } from "xterm";
    import { AttachAddon } from "xterm-addon-attach";
    import { FitAddon, type ITerminalDimensions } from "xterm-addon-fit";
    import "xterm/css/xterm.css";

    import { onDestroy } from "svelte";

    export let node: string;
    export let token: string;

    let termDiv: HTMLDivElement;

    let proto = window.location.protocol === "https:" ? "wss" : "ws";
    let socket = new WebSocket(
        `${proto}://${node}/ws/term${token ? `?token=${token}` : ""}`
    );

    $: token,
        node,
        (socket.onopen = () => {}),
        ((socket = new WebSocket(
            `${proto}://${node}/ws/term${token ? `?token=${token}` : ""}`
        )),
        (socket.onopen = socketOpen));

    const fitAddon = new FitAddon();

    let terminal = new Terminal();
    terminal.loadAddon(fitAddon);

    const sendSize = (e: ITerminalDimensions) => {
        let size = JSON.stringify({ cols: e.cols, rows: e.rows + 1 });
        socket.send(`size${size}`);
    };

    terminal.onResize(e => sendSize(e));

    window.onresize = () => {
        fitAddon.fit();
    };

    let socketOpen = () => {
        termDiv.replaceChildren();
        const attachAddon = new AttachAddon(socket);
        terminal.loadAddon(attachAddon);
        terminal.open(termDiv);
        fitAddon.fit();

        sendSize({ cols: terminal.cols, rows: terminal.rows });
    };

    onDestroy(() => socket.close(1000));
</script>

<div bind:this={termDiv} class="h-full" />
