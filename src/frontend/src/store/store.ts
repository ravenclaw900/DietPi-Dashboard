import { readable, derived } from 'svelte/store';

export const socket = readable(undefined, set => {
    set(new WebSocket(`ws://${window.location.hostname}:8080/ws`))
});

export let data = derived(socket, ($socket, set) => {
    $socket.addEventListener('open', (e) => {
        console.log("Connected")
    })
    $socket.addEventListener('error', (e) => {
        console.error(e)
    })
    $socket.addEventListener('close', (e) => {
        console.log("Disconnected")
    })
    $socket.addEventListener('message', (e) => {
        set(JSON.parse(e.data))
    });
})
