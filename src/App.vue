<script setup lang="ts">
import { ref, shallowRef, nextTick, onBeforeMount, onBeforeUnmount, onMounted } from "vue"

import { useIndexStore } from "@/store"

import { listen, UnlistenFn, emit } from "@tauri-apps/api/event"
import { invoke } from "@tauri-apps/api/tauri"
import { exit } from "@tauri-apps/api/process"

import { NConfigProvider, NGlobalStyle, darkTheme, NButton, NIcon } from "naive-ui"
import { Close as IconClose, Cog, FolderSharp, CloseCircle, ColorPalette } from "@vicons/ionicons5"

import FileTreeVue from "@/components/FileTree.vue"
import VditorVue from "@/components/Vditor.vue"
import WangEditorVue from "@/components/WangEditor.vue"
import SettingVue from "@/components/Setting.vue"

import Markdown from "@/components/icons/md.vue"
import Word from "@/components/icons/word.vue"
import Setting from "@/components/icons/setting.vue"
import Point from "@/components/icons/point.vue"
import Close from "@/components/icons/close.vue"
import type { DocFile, FileTree } from "@/types"

const tabsComponent = shallowRef([SettingVue, VditorVue, WangEditorVue])
const iconComponent = shallowRef([Setting, Markdown, Word])

const handleExit = async () => {
    await exit()
}

const indexStore = useIndexStore()

const mode = ref<"ir" | "sv" | "wysiwyg">("ir")
const key = ref(0)
const save = ref(false)
let unlisten: UnlistenFn
onBeforeMount(async () => {
    let m = localStorage.getItem("mode")
    if (m) {
        mode.value = m as "ir" | "sv" | "wysiwyg"
    }
    unlisten = await listen<any>("file-system-changed", async event => {
        switch (event.payload.type_) {
            case 1: // create folder
                break
            case 2: // create file
                break
            case 3: // write folder
                break
            case 4: // write file
                if (save.value) {
                    save.value = false
                    return
                }
                let has = tabs.value.find(v => v.path === event.payload.path)
                if (has) {
                    const file = await invoke<any>("read", {
                        path: event.payload.path,
                    })
                    has.content = file.content
                    has.updated = file.updated
                }
                console.log("write file", event.payload.path)
                break
            case 5: // rename
                break
            case -1: // delete folder
                break
            case -2: // delete file
                break
        }
    })
    // ctrl s
    document.addEventListener("keydown", async e => {
        if (e.ctrlKey && e.key === "s") {
            e.preventDefault()
            await handleSave()
        }
    })
    document.addEventListener("contextmenu", e => {
        e.preventDefault()
    })
    let f = localStorage.getItem("folder")
    if (f) {
        await emit("watch-path-changed", {
            type_: 0,
            path: f,
        })
        folder.value = f
        await openFolder(f)
    } else {
        localStorage.removeItem("defaultExpandedKeys")
        tabs.value = []
    }
    let ts = localStorage.getItem("tabs")
    if (ts) {
        let tbs = JSON.parse(ts)
        let paths = tbs.map((t: any) => t.path)
        let files = await invoke<any>("reads", { paths })
        tabs.value = tbs
            .map((tab: any) => {
                let file = files.find((file: any) => file.path === tab.path)
                if (file) {
                    tab.updated = file.updated
                    tab.content = file.content
                    return tab
                }
                return tab
            })
            .filter((tab: any) => tab)
        if (tabs.value.length > 0) {
            let ctab = localStorage.getItem("tab")
            if (ctab) {
                let tab = tabs.value.find((tab: any) => tab.path === ctab)
                currentTab.value = tab || tabs.value[0]
            }
        }
    }
})

