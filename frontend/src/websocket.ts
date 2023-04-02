import type { socketData } from "./types";

type request =
    | { page: string }
    | { cmd: string; args?: string[] }
    | { token: string };

// Inspired by the svelte-websocket-store package
function createWebsocketStore(host: string) {
    let socket: WebSocket | undefined;
    const subscribers: Set<(value: socketData | {dataKind: "NOTCONNECTED"}) => void> = new Set();
    let reopenTimeout: ReturnType<typeof setTimeout> | undefined;
    let retryCount = 0;
    let proto = window.location.protocol === "http:" ? "ws" : "wss";
    let lastValue: socketData | undefined;

    function subscribe(callback: (value: socketData | {dataKind: "NOTCONNECTED"}) => void): () => void {
        console.debug("new subscriber");
        subscribers.add(callback);
        if (lastValue) {
            callback(lastValue);
        }
        return () => {
            subscribers.delete(callback);
        };
    }

    function send(message: request): void {
        if (socket && socket.readyState === WebSocket.OPEN) {
            socket.send(JSON.stringify(message));
        } else {
            console.error('WebSocket is not connected');
        }
    }

    function scheduleReconnect(delay: number) {
        console.log(`WebSocket connection lost. Retrying in ${delay / 1000} seconds...`);
        reopenTimeout = setTimeout(() => {
            reopen();
        }, delay);
    }

    function reopen(newHost?: string): void {
        clearTimeout(reopenTimeout);

        if (socket) {
            socket.removeEventListener('open', onOpen);
            socket.removeEventListener('close', onClose);
            socket.removeEventListener('message', onMessage);
            socket.close();
        }

        if (newHost) {
            host = newHost;
        }

        let newUrl = `${proto}://${host}/ws`;

        socket = new WebSocket(newUrl);

        socket.addEventListener('open', onOpen);
        socket.addEventListener('close', onClose);
        socket.addEventListener('message', onMessage);
    }

    function onOpen() {
        console.log('WebSocket connected');
        retryCount = 0;
    }

    function onClose() {
        console.log('WebSocket disconnected');
        if (!reopenTimeout) {
            const delay = Math.pow(2, retryCount) * 1000;
            retryCount++;
            scheduleReconnect(delay);
        }
    }

    function onMessage(event: MessageEvent) {
        const data = JSON.parse(event.data);
        lastValue = data;
        subscribers.forEach((callback) => {
            callback(data);
        });
    }

    reopen();

    return { subscribe, send, reopen };
}


export const socket = createWebsocketStore(window.location.host);
