<script setup lang="ts">
import { h, ref, onBeforeMount, VNodeChild } from "vue"

import { NTree, TreeOption, NIcon, NDropdown } from "naive-ui"
import { ChevronForward } from "@vicons/ionicons5"

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

defineExpose({
    handleClose,
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

const showContextmenu = ref(false)
const optionsContextmenu = ref<any[]>([])
const xPos = ref(0)
const yPos = ref(0)
const nodeProps = ({ option }: { option: any }): any => {
    return {
        onClick() {},
        onDblclick() {
            if (option.type !== 0) {
                emits("handleOpenFile", option)
                showContextmenu.value = false
            }
        },
        onContextmenu(e: MouseEvent): void {
            e.preventDefault()
            e.stopPropagation()
        },
    }
}
</script>

<template>
    <div class="file-tree" :class="props.theme">
        <n-dropdown
            trigger="manual"
            size="small"
            placement="bottom-start"
            :show="showContextmenu"
            :options="(optionsContextmenu as any)"
            :x="xPos"
            :y="yPos"
            @clickoutside="showContextmenu = false"
        />
        <n-tree
            block-line
            :data="data"
            :default-expanded-keys="defaultExpandedKeys"
            expand-on-click
            :render-switcher-icon="renderSwitcherIcon"
            :render-label="renderLabel"
            @update-expanded-keys="handleUpdateExpandedKeys"
            :node-props="nodeProps"
        />
    </div>
</template>

<style scoped lang="scss">
.file-tree {
    padding: 10px;
    color: #c6d5da;
    color: #24292e;

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
