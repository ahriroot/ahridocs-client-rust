<script setup lang="ts">
import { ref, nextTick, onBeforeMount, onBeforeUnmount, onMounted } from "vue"

import { useIndexStore } from "@/store"

import { listen, UnlistenFn, emit } from "@tauri-apps/api/event"
import { invoke } from "@tauri-apps/api/tauri"

import { NConfigProvider, darkTheme } from "naive-ui"

import FileTreeVue from "@/components/FileTree.vue"
import EditorVue from "@/components/Editor.vue"
import Markdown from "@/components/icons/md.vue"
import Word from "@/components/icons/word.vue"
import Point from "@/components/icons/point.vue"
import Close from "@/components/icons/close.vue"
import type { DocFile, FileTree } from "@/types"

const indexStore = useIndexStore()

const key = ref(0)
let unlisten: UnlistenFn
onBeforeMount(async () => {
    unlisten = await listen<any>("file-system-changed", async event => {
        switch (event.payload.type_) {
            case 1: // create folder
                break
            case 2: // create file
                break
            case 3: // write folder
                break
            case 4: // write file
                let has = tabs.value.find(v => v.path === event.payload.path)
                if (has) {
                    const file = await invoke<any>("read", {
                        path: event.payload.path,
                    })
                    has.content = file.content
                    has.updated = file.updated
                }
                key.value++
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
                return null
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
})

onBeforeUnmount(() => {
    unlisten()
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
        folder.value = ""
        localStorage.removeItem("folder")
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
const closeFolder = async () => {
    emit("watch-path-changed", {
        type_: -100,
        path: "",
    })
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

const handleClick = (data: any) => {
    currentTab.value = data
    localStorage.setItem("tab", currentTab.value.path)
}

const handleSetTheme = (theme: string) => {
    indexStore.updateConfig({
        theme: theme,
    })
}

const handleOpenFile = async (t: any) => {
    if (t.type !== 1) {
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
        await invoke("write", {
            path: currentTab.value.path,
            content: currentTab.value.content,
        })
        currentTab.value.changed = false
        currentTab.value.updated = Date.now()
        localStorage.setItem("tabs", JSON.stringify(tabs.value))
    }
}
</script>

<template>
    <NConfigProvider :theme="indexStore.theme == 'dark' ? darkTheme : null">
        <div id="container" class="container" :class="indexStore.theme">
            <aside data-tauri-drag-region class="aside"></aside>
            <div class="file-tree-container" :style="`width: ${width}px`">
                <button class="btn" @click="selectFolder">Open</button>
                <button class="btn" @click="closeFolder">Close</button>
                <button class="btn" @click="handleSetTheme('dark')">Dark</button>
                <button class="btn" @click="handleSetTheme('light')">Light</button>
                <div class="file-tree-body">
                    <FileTreeVue
                        v-if="showTree"
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
                            @click="handleClick(item)"
                        >
                            <span class="icon">
                                <component :is="item.type_ === 1 ? Markdown : Word" />
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
                        <EditorVue
                            v-if="tabs.length > 0 && currentTab.name !== ''"
                            :key="key"
                            @handleUpdateFile="handleUpdateFile"
                            :value="currentTab"
                        />
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
}
#container.dark {
    .tab-view-container {
        .tab {
            .tab-bar {
                .tab-bar-item {
                    &.active {
                        svg.md path {
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
    .tab-view-container {
        .tab {
            .tab-bar {
                .tab-bar-item {
                    &.active {
                        svg.md path {
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
.dark .btn {
    border: none;
    background: none;
    background-color: #1d2125;
    color: #eee;
    padding: 10px 20px;
    margin: 6px 2px;
    cursor: pointer;
}

.dark .btn:hover {
    background-color: #2d2f33;
}

.light .btn {
    border: none;
    background: none;
    background-color: #f3f6f8;
    color: #333;
    padding: 10px 20px;
    margin: 10px 2px;
    cursor: pointer;
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
        width: 60px;
        cursor: move;
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
                top: 46px;
                left: 0;
                right: 0;
                bottom: 0;
                overflow: hidden;
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
