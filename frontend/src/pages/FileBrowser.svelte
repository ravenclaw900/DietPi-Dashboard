<script lang="ts">
    import microlight from "microlight";
    import prettyBytes from "pretty-bytes";

    import { browserStore } from "../websocket";

    import type { browserItem } from "../types";

    let selPath: browserItem = {
        name: "",
        path: "",
        maintype: "",
        subtype: "",
        prettytype: "",
        size: 0,
    };

    export let node: string;
    export let login: boolean;
    export let token: string;

    let fileDialog: HTMLInputElement;
    let fileText: HTMLTextAreaElement;
    let fileDiv: HTMLDivElement;
    let binData: BlobPart[] = [];
    let pathArray: string[];
    let fileDataSet = false;
    let showHidden = false;
    let highlighting = false;
    let downloading = false;
    let saved = true;
    let maxSlices = 0;
    let currentSlices = 0;
    let fileData = "";
    // TODO: better solution than just assuming dashboard is being run by root
    let currentPath = "/root";
    let binURL = "";

    const fileSocket = new WebSocket(
        `${window.location.protocol === "https:" ? "wss" : "ws"}://${node}/ws/file${
            login ? `?token=${token}` : ""
        }`
    );
    fileSocket.onmessage = (e: MessageEvent) => {
        if (typeof e.data === "string") {
            try {
                let msg = JSON.parse(e.data);
                if (msg.finished) {
                    sendCmd(`${currentPath}`, "cd");
                } else if (msg.size) {
                    maxSlices = msg.size;
                }
            } catch {
                fileData = e.data;
                fileDataSet = true;
            }
        } else {
            currentSlices += 1;
            binData.push(e.data);
            if (currentSlices === maxSlices) {
                binURL = URL.createObjectURL(new Blob(binData));
                maxSlices = 0;
                currentSlices = 0;
                binData = [];
            }
        }
    };

    // Skip first array element (empty string)
    $: pathArray = currentPath.split("/").slice(1);
    // Set innerHTML manually to avoid issues with highlighting
    $: fileDiv !== undefined &&
        (fileDiv.innerHTML = (
            fileData[fileData.length - 1] === "\n" ? fileData + " " : fileData
        )
            .replace(new RegExp("&", "g"), "&amp;")
            .replace(new RegExp("<", "g"), "&lt;")),
        microlight.reset();
    $: $browserStore.contents &&
        $browserStore.contents.sort((a, b) => {
            return a.name < b.name ? -1 : 1;
        });

    function sendCmd(path: string, cmd: string) {
        if (cmd === "rm" || cmd === "rmdir" || cmd === "cd") {
            selPath = {
                name: "",
                path: "",
                maintype: "",
                subtype: "",
                prettytype: "",
                size: 0,
            };
        }
        browserStore.send({ cmd, args: [path] });
        fileDataSet = false;
        if (downloading) {
            downloading = false;
            URL.revokeObjectURL(binURL);
            binURL = "";
        }
    }

    function fileSend(path: string, cmd: string, arg: string) {
        fileSocket.send(
            JSON.stringify({
                cmd,
                path,
                arg,
            })
        );
    }

    function rename(oldname: string, newname: string) {
        selPath = {
            name: "",
            path: "",
            maintype: "",
            subtype: "",
            prettytype: "",
            size: 0,
        };
        browserStore.send({ cmd: "rename", args: [oldname, newname] });
    }

    function syncScroll() {
        fileDiv.scrollTop = fileText.scrollTop;
        fileDiv.scrollLeft = fileText.scrollLeft;
    }

    function checkTab(event: KeyboardEvent) {
        if (event.key === "Tab") {
            event.preventDefault();

            let startPos = fileText.selectionStart;
            let endPos = fileText.selectionEnd;

            let tabAdded =
                fileData.substring(0, startPos) + "\t" + fileData.substring(endPos);

            fileText.value = tabAdded;
            fileData = tabAdded;
            fileText.selectionStart = fileText.selectionEnd = startPos + 1;
        }
    }

    function getIcon(maintype: string, subtype: string) {
        switch (maintype) {
            case "dir":
                return "i-fa-folder";
            case "image":
                return "i-fa-file-image";
            case "video":
                return "i-fa-file-video";
            case "audio":
                return "i-fa-file-audio";
            case "archive":
                if (subtype === "pdf") {
                    return "i-fa-file-pdf";
                }
                return "i-fa-file-archive";
            case "text":
                return "i-fa-file-lines";
            case "notafile":
                return "i-fa-cube";
            default:
                return "i-fa-file";
        }
    }

    function validateInput(name: string | null) {
        if (name) {
            for (let element of $browserStore.contents) {
                if (element.name === name) {
                    if (
                        confirm(
                            `This will overwrite the ${
                                element.maintype === "dir" ? "directory" : "file"
                            } ${name}${
                                element.maintype === "dir"
                                    ? " and delete everything in it"
                                    : ""
                            }. Are you sure you want to continue?`
                        )
                    ) {
                        sendCmd(
                            `${currentPath}/${name}`,
                            `rm${element.maintype === "dir" ? "dir" : ""}`
                        );
                        return true;
                    } else {
                        return false;
                    }
                }
            }
            return true;
        }
        return false;
    }