const width = ref(300)
const resizeable = ref<boolean>(false)
const oldWidth = ref(0)
const oldX = ref(0)
const serResizeable = (ev: MouseEvent) => {
    resizeable.value = true
    oldWidth.value = width.value
    oldX.value = ev.clientX
}
onMounted(async () => {
    document.body.addEventListener("mousemove", ev => {
        if (resizeable.value) {
            let newWidth = oldWidth.value + ev.clientX - oldX.value
            if (newWidth < 200) {
                newWidth = 200
            } else if (newWidth > 1000) {
                newWidth = 1000
            } else {
                width.value = newWidth
            }
        }
    })
    document.body.addEventListener("mouseup", ev => {
        resizeable.value = false
    })

    setTimeout(async () => {
        await invoke("close_splashscreen")
        key.value++
    }, 300)
})

onBeforeUnmount(() => {
    // if unlisten is not called, the event listener will not be removed
    if (unlisten) {
        unlisten()
    }
})

const filetree = ref<FileTree[]>([])
const folder = ref("")
const showTree = ref(true)
const openFolder = async (path: string) => {
    const open = await invoke<FileTree[]>("open", { path })
    if (open.length > 0) {
        filetree.value = open
        showTree.value = false
        nextTick(() => {
            showTree.value = true
        })
    } else {
        await closeFolder()
    }
}
const selectFolder = async () => {
    const select = await invoke<{ type_: number; path: string }>("select")
    if (select.type_ === 0) {
        folder.value = select.path
        localStorage.setItem("folder", select.path)
        localStorage.removeItem("defaultExpandedKeys")
        tabs.value = []
        await openFolder(select.path)
        emit("watch-path-changed", select)
    } else {
        // alert("No folder selected")
    }
}

const filetreeRef = ref()
const closeFolder = async () => {
    emit("watch-path-changed", {
        type_: -100,
        path: "",
    })
    tabs.value = []
    folder.value = ""
    currentTab.value.name = ""
    localStorage.removeItem("folder")
    localStorage.removeItem("defaultExpandedKeys")
    localStorage.removeItem("tab")
    localStorage.removeItem("tabs")
    if (filetreeRef.value) {
        filetreeRef.value.handleClose()
    }
}

const setMode = () => {
    let modes: ["ir", "sv", "wysiwyg"] = ["ir", "sv", "wysiwyg"]
    let index = modes.indexOf(mode.value)
    if (index < 2) {
        mode.value = modes[index + 1]
    } else {
        mode.value = modes[0]
    }
    localStorage.setItem("mode", mode.value)
    key.value++
}

const tabs = ref<DocFile[]>([])
const currentTab = ref<DocFile>({
    name: "",
    path: "",
    type_: 0,
    updated: 0,
    content: "",
    changed: false,
})

const handleTabChanged = (data: any) => {
    currentTab.value = data
    localStorage.setItem("tab", currentTab.value.path)
}

const handleSetTheme = () => {
    if (indexStore.theme === "dark") {
        indexStore.updateConfig({
            theme: "light",
        })
    } else {
        indexStore.updateConfig({
            theme: "dark",
        })
    }
}

const handleOpenFile = async (t: any) => {
    if (t.type === 0) {
        let tmp = {
            type_: t.type,
            name: t.name,
            path: t.key,
            updated: 0,
            content: "",
            changed: false,
        }
        tabs.value.push(tmp)
        currentTab.value = tmp
        localStorage.setItem("tabs", JSON.stringify(tabs.value))
        localStorage.setItem("tab", currentTab.value.path)
        return
    }
    const file = await invoke<any>("read", {
        path: t.key,
    })
    let has = tabs.value.find(v => v.path === t.key)
    if (has) {
        has.content = file.content
        has.updated = file.updated
        currentTab.value = has
    } else {
        let tmp = {
            type_: t.type,
            name: t.name,
            path: t.key,
            updated: file.updated,
            content: file.content,
            changed: false,
        }
        tabs.value.push(tmp)
        currentTab.value = tmp
        localStorage.setItem("tabs", JSON.stringify(tabs.value))
    }
    localStorage.setItem("tab", currentTab.value.path)
}

