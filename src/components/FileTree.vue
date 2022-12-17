<script setup lang="ts">
import { h, ref, onBeforeMount, VNodeChild, computed } from "vue"
import { emit } from "@tauri-apps/api/event"

import { NTree, TreeOption, NIcon, NDropdown, useDialog } from "naive-ui"
import { ChevronForward } from "@vicons/ionicons5"
import { invoke } from "@tauri-apps/api/tauri"

import type { FileTree, Response, ProjectConfig } from "@/types"
import { TreeRenderProps } from "naive-ui/es/tree/src/interface"

import AInputFocus from './AInputFocus.vue'
import Markdown from "@/components/icons/md.vue"
import Word from "@/components/icons/word.vue"

const props = defineProps<{ value: FileTree[]; theme: string }>()
const emits = defineEmits<{
    (e: "handleOpenFile", val: any): void
}>()

const dialog = useDialog()
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
        let p = node.path
            .replace(base, "")
            .split(/\\|\//)
            .filter((s: string) => s !== "")
            .join("/")
        if (node.type === 0) {
            arr.push({ path: p, fullpath: node.path, parent: parent })
            arr = arr.concat(tree2array(node.children, p, base))
        } else {
            arr.push({ path: p, fullpath: node.path, parent: parent })
        }
    })
    return arr
}

/**
 * @description: tree to array
 */
