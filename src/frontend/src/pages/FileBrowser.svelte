<script lang="ts">
    import microlight from "microlight";
    import {
        faFile,
        faFileAlt,
        faFileArchive,
        faFileImage,
        faFileAudio,
        faFileVideo,
        faFilePdf,
        faFolder,
        faCopy,
        faSave,
        faTrash,
        faFolderPlus,
        faFileMedical,
        faICursor,
        faSyncAlt,
        faHighlighter,
        faEyeSlash,
        faEye,
        faFileDownload,
        faFileUpload,
    } from "@fortawesome/free-solid-svg-icons";
    import Fa from "svelte-fa";
    import prettyBytes from "pretty-bytes";

    export let socketSend: (cmd: string, args: string[]) => void;
    export let socketData: browserList;
    export let node: string;
    export let login: boolean;
    let fileDataSet = false;
    let showHidden = false;
    let maxSlices = 0;
    let currentSlices = 0;
    let binData: BlobPart[] = [];
    let downloading = false;

    let fileDialog: HTMLInputElement;

    const fileSocket = new WebSocket(
        `${
            window.location.protocol == "https:" ? "wss" : "ws"
        }://${node}/ws/file`
    );
    fileSocket.onmessage = (e: MessageEvent) => {
        if (typeof e.data == "string") {
            try {
                let msg = JSON.parse(e.data);
                if (msg.finished) {
                    sendCmd(`${currentPath}/.`, "refresh");
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
            if (currentSlices == maxSlices) {
                binURL = URL.createObjectURL(new Blob(binData));
                maxSlices = 0;
                currentSlices = 0;
                console.log(binURL);
            }
        }
    };

    let pathArray: string[];
    let fileData = "";
    let fileText: HTMLTextAreaElement;
    let fileDiv: HTMLDivElement;
    let saved = true;
    // TODO: better solution than just assuming dashboard is being run by root
    let currentPath = "/root";
    let binURL = "";

    let selPath: browser = {
        name: "",
        path: "",
        maintype: "",
        subtype: "",
        prettytype: "",
        size: 0,
    };

    let highlighting = false;

    // Skip first array element (empty string)
    $: pathArray = currentPath.split("/").slice(1);
    // Set innerHTML manually to avoid issues with highlighting
    $: fileDiv != undefined &&
        (fileDiv.innerHTML =
            fileData[fileData.length - 1] == "\n"
                ? fileData + " "
                : fileData
                      .replace(new RegExp("&", "g"), "&amp;")
                      .replace(new RegExp("<", "g"), "&lt;")),
        microlight.reset();

    interface browserList {
        contents?: browser[];
        currentpath?: string;
    }

    interface browser {
        name: string;
        path: string;
        maintype: string;
        subtype: string;
        prettytype: string;
        size: number;
    }

    function sendCmd(path: string, cmd: string) {
        if (cmd == "rm" || cmd == "rmdir" || cmd == "cd") {
            selPath = {
                name: "",
                path: "",
                maintype: "",
                subtype: "",
                prettytype: "",
                size: 0,
            };
        }
        socketSend(cmd, [path]);
        fileDataSet = false;
        if (downloading) {
            downloading = false;
            URL.revokeObjectURL(binURL);
            binURL = "";
        }
    }

    function fileSend(path: string, cmd: string, arg: string) {
        let json;
        if (login) {
            json = JSON.stringify({
                cmd,
                path,
                arg,
                token: JSON.parse(localStorage.getItem("tokens"))[node],
            });
        } else {
            json = JSON.stringify({
                cmd,
                path,
                arg,
            });
        }
        fileSocket.send(json);
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
        socketSend("rename", [oldname, newname]);
    }

    function syncScroll() {
        fileDiv.scrollTop = fileText.scrollTop;
        fileDiv.scrollLeft = fileText.scrollLeft;
    }

    function checkTab(event: KeyboardEvent) {
        if (event.key == "Tab") {
            event.preventDefault();

            let startPos = fileText.selectionStart;
            let endPos = fileText.selectionEnd;

            let tabAdded =
                fileData.substring(0, startPos) +
                "\t" +
                fileData.substring(endPos);

            fileText.value = tabAdded;
            fileData = tabAdded;
            fileText.selectionStart = fileText.selectionEnd = startPos + 1;
        }
    }

    function getIcon(maintype: string, subtype: string) {
        switch (maintype) {
            case "dir":
                return faFolder;
            case "image":
                return faFileImage;
            case "video":
                return faFileVideo;
            case "audio":
                return faFileAudio;
            case "archive":
                if (subtype == "pdf") {
                    return faFilePdf;
                }
                return faFileArchive;
            case "text":
                return faFileAlt;
            default:
                return faFile;
        }
    }

    function validateInput(name: string) {
        if (name) {
            for (let element of socketData.contents) {
                if (element.name == name) {
                    if (
                        confirm(
                            `This will overwrite the ${
                                element.maintype == "dir" ? "directory" : "file"
                            } ${name}${
                                element.maintype == "dir"
                                    ? ", and delete everything in it"
                                    : ""
                            }. Are you sure you want to continue?`
                        )
                    ) {
                        sendCmd(
                            `${currentPath}/${name}`,
                            `rm${element.maintype == "dir" ? "dir" : ""}`
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
            <div class="mb-2 p bg-white dark:bg-black">
                <button
                    class="btn px-2 focus:outline-none"
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
                    {path != pathArray[0] ? " /" : ""}
                    {#if path == pathArray[pathArray.length - 1]}
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
                                    if (element == path) {
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
                                fileText.style.height = `${
                                    fileText.scrollHeight + 10
                                }px`;
                                fileDiv.style.height = "auto";
                            }
                        }}
                        spellcheck="false"
                        class="w-full font-mono text-sm{highlighting
                            ? ' bg-transparent text-transparent'
                            : ''} whitespace-pre tab-4 caret-black z-20 dark:caret-white focus:outline-none p-px resize-none overflow-y-hidden"
                    />
                    <div
                        bind:this={fileDiv}
                        class="w-full microlight font-mono whitespace-pre bg-white dark:bg-black text-sm z-10 tab-4 p-px -ml-[100%] overflow-y-hidden{highlighting
                            ? ''
                            : ' invisible'}"
                    />
                </div>
            {:else if downloading}
                {#if binURL != ""}
                    <a
                        href={binURL}
                        target="_blank"
                        download={`${selPath.name.split(".")[0]}.zip`}
                        >Click to Download</a
                    >
                {:else if maxSlices == 0}
                    <h2>
                        Zipping {selPath.maintype == "dir"
                            ? "directory"
                            : "file"}...
                    </h2>
                {:else}
                    <h2>Receiving {currentSlices}MB out of {maxSlices}MB</h2>
                {/if}
            {:else if socketData.contents != undefined}
                <table
                    class="bg-white w-full dark:bg-black table-fixed min-w-50"
                >
                    <tr>
                        <th class="px-2">Name</th>
                        <th class="px-2">Kind</th>
                        <th class="px-2">Size</th>
                    </tr>
                    {#each socketData.contents as contents}
                        <tr
                            class="select-none{selPath.path == contents.path
                                ? ' !bg-dplime-dark'
                                : ''} even:bg-white odd:bg-gray-200 dark:even:bg-black dark:odd:bg-gray-800{!showHidden &&
                            contents.name[0] == '.'
                                ? ' hidden'
                                : ''}"
                            on:dblclick={() => {
                                switch (contents.maintype) {
                                    case "dir":
                                        sendCmd(contents.path, "cd");
                                        currentPath = contents.path;
                                        break;
                                    case "text":
                                        fileSend(contents.path, "open", "");
                                        currentPath = contents.path;
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
                                ><Fa
                                    icon={getIcon(
                                        contents.maintype,
                                        contents.subtype
                                    )}
                                    class="mr-2"
                                /><span class="break-words"
                                    >{contents.name}</span
                                ></td
                            >
                            <td class="px-2">{contents.prettytype}</td>
                            <td class="px-2"
                                >{contents.maintype == "dir"
                                    ? "-"
                                    : prettyBytes(contents.size)}</td
                            >
                        </tr>
                    {/each}
                </table>
            {/if}
        </div>
        <div
            class="min-w-16 bg-gray-300 dark:bg-gray-800 flex flex-col items-center ml-4 justify-center sticky top-10 p-4 h-min gap-2"
        >
            {#if fileDataSet}
                <span
                    title="Syntax Highlighting"
                    on:click={() => {
                        highlighting = !highlighting;
                    }}
                    ><Fa
                        icon={faHighlighter}
                        class={highlighting ? "" : "opacity-50"}
                        size="lg"
                    /></span
                >
                <span
                    class="cursor-pointer"
                    on:click={() => {
                        fileSend(currentPath, "save", fileData), (saved = true);
                    }}><Fa icon={faSave} size="lg" /></span
                >
            {:else if socketData.contents != undefined}
                <span
                    class="cursor-pointer"
                    title="Refresh"
                    on:click={() => {
                        sendCmd(`${currentPath}/.`, "refresh");
                    }}><Fa icon={faSyncAlt} size="lg" /></span
                >
                <span
                    class="cursor-pointer"
                    title="New Directory"
                    on:click={() => {
                        let name = prompt(
                            "Please enter the name of the new directory"
                        );
                        if (validateInput(name)) {
                            sendCmd(`${currentPath}/${name}`, "mkdir");
                        }
                    }}><Fa icon={faFolderPlus} size="lg" /></span
                >
                <span
                    class="cursor-pointer"
                    title="New File"
                    on:click={() => {
                        let name = prompt(
                            "Please enter the name of the new file"
                        );
                        if (validateInput(name)) {
                            sendCmd(`${currentPath}/${name}`, "mkfile");
                        }
                    }}><Fa icon={faFileMedical} size="lg" /></span
                >
                <span
                    title="{showHidden ? 'Hide' : 'Show'} Hidden Files"
                    on:click={() => {
                        showHidden = !showHidden;
                    }}
                    ><Fa
                        icon={showHidden ? faEyeSlash : faEye}
                        size="lg"
                    /></span
                >
                <span
                    class="cursor-pointer"
                    title="Upload File"
                    on:click={() => {
                        fileDialog.click();
                    }}
                    ><input
                        type="file"
                        class="hidden"
                        bind:this={fileDialog}
                        on:input={() => {
                            let size = Math.ceil(
                                fileDialog.files[0].size / (1000 * 1000)
                            );
                            fileSend(
                                `${currentPath}/${fileDialog.files[0].name}`,
                                "up",
                                `${size}`
                            );
                            for (let i = 0; i < size; i++) {
                                fileSocket.send(
                                    fileDialog.files[0].slice(
                                        i * 1000 * 1000,
                                        Math.min(
                                            (i + 1) * 1000 * 1000,
                                            fileDialog.files[0].size
                                        )
                                    )
                                );
                            }
                        }}
                    /><Fa icon={faFileUpload} size="lg" /></span
                >
                {#if selPath.path != ""}
                    <span
                        class="cursor-pointer"
                        title="Rename"
                        on:click={() => {
                            let name = prompt(
                                "Please enter the new name of the file"
                            );
                            if (validateInput(name)) {
                                rename(selPath.path, `${currentPath}/${name}`);
                            }
                        }}><Fa icon={faICursor} size="lg" /></span
                    >
                    {#if selPath.maintype != "dir"}
                        <span
                            class="cursor-pointer"
                            title="Copy"
                            on:click={() => sendCmd(selPath.path, "copy")}
                            ><Fa icon={faCopy} size="lg" /></span
                        >
                    {/if}
                    <span
                        class="cursor-pointer"
                        title="Delete"
                        on:click={() => {
                            if (
                                confirm(
                                    `Are you sure you want to delete the ${
                                        selPath.maintype == "dir"
                                            ? "directory"
                                            : "file"
                                    } ${selPath.name}?${
                                        selPath.maintype == "dir"
                                            ? " This will delete everything in it!"
                                            : ""
                                    }`
                                )
                            ) {
                                sendCmd(
                                    selPath.path,
                                    `rm${
                                        selPath.maintype == "dir" ? "dir" : ""
                                    }`
                                );
                            }
                        }}><Fa icon={faTrash} size="lg" /></span
                    >
                    <span
                        class="cursor-pointer"
                        title="Download"
                        on:click={() => {
                            fileSend(selPath.path, "dl", "");
                            currentPath = selPath.path;
                            downloading = true;
                        }}><Fa icon={faFileDownload} size="lg" /></span
                    >
                {/if}
            {/if}
        </div>
    </div>
</main>
