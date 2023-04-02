import type {
    browserPage,
    managementPage,
    processPage,
    servicesPage,
    socketData,
    softwarePage,
    statisticsPage,
} from "./types";
import { derived } from "svelte/store";

type request = { page: string } | { cmd: string; args?: string[] } | { token: string };

// Inspired by the svelte-websocket-store package
function createWebsocketStore(host: string) {
    let socket: WebSocket | undefined;
    const subscribers: Set<(value: socketData | { dataKind: "NOTCONNECTED" }) => void> =
        new Set();
    let reopenTimeout: ReturnType<typeof setTimeout> | undefined;
    let retryCount = 0;
    let proto = window.location.protocol === "http:" ? "ws" : "wss";
    let lastValue: socketData | undefined;

    function subscribe(
        callback: (value: socketData | { dataKind: "NOTCONNECTED" }) => void
    ): () => void {
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
            console.error("WebSocket is not connected");
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
            socket.removeEventListener("open", onOpen);
            socket.removeEventListener("close", onClose);
            socket.removeEventListener("message", onMessage);
            socket.close();
        }

        if (newHost) {
            host = newHost;
        }

        let newUrl = `${proto}://${host}/ws`;

        socket = new WebSocket(newUrl);

        socket.addEventListener("open", onOpen);
        socket.addEventListener("close", onClose);
        socket.addEventListener("message", onMessage);
    }

    function onOpen() {
        console.log("WebSocket connected");
        retryCount = 0;
    }

    function onClose() {
        console.log("WebSocket disconnected");
        if (!reopenTimeout) {
            const delay = Math.pow(2, retryCount) * 1000;
            retryCount++;
            scheduleReconnect(delay);
        }
    }

    function onMessage(event: MessageEvent) {
        const data = JSON.parse(event.data);
        lastValue = data;
        subscribers.forEach(callback => {
            callback(data);
        });
    }

    reopen();

    return { subscribe, send, reopen };
}

export const socket = createWebsocketStore(window.location.host);

// Creates a derived store that filters the WebSocket data by `dataKind`
function createStore<T extends socketData>(defaultValue: T) {
    const store = derived(socket, $socket => {
        let lastValue = defaultValue;
        if ($socket.dataKind == defaultValue.dataKind) {
            lastValue = $socket as T;
        }
        return lastValue;
    });
    return { ...store, send: socket.send };
}

// Gives each page a separate store that either holds a default value or the last value given by the websocket
export const statisticsStore = createStore<statisticsPage>({
    dataKind: "STATISTIC",
    cpu: 0,
    disk: { used: 0, total: 0, percent: 0 },
    ram: { used: 0, total: 0, percent: 0 },
    network: { sent: 0, received: 0 },
    swap: { used: 0, total: 0, percent: 0 },
    // TODO: modify so that { available: false } and { available: true, celsius: 0, fahrenheit: 32 }
    // is possible, there shouldn't have to be values if it isn't available
    temp: { celsius: 0, fahrenheit: 0, available: false },
});

export const processStore = createStore<processPage>({
    dataKind: "PROCESS",
    processes: [],
});

export const serviceStore = createStore<servicesPage>({
    dataKind: "SERVICE",
    services: [],
});

export const softwareStore = createStore<softwarePage>({
    dataKind: "SOFTWARE",
    installed: [],
    uninstalled: [],
});

export const managementStore = createStore<managementPage>({
    dataKind: "MANAGEMENT",
    hostname: "unknown",
    uptime: 0,
    arch: "unknown",
    kernel: "unknown",
    dp_version: "unknown",
    packages: 0,
    upgrades: 0,
    nic: "unknown",
    ip: "127.0.0.1",
});

export const browserStore = createStore<browserPage>({
    dataKind: "BROWSER",
    contents: [],
    textdata: "",
});