const handleUpdateFile = ({ path, content }: { path: string; content: string }) => {
    if (currentTab.value.path === path) {
        currentTab.value.content = content
        currentTab.value.changed = true
        localStorage.setItem("tabs", JSON.stringify(tabs.value))
    }
}

const handleSave = async () => {
    if (currentTab.value.changed) {
        save.value = true
        await invoke("write", {
            path: currentTab.value.path,
            content: currentTab.value.content,
        })
        currentTab.value.changed = false
        currentTab.value.updated = Date.now()
        localStorage.setItem("tabs", JSON.stringify(tabs.value))
    }
}

const handleRelaod = async () => {
    window.location.reload()
}

const handleOpenSetting = async () => {
    let has = tabs.value.find(v => v.path === "AhriDocs - Setting")
    if (has) {
        currentTab.value = has
    } else {
        await handleOpenFile({
            label: "Setting",
            name: "Setting",
            key: "AhriDocs - Setting",
            type: 0,
            edit: false,
        })
    }
}
</script>

<template>
    <NConfigProvider :theme="indexStore.theme == 'dark' ? darkTheme : null">
        <NGlobalStyle />
        <div id="container" class="container" :class="indexStore.theme">
            <aside data-tauri-drag-region class="aside">
                <div class="top">
                    <svg
                        @click="handleRelaod"
                        title="AhriDocs Reload"
                        t="1669484176886"
                        class="logo"
                        viewBox="0 0 1024 1024"
                        version="1.1"
                        xmlns="http://www.w3.org/2000/svg"
                        p-id="6458"
                        width="24"
                        height="24"
                    >
                        <path
                            d="M512 448V0H64v1024h896V448H512zM192 256h192v64H192V256z m640 576H192v-64h640v64z m0-256H192V512h640v64z"
                            p-id="6459"
                            fill="#1296db"
                        ></path>
                        <path d="M576 384h384L576 0v384z" p-id="6460" fill="#1296db"></path>
                    </svg>
                </div>
                <div class="bottom">
                    <n-button circle quaternary size="small" @click.stop="handleOpenSetting">
                        <template #icon>
                            <n-icon class="btn-icon-setting">
                                <Cog />
                            </n-icon>
                        </template>
                    </n-button>
                    <n-button circle quaternary size="small" @click.stop="handleExit">
                        <template #icon>
                            <n-icon class="btn-icon-setting">
                                <IconClose />
                            </n-icon>
                        </template>
                    </n-button>
                </div>
            </aside>
            <div class="file-tree-container" :style="`width: ${width}px`">
                <div class="header">
                    <button class="btn" @click="selectFolder">
                        <NIcon size="18">
                            <FolderSharp />
                        </NIcon>
                    </button>
                    <button class="btn" @click="closeFolder">
                        <NIcon size="18">
                            <CloseCircle />
                        </NIcon>
                    </button>
                    <button class="btn" @click="handleSetTheme">
                        <NIcon size="18">
                            <ColorPalette />
                        </NIcon>
                    </button>
                    <button class="btn" @click="setMode">{{ mode }}</button>
                </div>
                <div class="file-tree-body">
                    <FileTreeVue
                        v-if="showTree"
                        ref="filetreeRef"
                        @handleOpenFile="handleOpenFile"
                        :value="filetree"
                        :theme="indexStore.theme"
                    />
                </div>
            </div>
            <div class="split" @mousedown="serResizeable" :style="`left: ${width + 50}px`"></div>
            <div class="tab-view-container" :style="`left: ${width + 55}px`">
                <div class="tab" :class="indexStore.theme">
                    <div class="tab-bar nocopy">
                        <div data-tauri-drag-region class="title"></div>
                        <div
                            v-for="item in tabs"
                            :key="item.name"
                            class="tab-bar-item"
                            :class="{
                                active: item.name === currentTab.name,
                                changed: item.changed,
                            }"
                            @click="handleTabChanged(item)"
                        >
                            <span class="icon">
                                <component :is="iconComponent[item.type_]" />
                            </span>
                            <span class="name">{{ item.name }}</span>
                            <span class="close-btn">
                                <Point class="point" />
                                <Close class="close" />
                            </span>
                        </div>
                        <div class="bg"></div>
                    </div>
                    <div class="tab-panel">
                        <div
                            class="tab-panel-item"
                            v-for="item in tabs"
                            :style="`z-index: ${item.name === currentTab.name ? 100 : 0}`"
                        >
                            <component
                                :is="tabsComponent[currentTab.type_]"
                                :mode="mode"
                                :key="key"
                                @handleUpdateFile="handleUpdateFile"
                                :value="currentTab"
                            />
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </NConfigProvider>
</template>

