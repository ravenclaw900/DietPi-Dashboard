<script lang="ts">
    import { Router, Route } from "svelte-routing";
    import { onMount } from "svelte";
    import Home from "./pages/Home.svelte";
    import Process from "./pages/Process.svelte";
    import Software from "./pages/Software.svelte";
    import Terminal from "./pages/Terminal.svelte";
    import NavbarLink from "./components/NavbarLink.svelte";
    import Fa from "svelte-fa";
    import { faGithub } from "@fortawesome/free-brands-svg-icons";
    import {
        faTachometerAlt,
        faMicrochip,
        faDatabase,
        faTerminal,
    } from "@fortawesome/free-solid-svg-icons";
    import { version } from "./version.js";

    let url = "";

    let socket;
    let socketData = {};
    let shown = false;
    const socketMessageListener = (e) => {
        socketData = JSON.parse(e.data);
    };
    const socketOpenListener = () => {
        console.log("Connected");
        shown = true;
    };
    const socketErrorListener = (e) => {
        console.error(e);
    };
    const socketCloseListener = () => {
        if (socket) {
            console.log("Disconnected");
        }
        socket = new WebSocket(`ws://${window.location.hostname}:8080/ws`);
        socket.onopen = socketOpenListener;
        socket.onmessage = socketMessageListener;
        socket.onclose = socketCloseListener;
        socket.onerror = socketErrorListener;
    };

    function pollServer() {
        socket.send(JSON.stringify({ page: window.location.pathname }));
    }

    $: shown && pollServer();

    onMount(() => {
        socketCloseListener();
    });
</script>

<main class="min-h-screen -m-2 flex">
    <Router {url}>
        <div class="bg-gray-900 w-1/6 flex-grow">
            <div
                class="h-12 bg-dplime-dark text-2xl flex items-center justify-center"
            >
                DietPi Dashboard
            </div>
            <span on:click={pollServer}
                ><NavbarLink icon={faTachometerAlt} to="/"
                    >Statistics</NavbarLink
                ></span
            >
            <span on:click={pollServer}
                ><NavbarLink icon={faMicrochip} to="process"
                    >Processes</NavbarLink
                ></span
            >
            <span on:click={pollServer}
                ><NavbarLink icon={faDatabase} to="software"
                    >Software</NavbarLink
                ></span
            >
            <NavbarLink icon={faTerminal} to="terminal">Terminal</NavbarLink>
        </div>
        <div class="w-5/6 flex flex-col flex-grow min-h-full">
            <header class="bg-dplime h-12 flex justify-center items-center">
                <a href="https://dietpi.com" target="_blank"
                    ><img
                        src="/assets/dietpi.png"
                        alt="DietPi logo"
                        class="h-10"
                    /></a
                >
            </header>
            <div class="bg-gray-100 flex-grow p-6">
                {#if shown}
                    <Route path="process"><Process {socketData} /></Route>
                    <Route path="/"><Home {socketData} /></Route>
                    <Route path="software"
                        ><Software {socketData} {socket} /></Route
                    >
                    <Route path="terminal"><Terminal /></Route>
                {:else}
                    <h3>Connecting to API...</h3>
                {/if}
            </div>
            <footer
                class="border-t bg-gray-200 border-gray-300 h-16 flex flex-col justify-center items-center"
            >
                <div>
                    DietPi-Dashboard <a
                        class="text-blue-500"
                        href="https://github.com/ravenclaw900/DietPi-Dashboard/releases/tag/v{version}"
                        target="_blank">v{version}</a
                    > created by ravenclaw900
                </div>
                <a
                    href="https://github.com/ravenclaw900/DietPi-Dashboard"
                    target="_blank"
                    ><Fa
                        icon={faGithub}
                        class="hover:opacity-75"
                        size="2x"
                    /></a
                >
            </footer>
        </div>
    </Router>
</main>

<style global>
    @tailwind base;
    @tailwind components;
    @tailwind utilities;

    /**
 * Copyright (c) 2014 The xterm.js authors. All rights reserved.
 * Copyright (c) 2012-2013, Christopher Jeffrey (MIT License)
 * https://github.com/chjj/term.js
 * @license MIT
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
 * THE SOFTWARE.
 *
 * Originally forked from (with the author's permission):
 *   Fabrice Bellard's javascript vt100 for jslinux:
 *   http://bellard.org/jslinux/
 *   Copyright (c) 2011 Fabrice Bellard
 *   The original design remains. The terminal itself
 *   has been extended to include xterm CSI codes, among
 *   other features.
 */

    /**
 *  Default styles for xterm.js
 */

    .xterm {
        position: relative;
        user-select: none;
        -ms-user-select: none;
        -webkit-user-select: none;
    }

    .xterm.focus,
    .xterm:focus {
        outline: none;
    }

    .xterm .xterm-helpers {
        position: absolute;
        top: 0;
        /**
     * The z-index of the helpers must be higher than the canvases in order for
     * IMEs to appear on top.
     */
        z-index: 5;
    }

    .xterm .xterm-helper-textarea {
        padding: 0;
        border: 0;
        margin: 0;
        /* Move textarea out of the screen to the far left, so that the cursor is not visible */
        position: absolute;
        opacity: 0;
        left: -9999em;
        top: 0;
        width: 0;
        height: 0;
        z-index: -5;
        /** Prevent wrapping so the IME appears against the textarea at the correct position */
        white-space: nowrap;
        overflow: hidden;
        resize: none;
    }

    .xterm .composition-view {
        /* TODO: Composition position got messed up somewhere */
        background: #000;
        color: #fff;
        display: none;
        position: absolute;
        white-space: nowrap;
        z-index: 1;
    }

    .xterm .composition-view.active {
        display: block;
    }

    .xterm .xterm-viewport {
        /* On OS X this is required in order for the scroll bar to appear fully opaque */
        background-color: #000;
        overflow-y: scroll;
        cursor: default;
        position: absolute;
        right: 0;
        left: 0;
        top: 0;
        bottom: 0;
    }

    .xterm .xterm-screen {
        position: relative;
    }

    .xterm .xterm-screen canvas {
        position: absolute;
        left: 0;
        top: 0;
    }

    .xterm .xterm-scroll-area {
        visibility: hidden;
    }

    .xterm-char-measure-element {
        display: inline-block;
        visibility: hidden;
        position: absolute;
        top: 0;
        left: -9999em;
        line-height: normal;
    }

    .xterm {
        cursor: text;
    }

    .xterm.enable-mouse-events {
        /* When mouse events are enabled (eg. tmux), revert to the standard pointer cursor */
        cursor: default;
    }

    .xterm.xterm-cursor-pointer {
        cursor: pointer;
    }

    .xterm.column-select.focus {
        /* Column selection mode */
        cursor: crosshair;
    }

    .xterm .xterm-accessibility,
    .xterm .xterm-message {
        position: absolute;
        left: 0;
        top: 0;
        bottom: 0;
        right: 0;
        z-index: 10;
        color: transparent;
    }

    .xterm .live-region {
        position: absolute;
        left: -9999px;
        width: 1px;
        height: 1px;
        overflow: hidden;
    }

    .xterm-dim {
        opacity: 0.5;
    }

    .xterm-underline {
        text-decoration: underline;
    }
</style>