const handleExpand = async () => {
    let folder: string = localStorage.getItem("folder") || ""
    let base = folder
    let result = await invoke<Response<ProjectConfig>>("get_config", {
        path: base
    })
    if (result.code !== 10000) {
        alert(result.msg)
        return
    }
    let config = result.data
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
    if (option.create) {
        return h(AInputFocus, {
            value: option.label as string,
            onUpdateValue: (val: any) => {
                option.label = val
                option.name = val
            },
            onBlur: async () => {
                if (option.label === '') {
                    // delete this node
                    const parent: TreeOption = option.parent as TreeOption
                    if (parent) {
                        const index = parent.children?.indexOf(option)
                        if (index !== undefined) {
                            parent.children?.splice(index, 1)
                        }
                    }
                } else {
                    let name = option.label as string
                    if (option.type === 1) {
                        if (!name.endsWith(".md")) {
                            if (name.endsWith(".")) {
                                name = name.slice(0, name.length - 1)
                            }
                            name += ".md"
                        }
                    } else if (option.type === 2) {
                        if (!name.endsWith(".ahtml")) {
                            if (name.endsWith(".")) {
                                name = name.slice(0, name.length - 1)
                            }
                            name += ".ahtml"
                        }
                    }
                    let res = await create(option.path as string, name, option.type === 0)
                    if (res) {
                        if (option.type === 1 || option.type === 2) {
                            emits("handleOpenFile", {
                                type: option.type,
                                name: name,
                                path: res.path,
                            })
                        }
                    } else {
                        const parent: TreeOption = option.parent as TreeOption
                        if (parent) {
                            const index = parent.children?.indexOf(option)
                            if (index !== undefined) {
                                parent.children?.splice(index, 1)
                            }
                        }
                    }
                    option.name = option.label as string
                }
                option.edit = false
            }
        })
    } else if (option.edit) {
        return h(AInputFocus, {
            value: option.label as string,
            onUpdateValue: (val: any) => {
                option.label = val
            },
            onBlur: async () => {
                if (option.label !== option.name) {
                    option.name = option.label as string
                    const res = await invoke("rename", {
                        path: option.path,
                        name: option.label as string,
                    })
                    if (res) {
                        emit("file-system-changed", {
                            type_: 5,
                        })
                    } else {
                        console.log("error rename" + option.label)
                    }
                }
                option.edit = false
            }
        })
    } else {
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
}

const fileToNode = (file: FileTree, parent: TreeOption | undefined): TreeOption => {
    const { name, type_, path, children } = file
    let node: TreeOption = {
        label: name,
        name: name,
        path: path,
        key: path,
        type: type_,
        edit: false,
        create: false,
        parent: parent,
    }
    if (type_ === 0) {
        node.children = children?.map((child) => fileToNode(child, node))
    }
    return node
}

onBeforeMount(async () => {
    let dks = localStorage.getItem("defaultExpandedKeys")
    if (dks) {
        defaultExpandedKeys.value = JSON.parse(dks)
    }
    let folder: string = localStorage.getItem("folder") || ""
    let dt = [{
        type_: 0,
        name: "root",
        path: folder,
        updated: 0,
        children: props.value
    }]
    data.value = dt.map((file) => fileToNode(file, undefined))
})

const create = async (path: string, name: string, is_dir: boolean = false): Promise<{ content: string; path: string; type_: number; updated: number } | null> => {
    const res = await invoke<Response<{ content: string; path: string; type_: number; updated: number } | null>>("create", {
        path: path,
        name: name,
        isDir: is_dir
    })
    if (res) {
        emit("file-system-changed", {
            type_: is_dir ? 1 : 2,
        })
    }
    if (res.code !== 10000) {
        alert(res.msg)
    }
    return res.data
}

const delete_ = async (path: string, is_dir: boolean = false) => {
    const res = await invoke<Response<boolean>>("delete", {
        path: path,
        isDir: is_dir
    })
    if (res.code !== 10000) {
        alert(res.msg)
        return
    }
    emit("file-system-changed", {
        type_: is_dir ? -1 : -2,
        path: path
    })
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
                if (data.value) {
                    let option = data.value[0]
                    option.children = option.children || []
                    let i = 0
                    for (; i < option.children.length; i++) {
                        if (option.children[i].type !== 0) {
                            break
                        }
                    }
                    option.children.splice(i, 0, {
                        label: '',
                        name: '',
                        path: option.path,
                        key: option.key + '#new',
                        type: 1,
                        edit: false,
                        create: true,
                        children: [],
                        parent: option
                    })
                    showContextmenu.value = false
                }
            }
        }
    }, {
        label: 'Create AHtml',
        key: 'create_ahtml',
        props: {
            onClick: async () => {
                if (data.value) {
                    let option = data.value[0]
                    option.children = option.children || []
                    let i = 0
                    for (; i < option.children.length; i++) {
                        if (option.children[i].type !== 0) {
                            break
                        }
                    }
                    option.children.splice(i, 0, {
                        label: '',
                        name: '',
                        path: option.path,
                        key: option.key + '#new',
                        type: 2,
                        edit: false,
                        create: true,
                        children: [],
                        parent: option
                    })
                    showContextmenu.value = false
                }
                showContextmenu.value = false
            }
        }
    }, {
        label: 'Create Folder',
        key: 'create_folder',
        props: {
            onClick: async () => {
                if (data.value) {
                    let option = data.value[0]
                    option.children = option.children || []
                    option.children.unshift({
                        label: '',
                        name: '',
                        path: option.path,
                        key: option.key + '#new',
                        type: 0,
                        edit: false,
                        create: true,
                        children: [],
                        parent: option
                    })
                    showContextmenu.value = false
                }
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
                                option.children = option.children || []
                                let i = 0
                                for (; i < option.children.length; i++) {
                                    if (option.children[i].type !== 0) {
                                        break
                                    }
                                }
                                option.children.splice(i, 0, {
                                    label: '',
                                    name: '',
                                    path: option.path,
                                    key: option.key + '#new',
                                    type: 1,
                                    edit: false,
                                    create: true,
                                    children: [],
                                    parent: option
                                })
                                showContextmenu.value = false
                            }
                        }
                    }, {
                        label: 'Create AHtml',
                        key: 'create_ahtml',
                        props: {
                            onClick: async () => {
                                option.children = option.children || []
                                let i = 0
                                for (; i < option.children.length; i++) {
                                    if (option.children[i].type !== 0) {
                                        break
                                    }
                                }
                                option.children.splice(i, 0, {
                                    label: '',
                                    name: '',
                                    path: option.path,
                                    key: option.key + '#new',
                                    type: 2,
                                    edit: false,
                                    create: true,
                                    children: [],
                                    parent: option
                                })
                                showContextmenu.value = false
                            }
                        }
                    }, {
                        label: 'Create Folder',
                        key: 'create_folder',
                        props: {
                            onClick: async () => {
                                option.children = option.children || []
                                option.children.unshift({
                                    label: '',
                                    name: '',
                                    path: option.path,
                                    key: option.key + '#new',
                                    type: 0,
                                    edit: false,
                                    create: true,
                                    children: [],
                                    parent: option
                                })
                                showContextmenu.value = false
                            }
                        }
                    }, {
                        label: 'Rename',
                        key: 'rename',
                        props: {
                            onClick: async () => {
                                option.edit = true
                                showContextmenu.value = false
                            }
                        }
                    }, {
                        label: 'Delete',
                        key: 'delete',
                        props: {
                            onClick: async () => {
                                dialog.warning({
                                    title: '删除',
                                    content: `确定删除 ${option.label} ？`,
                                    positiveText: '删除',
                                    negativeText: '取消',
                                    onPositiveClick: async () => {
                                        await delete_(option.path, true)
                                    }
                                })
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
                                emits("handleOpenFile", option)
                                showContextmenu.value = false
                            }
                        }
                    }, {
                        label: 'Rename',
                        key: 'rename',
                        props: {
                            onClick: async () => {
                                option.edit = true
                                showContextmenu.value = false
                            }
                        }
                    }, {
                        label: 'Delete',
                        key: 'delete',
                        props: {
                            onClick: async () => {
                                dialog.warning({
                                    title: '删除',
                                    content: `确定删除 ${option.label} ？`,
                                    positiveText: '删除',
                                    negativeText: '取消',
                                    onPositiveClick: async () => {
                                        await delete_(option.path)
                                    }
                                })
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
const d = computed<TreeOption[] | undefined>(() => data.value ? data.value[0].children : undefined)
</script>

<template>
    <div class="file-tree" :class="props.theme" @contextmenu="handleContextmenu">
        <n-dropdown trigger="manual" size="small" placement="bottom-start" :show="showContextmenu"
            :options="(optionsContextmenu as any)" :x="xPos" :y="yPos" @clickoutside="showContextmenu = false" />
        <n-tree block-line :data="d" :default-expanded-keys="defaultExpandedKeys"
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
