import websocketStore from "svelte-websocket-store";

export const myStore = websocketStore("ws://mydomain.com/ws1", {}, []);
