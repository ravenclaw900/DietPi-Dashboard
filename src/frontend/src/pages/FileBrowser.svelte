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
    } from "@fortawesome/free-solid-svg-icons";
    import Fa from "svelte-fa";
    import prettyBytes from "pretty-bytes";

    export let socketSend: (cmd: string, args: string[]) => void;
    export let socketData: broserList;
    export let binData: string;

    let fileDataSet = false;
    let pathArray: string[];
    let fileData: string;
    let fileText: HTMLTextAreaElement;
    let fileDiv: HTMLDivElement;
    // TODO: better solution than just assuming dashboard is being run by root
    let currentPath = "/root";

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
    $: socketData.textdata != undefined &&
        !fileDataSet &&
        ((fileData = socketData.textdata), (fileDataSet = true));
    // Set innerHTML manually to avoid issues with highlighting
    $: fileDiv != undefined &&
        (fileDiv.innerHTML = fileData
            .replace(new RegExp("&", "g"), "&amp;")
            .replace(new RegExp("<", "g"), "&lt;")),
        microlight.reset();

    interface broserList {
        contents?: browser[];
        textdata?: string;
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
        selPath = {
            name: "",
            path: "",
            maintype: "",
            subtype: "",
            prettytype: "",
            size: 0,
        };
        socketSend(cmd, [path]);
        fileDataSet = false;
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
        fileDataSet = false;
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
                        sendCmd("/", "cd");
                        currentPath = "/";
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
                                sendCmd(fullPath, "cd");
                                currentPath = fullPath;
                            }}>{path}</button
                        >
                    {/if}
                {/each}
            </div>
            {#if socketData.contents != undefined}
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
                            class="select-none{selPath == contents
                                ? ' !bg-dplime-dark'
                                : ''} even:bg-white odd:bg-gray-200 dark:even:bg-black dark:odd:bg-gray-800"
                            on:dblclick={() => {
                                switch (contents.maintype) {
                                    case "dir":
                                        sendCmd(contents.path, "cd");
                                        currentPath = contents.path;
                                        break;
                                    case "text":
                                        sendCmd(contents.path, "open");
                                        currentPath = contents.path;
                                        break;
                                    case "image":
                                        sendCmd(contents.path, "img");
                                        currentPath = contents.path;
                                        break;
                                    default:
                                        alert(
                                            "ERROR: can't view that type of file"
                                        );
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
            {:else if socketData.textdata != undefined}
                <div class="flex">
                    <textarea
                        bind:value={fileData}
                        bind:this={fileText}
                        on:scroll={syncScroll}
                        on:keydown={checkTab}
                        on:input={() => {
                            this.style.height = "auto";
                            this.style.height = `${this.scrollHeight}px`;
                        }}
                        spellcheck="false"
                        class="w-full font-mono text-sm{highlighting
                            ? ' bg-transparent text-transparent'
                            : ''} whitespace-pre tab-4 caret-black dark:caret-white z-20 focus:outline-none p-px resize-none"
                        style="height:{this.scrollHeight}px;overflow-y:hidden;"
                    />
                    <div
                        bind:this={fileDiv}
                        class="w-full microlight font-mono whitespace-pre bg-white dark:bg-black text-sm z-10 tab-4 p-px -ml-[100%]{highlighting
                            ? ''
                            : ' invisible'}"
                    />
                </div>
            {:else if binData != ""}
                <div>
                    <img src={binData} alt="Unknown" />
                </div>
            {/if}
        </div>
        <div
            class="min-w-16 bg-gray-300 dark:bg-gray-800 flex flex-col items-center ml-4 justify-center sticky top-10 p-4 h-min gap-2"
        >
            {#if socketData.contents != undefined}
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
                {/if}
            {:else if socketData.textdata != undefined}
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
                    on:click={() => socketSend("save", [currentPath, fileData])}
                    ><Fa icon={faSave} size="lg" /></span
                >{/if}
        </div>
    </div>
</main>
