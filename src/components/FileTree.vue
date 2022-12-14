<script setup lang="ts">
import { h, ref, onBeforeMount, VNodeChild } from "vue"
import { emit } from "@tauri-apps/api/event"

import { NTree, TreeOption, NIcon, NDropdown } from "naive-ui"
import { ChevronForward } from "@vicons/ionicons5"
import { invoke } from "@tauri-apps/api/tauri"

import type { FileTree } from "@/types"
import { TreeRenderProps } from "naive-ui/es/tree/src/interface"

import Markdown from "@/components/icons/md.vue"
import Word from "@/components/icons/word.vue"

const props = defineProps<{ value: FileTree[]; theme: string }>()
const emits = defineEmits<{
    (e: "handleOpenFile", val: any): void
}>()

const data = ref<TreeOption[] | undefined>()
const defaultExpandedKeys = ref<string[]>([])
const handleUpdateExpandedKeys = (keys: any[]) => {
    defaultExpandedKeys.value = keys
    localStorage.setItem("defaultExpandedKeys", JSON.stringify(keys))
}

const handleClose = () => {
    defaultExpandedKeys.value = []
    data.value = undefined
}

const tree2array = (
    tree: any,
    parent: string,
    base: string
): { path: string; fullpath: string; parent: string }[] => {
    let arr: { path: string; fullpath: string; parent: string }[] = []
    tree.forEach((node: any) => {
        let p = node.key
            .replace(base, "")
            .split(/\\|\//)
            .filter((s: string) => s !== "")
            .join("/")
        if (node.type === 0) {
            arr.push({ path: p, fullpath: node.key, parent: parent })
            arr = arr.concat(tree2array(node.children, p, base))
        } else {
            arr.push({ path: p, fullpath: node.key, parent: parent })
        }
    })
    return arr
}

const handleExpand = async () => {
    let folder: string = localStorage.getItem("folder") || ""
    let base = folder
    const config = await invoke<{ type_: number; path: string }>("get_config", {
        path: base
    })
    console.log(config)
    let res = tree2array(data.value, "/", base)

    // documents\\children\\1.ahtml -> [documents,children,1.ahtml]
    // documents/children/1.ahtml -> [documents,children,1.ahtml]
    // let result = res.map(r => r.join('/'))
    console.log(res)
}

defineExpose({
    handleClose,
    handleExpand,
})

const renderSwitcherIcon = () => h(NIcon, null, { default: () => h(ChevronForward) })

const renderLabel = ({ option }: TreeRenderProps): VNodeChild => {
    const { label } = option
    let icon
    if (option.type === 1) {
        icon = h(Markdown)
    } else if (option.type === 2) {
        icon = h(Word)
    } else {
    }
    return h("span", { class: "tree-label", title: label }, [icon, label])
}

const fileToNode = (file: FileTree): TreeOption => {
    const { name, type_, path, children } = file
    let c = undefined
    if (type_ === 0) {
        c = children?.map(fileToNode)
    }
    return {
        label: name,
        name: name,
        key: path,
        type: type_,
        edit: false,
        children: c,
    }
}

onBeforeMount(async () => {
    let dks = localStorage.getItem("defaultExpandedKeys")
    if (dks) {
        defaultExpandedKeys.value = JSON.parse(dks)
    }
    data.value = props.value.map(fileToNode)
})

const create = async (path: string, name: string, is_dir: boolean = false) => {
    const res = await invoke("create", {
        path: path,
        name: name,
        isDir: is_dir
    })
    if (res) {
        emit("file-system-changed", {
            type_: is_dir ? 1 : 2,
        })
    }
}

const delete_ = async (path: string, is_dir: boolean = false) => {
    const res = await invoke("delete", {
        path: path,
        isDir: is_dir
    })
    if (res) {
        emit("file-system-changed", {
            type_: is_dir ? -1 : -2,
            path: path
        })
    }
}

const showContextmenu = ref(false)
const optionsContextmenu = ref<any[]>([])
const xPos = ref(0)
const yPos = ref(0)
const handleContextmenu = (e: MouseEvent) => {
    e.preventDefault()
    e.stopPropagation()
    xPos.value = e.clientX
    yPos.value = e.clientY
    optionsContextmenu.value = [{
        label: 'Create Markdown',
        key: 'create_markdown',
        props: {
            onClick: async () => {
                console.log('root')
                showContextmenu.value = false
            }
        }
    }, {
        label: 'Create AHtml',
        key: 'create_ahtml',
        props: {
            onClick: async () => {
                console.log('root')
                showContextmenu.value = false
            }
        }
    }, {
        label: 'Create Folder',
        key: 'create_folder',
        props: {
            onClick: async () => {
                console.log('root')
                showContextmenu.value = false
            }
        }
    }]
    showContextmenu.value = true
}
const nodeProps = ({ option }: { option: any }): any => {
    return {
        onClick() { },
        onDblclick() {
            if (option.type !== 0) {
                emits("handleOpenFile", option)
            }
        },
        onContextmenu(e: MouseEvent): void {
            e.preventDefault()
            e.stopPropagation()
            switch (option.type) {
                case 0:
                    optionsContextmenu.value = [{
                        label: 'Create Markdown',
                        key: 'create_markdown',
                        props: {
                            onClick: async () => {
                                await create(option.key, 'new.md')
                                showContextmenu.value = false
                            }
                        }
                    }, {
                        label: 'Create AHtml',
                        key: 'create_ahtml',
                        props: {
                            onClick: async () => {
                                console.log(option)
                                showContextmenu.value = false
                            }
                        }
                    }, {
                        label: 'Create Folder',
                        key: 'create_folder',
                        props: {
                            onClick: async () => {
                                console.log('root')
                                showContextmenu.value = false
                            }
                        }
                    }, {
                        label: 'Rename',
                        key: 'rename',
                        props: {
                            onClick: async () => {
                                console.log(option)
                                showContextmenu.value = false
                            }
                        }
                    }, {
                        label: 'Delete',
                        key: 'delete',
                        props: {
                            onClick: async () => {
                                console.log(option)
                                showContextmenu.value = false
                            }
                        }
                    }]
                    break
                default:
                    optionsContextmenu.value = [{
                        label: 'Open',
                        key: 'open',
                        props: {
                            onClick: async () => {
                                console.log(option)
                                showContextmenu.value = false
                            }
                        }
                    }, {
                        label: 'Rename',
                        key: 'rename',
                        props: {
                            onClick: async () => {
                                console.log(option)
                                showContextmenu.value = false
                            }
                        }
                    }, {
                        label: 'Delete',
                        key: 'delete',
                        props: {
                            onClick: async () => {
                                await delete_(option.key)
                                showContextmenu.value = false
                            }
                        }
                    }]
                    break
            }
            xPos.value = e.clientX
            yPos.value = e.clientY
            showContextmenu.value = true
        },
    }
}
</script>

<template>
    <div class="file-tree" :class="props.theme" @contextmenu="handleContextmenu">
        <n-dropdown trigger="manual" size="small" placement="bottom-start" :show="showContextmenu"
            :options="(optionsContextmenu as any)" :x="xPos" :y="yPos" @clickoutside="showContextmenu = false" />
        <n-tree block-line :data="data" :default-expanded-keys="defaultExpandedKeys" expand-on-click
            :render-switcher-icon="renderSwitcherIcon" :render-label="renderLabel"
            @update-expanded-keys="handleUpdateExpandedKeys" :node-props="nodeProps" />
    </div>
</template>

<style scoped lang="scss">
.file-tree {
    height: 100%;
    padding: 10px;

    :deep(.tree-label) {
        display: flex;
        align-items: center;
        gap: 5px;
        overflow: hidden;
        text-overflow: ellipsis;
        white-space: nowrap;
        user-select: none;

        svg path {
            fill: #c6d5da !important;
        }
    }
}

.file-tree.dark {
    color: #c6d5da;

    :deep(.tree-label) {
        svg path {
            fill: #c6d5da !important;
        }
    }
}

.file-tree.light {
    color: #24292e;

    :deep(.tree-label) {
        svg path {
            fill: #24292e !important;
        }
    }
}
</style>
