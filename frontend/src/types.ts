type socketData =
    | statisticsPage
    | softwarePage
    | processPage
    | servicesPage
    | browserPage
    | managementPage
    | globalSettings
    | reauthenticate;

interface statisticsPage {
    dataKind: "STATISTIC";
    cpu: number;
    ram: usage;
    swap: usage;
    disk: usage;
    network: net;
    temp: temp;
}

interface softwarePage {
    dataKind: "SOFTWARE";
    uninstalled: softwareItem[];
    installed: softwareItem[];
    response?: string;
}

interface processPage {
    dataKind: "PROCESS";
    processes: processItem[];
}

interface servicesPage {
    dataKind: "SERVICE";
    services: serviceItem[];
}

interface browserPage {
    dataKind: "BROWSER";
    contents: browserItem[];
    textdata: string;
}

interface managementPage {
    dataKind: "MANAGEMENT";
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
    dataKind: "GLOBAL";
    update: string;
    login: boolean;
    nodes: string[];
    version: string;
    update_check: boolean;
    temp_unit: "fahrenheit" | "celsius";
}

interface reauthenticate {
    dataKind: "REAUTH";
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

// 'browserItem' required for selected path in file browser, 'processItem' required for sorting
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
    processItem,
};
