<script lang="ts">
    import microlight from "microlight";
    import autosize from "autosize";
    import {
        faFile,
        faFileAlt,
        faFileArchive,
        faFileImage,
        faFileAudio,
        faFileVideo,
        IconDefinition,
        faFilePdf,
        faFolder,
        faCopy,
        faSave,
        faTrash,
        faFolderPlus,
        faFileMedical,
        faICursor,
    } from "@fortawesome/free-solid-svg-icons";
    import Fa from "svelte-fa";

    export let socket;
    export let socketData: broserList;
    export let binData;

    let fileDataSet = false;
    let pathArray;
    let fileData;
    let fileText;
    let fileDiv;
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

    $: autosize(fileText), autosize.update();
    // Skip first array element (empty string)
    $: pathArray = currentPath.split("/").slice(1);
    $: socketData.textdata != undefined &&
        !fileDataSet &&
        ((fileData = socketData.textdata), (fileDataSet = true));
    // Set innerText manually to avoid issues with highlighting
    $: fileDiv != undefined &&
        ((fileDiv.innerHTML = fileData
            .replace(new RegExp("&", "g"), "&amp;")
            .replace(new RegExp("<", "g"), "&lt;")),
        microlight.reset());

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
        socket.send(JSON.stringify({ cmd: cmd, args: [path] }));
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
        socket.send(
            JSON.stringify({ cmd: "rename", args: [oldname, newname] })
        );
        fileDataSet = false;
    }

    function syncScroll() {
        fileDiv.scrollTop = fileText.scrollTop;
        fileDiv.scrollLeft = fileText.scrollLeft;
    }

    function checkTab(event) {
        if (event.keyCode == 9) {
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

    function getIcon(maintype: string, subtype: string): IconDefinition {
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

    function sendFile(path: string) {
        socket.send(JSON.stringify({ cmd: "save", args: [path, fileData] }));
    }

    function unitCalc(num: number) {
        if (num > 1000000000) {
            return Math.round((num / 1000000000) * 100) / 100 + " GB";
        } else if (num > 1000000) {
            return Math.round((num / 1000000) * 100) / 100 + " MB";
        } else if (num > 1000) {
            return Math.round((num / 1000) * 100) / 100 + " KB";
        } else if (num < 1000) {
            return num + " bytes";
        }
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
                <table class="bg-white w-full dark:bg-black">
                    <tr>
                        <th class="px-2">Name</th>
                        <th class="px-2">Kind</th>
                        <th class="px-2">Size</th>
                    </tr>
                    {#each socketData.contents as contents}
                        <tr
                            class="even:bg-white odd:bg-gray-200 dark:even:bg-black dark:odd:bg-gray-800 select-none{selPath ==
                            contents
                                ? ' !bg-blue-400 dark:!bg-blue-600'
                                : ''}"
                            on:dblclick={() => {
                                switch (contents.maintype) {
                                    case "dir":
                                        sendCmd(contents.path, "cd");
                                        break;
                                    case "text":
                                        sendCmd(contents.path, "open");
                                        break;
                                    case "image":
                                        sendCmd(contents.path, "img");
                                        break;
                                    default:
                                        alert(
                                            "ERROR: can't view that type of file"
                                        );
                                }
                                currentPath = contents.path;
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
                                />{contents.name}</td
                            >
                            <td class="px-2">{contents.prettytype}</td>
                            <td class="px-2"
                                >{contents.maintype == "dir"
                                    ? "--"
                                    : unitCalc(contents.size)}</td
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
                        spellcheck="false"
                        class="w-full font-mono text-sm bg-transparent text-transparent whitespace-pre tab-4 !overflow-x-scroll caret-black dark:caret-white z-20 focus:outline-none p-px"
                    />
                    <div
                        bind:this={fileDiv}
                        class="w-full microlight -ml-[100%] font-mono whitespace-pre bg-white dark:bg-black text-sm z-10 tab-4 !overflow-x-scroll p-px"
                    />
                </div>
            {:else if binData != ""}
                <div>
                    <img src={binData} alt="Unknown" />
                </div>
            {/if}
        </div>
        <div
            class="min-w-16 bg-gray-300 dark:bg-gray-800 ml-4 min-h-full max-h-full flex flex-col gap-2 items-center justify-center"
        >
            {#if socketData.contents != undefined}
                <span
                    class="cursor-pointer"
                    title="New Directory"
                    on:click={() => {
                        let name = prompt(
                            "Please enter the name of the new directory"
                        );
                        sendCmd(currentPath + "/" + name, "mkdir");
                    }}><Fa icon={faFolderPlus} size="lg" /></span
                >
                <span
                    class="cursor-pointer"
                    title="New File"
                    on:click={() => {
                        let name = prompt(
                            "Please enter the name of the new file"
                        );
                        sendCmd(currentPath + "/" + name, "mkfile");
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
                            rename(selPath.path, currentPath + "/" + name);
                        }}><Fa icon={faICursor} size="lg" /></span
                    >
                    {#if selPath.maintype == "dir"}
                        <span
                            class="cursor-pointer"
                            title="Delete"
                            on:click={() => sendCmd(selPath.path, "rmdir")}
                            ><Fa icon={faTrash} size="lg" /></span
                        >
                    {:else}
                        <span
                            class="cursor-pointer"
                            title="Copy"
                            on:click={() => sendCmd(selPath.path, "copy")}
                            ><Fa icon={faCopy} size="lg" /></span
                        >
                        <span
                            class="cursor-pointer"
                            title="Delete"
                            on:click={() => sendCmd(selPath.path, "rm")}
                            ><Fa icon={faTrash} size="lg" /></span
                        >
                    {/if}
                {/if}
            {:else if socketData.textdata != undefined}
                <span
                    class="cursor-pointer"
                    on:click={() => sendFile(currentPath)}
                    ><Fa icon={faSave} size="lg" /></span
                >{/if}
        </div>
    </div>
</main>
