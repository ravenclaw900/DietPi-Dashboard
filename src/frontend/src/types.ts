interface socketData {
    // Statistics page
    cpu: number;
    ram: usage;
    swap: usage;
    disk: usage;
    network: net;
    // Software page
    uninstalled: software[];
    installed: software[];
    response: string;
    // Process page
    processes: processes[];
    // Services page
    services: services[];
    // File browser page
    contents: browser[];
    textdata: string;
    // Management page
    hostname: string;
    uptime: number;
    arch: string;
    kernel: string;
    dp_version: string;
    packages: number;
    upgrades: number;
    nic: string;
    ip: string;
    // Global
    update: string;
    login: boolean;
    error: boolean;
    nodes: string[];
    version: string;
    update_check: boolean;
}

interface software {
    id: number;
    name: string;
    description: string;
    dependencies: string;
    docs: string;
}

interface processes {
    pid: number;
    name: string;
    cpu: number;
    ram: number;
    status: string;
}

interface services {
    name: string;
    status: string;
    log: string;
    start: string;
}

interface browser {
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

// 'browser' required for selected path in file browser
export type { socketData, browser };