</script>

<main class="min-h-full">
    <div class="flex">
        <div class="w-11/12">
            <div class="mb-2 bg-white p dark:bg-black">
                <button
                    class="px-2 btn focus:outline-none"
                    on:click={() => {
                        if (
                            !saved &&
                            !confirm(
                                "You have not saved the file! Are you sure you want to continue?"
                            )
                        ) {
                            return;
                        } else {
                            saved = true;
                            sendCmd("/", "cd");
                            currentPath = "/";
                        }
                    }}>/</button
                >
                {#each pathArray as path}
                    {path !== pathArray[0] ? " /" : ""}
                    {#if path === pathArray[pathArray.length - 1]}
                        <div class="inline-block cursor-default">
                            {path}
                        </div>
                    {:else}
                        <button
                            class="btn focus:outline-none"
                            on:click={() => {
                                let fullPath = "";
                                for (let element of pathArray) {
                                    fullPath += "/" + element;
                                    if (element === path) {
                                        break;
                                    }
                                }
                                if (
                                    !saved &&
                                    !confirm(
                                        "You have not saved the file! Are you sure you want to continue?"
                                    )
                                ) {
                                    return;
                                } else {
                                    saved = true;
                                    sendCmd(fullPath, "cd");
                                    currentPath = fullPath;
                                }
                            }}>{path}</button
                        >
                    {/if}
                {/each}
            </div>
            {#if fileDataSet}
                <div class="flex">
                    <textarea
                        bind:value={fileData}
                        bind:this={fileText}
                        on:scroll={syncScroll}
                        on:keydown={checkTab}
                        on:input={() => {
                            saved = false;
                            if (fileText) {
                                fileText.style.height = "auto";
                                fileText.style.height = `${fileText.scrollHeight + 10}px`;
                                fileDiv.style.height = "auto";
                            }
                        }}
                        spellcheck="false"
                        class="overflow-y-hidden z-20 p-px w-full font-mono text-sm whitespace-pre resize-none tab-4 caret-black dark:caret-white focus:outline-none"
                        class:bg-transparent={highlighting}
                        class:text-transparent={highlighting}
                        class:dark:bg-black={!highlighting}
                    />
                    <div
                        bind:this={fileDiv}
                        class="w-full microlight font-mono whitespace-pre bg-white dark:bg-black text-sm z-10 tab-4 p-px -ml-[100%] overflow-y-hidden"
                        class:invisible={!highlighting}
                    />
                </div>
            {:else if downloading}
                {#if binURL !== ""}
                    <a
                        href={binURL}
                        target="_blank"
                        download={selPath.maintype === "dir"
                            ? `${selPath.name}.zip`
                            : selPath.name}>Click to Download</a
                    >
                {:else if maxSlices === 0}
                    <h2>
                        {selPath.maintype === "dir"
                            ? "Zipping directory"
                            : "Reading file"}...
                    </h2>
                {:else}
                    <h2>Receiving {currentSlices}MB out of {maxSlices}MB</h2>
                {/if}
            {:else if $browserStore.contents !== undefined}
                <table class="w-full bg-white table-fixed dark:bg-black min-w-50">
                    <tr>
                        <th class="px-2">Name</th>
                        <th class="px-2">Kind</th>
                        <th class="px-2">Size</th>
                    </tr>
                    {#each $browserStore.contents as contents}
                        <tr
                            class="select-none even:bg-white odd:bg-gray-200 dark:even:bg-black dark:odd:bg-gray-800"
                            class:!bg-dplime-dark={selPath.path == contents.path}
                            class:hidden={!showHidden && contents.name[0] === "."}
                            on:dblclick={() => {
                                switch (contents.maintype) {
                                    case "dir":
                                        sendCmd(contents.path, "cd");
                                        currentPath = contents.path;
                                        break;
                                    case "text":
                                        if (contents.size > 2 * 1000 * 1000) {
                                            if (
                                                confirm(
                                                    "Can't view files above 2MB, would you like to download instead?"
                                                )
                                            ) {
                                                fileSend(selPath.path, "dl", "");
                                                downloading = true;
                                            } else {
                                                break;
                                            }
                                        } else {
                                            fileSend(contents.path, "open", "");
                                        }
                                        currentPath = contents.path;
                                        break;
                                    case "notafile":
                                        alert("Cannot download special files");
                                        break;
                                    default:
                                        if (
                                            confirm(
                                                "Can't view that type of file, would you like to download instead?"
                                            )
                                        ) {
                                            fileSend(selPath.path, "dl", "");
                                            currentPath = selPath.path;
                                            downloading = true;
                                        }
                                }
                            }}
                            on:click={() => (selPath = contents)}
                        >
                            <td class="px-2"
                                ><div
                                    class="mr-2 inline-block {getIcon(
                                        contents.maintype,
                                        contents.subtype
                                    )}"
                                />
                                <span class="break-words">{contents.name}</span></td
                            >
                            <td class="px-2">{contents.prettytype}</td>
                            <td class="px-2"
                                >{contents.maintype === "dir"
                                    ? "-"
                                    : prettyBytes(contents.size)}</td
                            >
                        </tr>
                    {/each}
                </table>
            {/if}
        </div>
        <div
            class="flex sticky top-10 flex-col gap-2 justify-center items-center p-4 ml-4 bg-gray-300 min-w-16 dark:bg-gray-800 h-min text-2xl children:pointer-events-auto"
        >
            {#if fileDataSet}
                <button
                    title="Syntax Highlighting"
                    on:click={() => {
                        highlighting = !highlighting;
                    }}
                    class="i-fa-highlighter"
                    class:opacity-50={!highlighting}
                />
                <button
                    class="i-fa-floppy-disk"
                    on:click={() => {
                        fileSend(currentPath, "save", fileData), (saved = true);
                    }}
                />
            {:else if $browserStore.contents !== undefined}
                <button
                    class="i-fa-rotate"
                    title="Refresh"
                    on:click={() => {
                        sendCmd(`${currentPath}`, "cd");
                    }}
                />
                <button
                    title="{showHidden ? 'Hide' : 'Show'} Hidden Files"
                    class:i-fa-eye={showHidden}
                    class:i-fa-eye-slash={!showHidden}
                    on:click={() => {
                        showHidden = !showHidden;
                    }}
                />
                {#if currentPath !== "/"}
                    <button
                        class="i-fa-folder-plus text-2xl"
                        title="New Directory"
                        on:click={() => {
                            let name = prompt(
                                "Please enter the name of the new directory"
                            );
                            if (validateInput(name)) {
                                sendCmd(`${currentPath}/${name}`, "mkdir");
                            }
                        }}
                    />
                    <button
                        class="i-fa-file-medical"
                        title="New File"
                        on:click={() => {
                            let name = prompt("Please enter the name of the new file");
                            if (validateInput(name)) {
                                sendCmd(`${currentPath}/${name}`, "mkfile");
                            }
                        }}
                    />
                    <button
                        class="i-fa-file-arrow-up"
                        title="Upload File"
                        on:click={() => {
                            fileDialog.click();
                        }}
                        ><input
                            type="file"
                            class="hidden"
                            bind:this={fileDialog}
                            on:input={() => {
                                if (fileDialog.files !== null) {
                                    let size = Math.ceil(
                                        fileDialog.files[0].size / (1000 * 1000)
                                    );
                                    fileSend(
                                        `${currentPath}/${fileDialog.files[0].name}`,
                                        "up",
                                        `${size}`
                                    );
                                    for (
                                        let i = 0;
                                        i < fileDialog.files[0].size;
                                        i += 1000 * 1000
                                    ) {
                                        fileSocket.send(
                                            fileDialog.files[0].slice(i, i + 1000 * 1000)
                                        );
                                    }
                                }
                            }}
                        /></button
                    >
                    {#if selPath.path !== "" && selPath.maintype !== "notafile"}
                        <button
                            class="i-fa-i-cursor"
                            title="Rename"
                            on:click={() => {
                                let name = prompt(
                                    "Please enter the new name of the file"
                                );
                                if (validateInput(name)) {
                                    rename(selPath.path, `${currentPath}/${name}`);
                                }
                            }}
                        />
                        {#if selPath.maintype !== "dir"}
                            <button
                                class="i-fa-copy"
                                title="Copy"
                                on:click={() => sendCmd(selPath.path, "copy")}
                            />
                        {/if}
                        <button
                            class="i-fa-trash"
                            title="Delete"
                            on:click={() => {
                                if (
                                    confirm(
                                        `Are you sure you want to delete the ${
                                            selPath.maintype === "dir"
                                                ? "directory"
                                                : "file"
                                        } ${selPath.name}?${
                                            selPath.maintype === "dir"
                                                ? " This will delete everything in it!"
                                                : ""
                                        }`
                                    )
                                ) {
                                    sendCmd(
                                        selPath.path,
                                        `rm${selPath.maintype === "dir" ? "dir" : ""}`
                                    );
                                }
                            }}
                        />
                        <button
                            class="i-fa-file-arrow-down"
                            title="Download"
                            on:click={() => {
                                fileSend(selPath.path, "dl", "");
                                currentPath = selPath.path;
                                downloading = true;
                            }}
                        />
                    {/if}
                {/if}
            {/if}
        </div>
    </div>
</main>
