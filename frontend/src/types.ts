type socketData =
    | statisticsPage
    | softwarePage
    | processPage
    | servicesPage
    | browserPage
    | managementPage
    | globalSettings
    | reauthenticate;

export enum MessageKind {
    Statistics,
    Software,
    Process,
    Service,
    Browser,
    Management,
    Global,
    Reauth,
}

interface statisticsPage {
    kind: MessageKind.Statistics;
    cpu: number;
    ram: usage;
    swap: usage;
    disk: usage;
    network: net;
    temp: temp;
}

interface softwarePage {
    kind: MessageKind.Software;
    uninstalled: softwareItem[];
    installed: softwareItem[];
    response: string;
}

interface processPage {
    kind: MessageKind.Process;
    processes: processItem[];
}

interface servicesPage {
    kind: MessageKind.Service;
    services: serviceItem[];
}

interface browserPage {
    kind: MessageKind.Browser;
    contents: browserItem[];
    textdata: string;
}

interface managementPage {
    kind: MessageKind.Management;
    hostname: string;
    uptime: number;
    arch: string;
    kernel: string;
    dp_version: string;
    packages: number;
    upgrades: number;
    nic: string;
    ip: string;
}

interface globalSettings {
    kind: MessageKind.Global;
    update: string;
    login: boolean;
    nodes: string[];
    version: string;
    update_check: boolean;
    temp_unit: "fahrenheit" | "celsius";
}

interface reauthenticate {
    kind: MessageKind.Reauth;
    reauth: true;
}

interface softwareItem {
    id: number;
    name: string;
    description: string;
    dependencies: string;
    docs: string;
}

interface processItem {
    pid: number;
    name: string;
    cpu: number;
    ram: number;
    status: string;
}

interface serviceItem {
    name: string;
    status: string;
    log: string;
    start: string;
}

interface browserItem {
    name: string;
    path: string;
    prettytype: string;
    maintype: string;
    subtype: string;
    size: number;
}

interface usage {
    used: number;
    total: number;
    percent: number;
}

interface net {
    sent: number;
    received: number;
}

interface temp {
    available: boolean;
    celsius: number;
    fahrenheit: number;
}

// 'browserItem' required for selected path in file browser
export type {
    socketData,
    statisticsPage,
    softwarePage,
    processPage,
    servicesPage,
    browserPage,
    managementPage,
    globalSettings,
    browserItem,
};