<style lang="scss">
#container {
    svg path {
        fill: #586069;
    }
    svg.logo path {
        cursor: pointer;
    }
}
#container.dark {
    svg.logo path {
        fill: #c6d5da;
    }
    .tab-view-container {
        .tab {
            .tab-bar {
                .tab-bar-item {
                    &.active {
                        svg.md path,
                        svg.word path,
                        svg.setting path {
                            fill: #c6d5da;
                        }
                        svg.close path,
                        svg.point path {
                            fill: #909da0;
                        }
                    }
                }
            }
        }
    }
}
#container.light {
    svg.logo path {
        fill: #55595e;
    }
    .tab-view-container {
        .tab {
            .tab-bar {
                .tab-bar-item {
                    &.active {
                        svg.md path,
                        svg.word path,
                        svg.setting path {
                            fill: #24292e;
                        }
                        svg.close path,
                        svg.point path {
                            fill: #909da0;
                        }
                    }
                }
            }
        }
    }
}
</style>

<style scoped lang="scss">
.file-tree-container .header {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    height: 40px;
    display: flex;
    justify-content: flex-start;
    align-items: center;
    .btn {
        margin-left: 6px;
        border: none;
        background: none;
        padding: 8px 14px;
        margin: 6px 2px;
        cursor: pointer;
    }
}
.dark .btn {
    background-color: #1d2125;
    color: #eee;
}

.dark .btn:hover {
    background-color: #2d2f33;
}

.light .btn {
    background-color: #f3f6f8;
    color: #333;
}

.light .btn:hover {
    background-color: #e3e6e8;
}

