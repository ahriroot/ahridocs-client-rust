<script setup lang="ts">
import { ref, onBeforeMount, onBeforeUnmount } from "vue"

import { useIndexStore } from "@/store"

import { listen, UnlistenFn, emit } from "@tauri-apps/api/event"
import { invoke } from "@tauri-apps/api/tauri"

import FileTreeVue from "@/components/FileTree.vue"
import EditorVue from "@/components/Editor.vue"
import Markdown from "@/components/icons/md.vue"
import Point from "@/components/icons/point.vue"
import Close from "@/components/icons/close.vue"
import { DocFile } from "@/types"
import { t } from "@tauri-apps/api/event-2a9960e7"

const indexStore = useIndexStore()

let unlisten: UnlistenFn
onBeforeMount(async () => {
    unlisten = await listen("file-system-changed", event => {
        console.log(event)
    })
})

onBeforeUnmount(() => {
    unlisten()
})

const folder = ref("")

const openFolder = async () => {
    const open = await invoke<{
        type_: number
        path: string
    }>("open")
    if (open.type_ === 0) {
        folder.value = open.path
        emit("watch-path-changed", open)
    } else {
        alert("No folder selected")
    }
}
const closeFolder = async () => {
    emit("watch-path-changed", {
        type_: -100,
        path: "",
    })
}

const tabs = ref<DocFile[]>([
    {
        name: "First.md",
        label: "First.md",
        content: "# 123",
        changed: true,
    },
    {
        name: "Second.md",
        label: "Second.md",
        content: "# 456",
        changed: false,
    },
    {
        name: "1First.md",
        label: "First.md",
        content: "# 123",
        changed: false,
    },
    {
        name: "2Second.md",
        label: "Second.md",
        content: "# 456",
        changed: false,
    },
    {
        name: "11First.md",
        label: "First.md",
        content: "# 123",
        changed: false,
    },
    {
        name: "22Second.md",
        label: "Second.md",
        content: "# 456",
        changed: false,
    },
    {
        name: "111First.md",
        label: "First.md",
        content: "# 123",
        changed: false,
    },
    {
        name: "222Second.md",
        label: "Second.md",
        content: "# 456",
        changed: false,
    },
    {
        name: "1111First.md",
        label: "First.md",
        content: "# 123",
        changed: false,
    },
    {
        name: "2222Second.md",
        label: "Second.md",
        content: "# 456",
        changed: false,
    },
    {
        name: "11111First.md",
        label: "First.md",
        content: "# 123",
        changed: false,
    },
    {
        name: "22222Second.md",
        label: "Second.md",
        content: "# 456",
        changed: false,
    },
    {
        name: "111111First.md",
        label: "First.md",
        content: "# 123",
        changed: false,
    },
    {
        name: "222222Second.md",
        label: "Second.md",
        content: "# 456",
        changed: false,
    },
    {
        name: "1111111First.md",
        label: "First.md",
        content: "# 123",
        changed: false,
    },
    {
        name: "2222222Second.md",
        label: "Second.md",
        content: "# 456",
        changed: false,
    },
])
const currentTab = ref<DocFile>(tabs.value[0])

const handleClick = (data: any) => {
    currentTab.value = data
}

const handleSetTheme = (theme: string) => {
    indexStore.updateConfig({
        theme: theme,
    })
}
</script>

<template>
    <div id="container" class="container" :class="indexStore.theme">
        <aside data-tauri-drag-region class="aside">
            <!-- <button>Open</button> -->
        </aside>
        <div class="file-tree-container">
            <button class="btn" @click="openFolder">Open</button>
            <button class="btn" @click="closeFolder">Close</button>
            <button class="btn" @click="handleSetTheme('dark')">Dark</button>
            <button class="btn" @click="handleSetTheme('light')">Light</button>
            {{ indexStore.theme }}
        </div>
        <div class="split"></div>
        <div class="tab-view-container">
            <div class="tab" :class="indexStore.theme">
                <div class="tab-bar nocopy">
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
                        <span class="icon"><Markdown /></span>
                        <span class="name">{{ item.label }}</span>
                        <span class="close-btn">
                            <Point class="point" />
                            <Close class="close" />
                        </span>
                    </div>
                    <div class="bg"></div>
                </div>
                <div class="tab-panel">
                    <EditorVue :value="currentTab" />
                </div>
            </div>
        </div>
    </div>
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
                            fill: #b6c4c8;
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
                        svg path {
                            fill: #24292e;
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
    margin: 10px 2px;
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
        width: 300px;
    }
    .split {
        position: absolute;
        top: 0;
        left: 350px;
        bottom: 0;
        width: 5px;
        cursor: ew-resize;
    }
    .tab-view-container {
        position: absolute;
        top: 0;
        left: 355px;
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
                overflow-x: auto;
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
