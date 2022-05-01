# Contributing guidelines

First off, thanks for taking time to contribute to DietPi Dashboard! Any contribution is welcome, from spelling and grammer fixes, to implementing a new feature, or even just reporting bugs.

## Things to know for bug reporting

### Before reporting

- Please make sure that your dashboard is completely up-to-date with the latest stable or nightly version.
- Check for similar bugs that have already been reported
- See if the problem is actually generally with DietPi, if so report at [the DietPi repository](https://github.com/MichaIng/DietPi)

### While reporting

Please follow the issue template, and give all information requested. Make sure to use a clear and concise title, and feel free to use screenshots, GIFs, and videos.

## Things to know for development

### Technologies/Languages used
- [Rust](https://rust-lang.org): Backend
- [Yarn](https://yarnpkg.com): Node package manager
- [Svelte](https://svelte.dev): Frontend
- [TypeScript](https://www.typescriptlang.org/): Frontend scripts

### Directory structure

Personally, this has turned me off from contributing to projects before, so here's how the source code is organized as of v0.6.0:

```
src
├── backend
│   ├── Cargo.lock (Rust lockfile)
│   ├── Cargo.toml (Rust config file)
│   └── src
│       ├── config.rs (Configuration file handling)
│       ├── main.rs (Main file, starts webserver)
│       ├── page_handlers.rs (Backend handlers for pages)
│       ├── shared.rs (Shared types)
│       ├── socket_handlers.rs (Backend websocket handlers)
│       └── systemdata.rs (Gets system data)
└── frontend
    ├── index.html
    ├── package.json (Node config file)
    ├── public
    │   └── favicon.png
    ├── src
    │   ├── App.svelte (Main page file, contains header, footer, and navigation)
    │   ├── assets
    │   │   ├── dietpi.png (DietPi logo)
    │   │   └── github-mark.svg (GitHub logo)
    │   ├── components
    │   │   ├── Card.svelte (Cards used on home and management pages)
    │   │   └── NavbarLink.svelte (Sidebar links)
    │   ├── main.ts
    │   └── pages (self-explanatory)
    │       ├── FileBrowser.svelte
    │       ├── Home.svelte
    │       ├── Management.svelte
    │       ├── Process.svelte
    │       ├── Service.svelte
    │       ├── Software.svelte
    │       └── Terminal.svelte
    ├── svelte.config.js (Svelte config)
    ├── tsconfig.json (TypeScript config)
    ├── vite.config.js (Vite config)
    ├── windi.config.ts (WindiCSS config)
    └── yarn.lock (Yarn lockfile)
```

### Style guide
- Frontend: [Prettier](https://prettier.io/)
- Backend: rustfmt
- Commits: [Conventional Commits](https://www.conventionalcommits.org/)
