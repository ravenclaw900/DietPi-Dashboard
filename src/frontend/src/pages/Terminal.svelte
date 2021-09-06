<script>
    import { Terminal } from "xterm";
    import { AttachAddon } from "xterm-addon-attach";
    import { FitAddon } from "xterm-addon-fit";

    import { onDestroy } from "svelte";

    let termDiv;

    const socket = new WebSocket(
        `ws://${window.location.hostname}:8080/ws/term`
    );
    const attachAddon = new AttachAddon(socket);

    const fitAddon = new FitAddon();

    let terminal = new Terminal();
    terminal.loadAddon(attachAddon);
    terminal.loadAddon(fitAddon);

    const sendSize = (e) => {
        let size = JSON.stringify({ cols: e.cols, rows: e.rows + 1 });
        socket.send("size" + size);
    };

    terminal.onResize((e) => sendSize(e));

    window.onresize = () => {
        fitAddon.fit();
    };

    socket.onopen = () => {
        terminal.open(termDiv);
        fitAddon.fit();
        sendSize({ cols: terminal.cols, rows: terminal.rows });
    };

    onDestroy(() => socket.close(1000));
</script>

<div bind:this={termDiv} class="h-full" />