#container {
    height: 100%;
    width: 100%;
    overflow: hidden;
    position: relative;
    .aside {
        position: absolute;
        top: 0;
        left: 0;
        bottom: 0;
        width: 50px;
        cursor: move;
        display: flex;
        flex-direction: column;
        justify-content: space-between;
        align-items: center;
        .top {
            width: 100%;
            display: flex;
            justify-content: center;
            align-items: center;
            padding-top: 10px;
        }
        .bottom {
            width: 100%;
            height: 76px;
            display: flex;
            flex-direction: column;
            justify-content: space-between;
            align-items: center;
            padding-bottom: 10px;
        }
    }
    .file-tree-container {
        position: absolute;
        top: 0;
        left: 50px;
        bottom: 0;
        .file-tree-body {
            position: absolute;
            top: 46px;
            left: 0;
            right: 0;
            bottom: 0;
        }
    }
    .split {
        position: absolute;
        top: 0;
        bottom: 0;
        width: 5px;
        cursor: ew-resize;
    }
    .tab-view-container {
        position: absolute;
        top: 0;
        bottom: 0;
        right: 0;
        .tab {
            position: absolute;
            top: 0;
            left: 0;
            right: 0;
            bottom: 0;
            .tab-bar {
                position: absolute;
                top: 0;
                left: 0;
                right: 0;
                height: 46px;
                display: flex;
                overflow-y: hidden;
                overflow-x: scroll;
                .title {
                    z-index: 1;
                    position: absolute;
                    top: 0;
                    left: 0;
                    right: 0;
                    height: 40px;
                }
                .bg {
                    position: absolute;
                    left: 0;
                    right: 0;
                    bottom: 0;
                    height: 6px;
                }
                &::-webkit-scrollbar {
                    height: 6px;
                    display: none;
                }
                &:hover {
                    .bg {
                        display: none;
                    }
                    &::-webkit-scrollbar {
                        display: inline-block;
                    }
                }
                &::-webkit-scrollbar {
                    background-color: transparent;
                }
                &::-webkit-scrollbar-track {
                    background-color: transparent;
                }
                .tab-bar-item {
                    z-index: 2;
                    display: inline-block;
                    height: 40px;
                    line-height: 40px;
                    padding: 0 10px;
                    cursor: pointer;
                    display: flex;
                    justify-content: space-between;
                    align-items: center;
                    .name {
                        max-width: 200px;
                        overflow: hidden;
                        text-overflow: ellipsis;
                        white-space: nowrap;
                        padding: 0 6px;
                    }
                    .close-btn {
                        width: 12px;
                        display: inline-block;
                        margin-left: 10px;
                        .point {
                            display: none;
                        }
                        .close {
                            display: none;
                        }
                    }
                    &:hover {
                        .point {
                            display: none;
                        }
                        .close {
                            display: block;
                        }
                    }
                    &.active {
                        .point {
                            display: none;
                        }
                        .close {
                            display: block;
                        }
                        &.changed {
                            .point {
                                display: block;
                            }
                            .close {
                                display: none;
                            }
                            &:hover {
                                .point {
                                    display: none;
                                }
                                .close {
                                    display: block;
                                }
                            }
                        }
                    }
                }
            }
            .tab-panel {
                position: absolute;
                top: 40px;
                left: 0;
                right: 0;
                bottom: 0;
                overflow: hidden;

                .tab-panel-item {
                    position: absolute;
                    top: 0;
                    left: 0;
                    right: 0;
                    bottom: 0;
                    overflow: hidden;
                }
            }
        }
    }
}

#container.container.dark {
    background: #24292e;
    .aside {
        background: #1d2125;
    }
    .file-tree-container {
        background: #24292e;
        .header {
            background: #1d2125;
        }
    }
    .split {
        background: #1d2125;
    }
    .tab-view-container {
        .tab {
            .tab-bar {
                background: #1d2125;
                .bg {
                    background: #24292e;
                }
                &::-webkit-scrollbar {
                    background-color: #24292e;
                }
                &::-webkit-scrollbar-track {
                    background-color: #24292e;
                }
                &::-webkit-scrollbar-thumb {
                    background: #2d2f33;
                }
                .tab-bar-item {
                    color: #586069;
                    &.active {
                        color: #c6d5da;
                        border-bottom: 2px solid #c6d5da;
                        background: #24292e;
                    }
                }
            }
        }
    }
}

#container.container.light {
    background: #ffffff;
    .aside {
        background: #f3f6f8;
    }
    .file-tree-container {
        background: #ffffff;
        .header {
            background: #f3f6f8;
        }
    }
    .split {
        background: #f3f6f8;
    }
    .tab-view-container {
        .tab {
            .tab-bar {
                .bg {
                    background: #fafbfc;
                }
                background: #f3f6f8;
                &::-webkit-scrollbar {
                    background-color: #fafbfc;
                }
                &::-webkit-scrollbar-track {
                    background-color: #fafbfc;
                }
                &::-webkit-scrollbar-thumb {
                    background: #e1e4e8;
                }
                .tab-bar-item {
                    color: #586069;
                    &.active {
                        color: #24292e;
                        border-bottom: 2px solid #24292e;
                        background: #fafbfc;
                    }
                }
            }
        }
    }
}
</style>
