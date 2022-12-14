<script setup lang="ts">
import { ref, shallowRef, onBeforeMount, onBeforeUnmount, computed } from "vue"

import { useIndexStore } from "@/store"

import { Editor, Toolbar } from "@wangeditor/editor-for-vue"
import "@wangeditor/editor/dist/css/style.css"

import { DocFile } from "@/types"
import { IDomEditor } from "@wangeditor/editor"

const props = defineProps<{ value: DocFile; mode: "simple" | "default" }>()
const emits = defineEmits<{
    (e: "handleUpdateFile", val: { path: string; content: string }): void
}>()

const indexStore = useIndexStore()

const init = ref(false)
const editorRef = shallowRef()
const valueHtml = ref(props.value.content)
const display = computed(() => (indexStore.showAhtmlToolbar ? "block" : "none"))

const getToolbarKeys = (): string[] => {
    let toolbar: string[] = []
    for (let key in indexStore.config.ahtmlToolbar) {
        if (indexStore.config.ahtmlToolbar[key]) {
            toolbar.push(key)
        }
    }
    return toolbar
}

const toolbarConfig = {
    toolbarKeys: getToolbarKeys(),
}
const editorConfig = {
    placeholder: "请输入内容",
}

onBeforeMount(() => {
    if (!["simple", "default"].includes(mode.value)) {
        mode.value = "default"
    }
})

onBeforeUnmount(() => {
    const editor = editorRef.value
    if (editor == null) return
    editor.destroy()
})

const mode = ref<"simple" | "default">(props.mode)

const handleCreated = (editor: any) => {
    editorRef.value = editor
    init.value = true
}

const handleChange = (editor: IDomEditor) => {
    let html = editor.getHtml()
    if ((init.value, html != props.value.content)) {
        emits("handleUpdateFile", { path: props.value.path, content: html })
    }
}
</script>

<template>
    <div id="wanteditor" :class="indexStore.theme">
        <Toolbar class="toolbar" :editor="editorRef" :defaultConfig="toolbarConfig" :mode="mode" />
        <Editor
            style="height: 100%; overflow-y: hidden"
            v-model="valueHtml"
            :defaultConfig="editorConfig"
            :mode="mode"
            @onCreated="handleCreated"
            @onChange="handleChange"
        />
    </div>
</template>

<style scoped lang="scss">
#wanteditor {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    overflow: hidden;

    .toolbar {
        display: v-bind(display);
        border-top: 1px solid var(--w-e-toolbar-border-color);
    }
}

#wanteditor.dark {
    --w-e-textarea-bg-color: #24292e;
    --w-e-textarea-color: #d1d5da;
    --w-e-textarea-border-color: #ccc;
    --w-e-textarea-slight-border-color: #e8e8e8;
    --w-e-textarea-slight-color: #d1d5da;
    --w-e-textarea-slight-bg-color: #15181b;
    --w-e-textarea-selected-border-color: #15181b; // 选中的元素，如选中了分割线
    --w-e-textarea-handler-bg-color: #15181b; // 工具，如图片拖拽按钮

    // toolbar - css vars
    --w-e-toolbar-color: #d1d5da;
    --w-e-toolbar-bg-color: #1d2125;
    --w-e-toolbar-active-color: #d1d5da;
    --w-e-toolbar-active-bg-color: #444d56;
    --w-e-toolbar-disabled-color: #999;
    --w-e-toolbar-border-color: #444d56;

    // modal - css vars
    --w-e-modal-button-bg-color: #1d2125;
    --w-e-modal-button-border-color: #24292e;
}

#wanteditor.light {
    --w-e-textarea-bg-color: #fff;
    --w-e-textarea-color: #333;
    --w-e-textarea-border-color: #ccc;
    --w-e-textarea-slight-border-color: #e8e8e8;
    --w-e-textarea-slight-color: #d4d4d4;
    --w-e-textarea-slight-bg-color: #f5f2f0;
    --w-e-textarea-selected-border-color: #b4d5ff; // 选中的元素，如选中了分割线
    --w-e-textarea-handler-bg-color: #4290f7; // 工具，如图片拖拽按钮

    // toolbar - css vars
    --w-e-toolbar-color: #595959;
    --w-e-toolbar-bg-color: #f3f6f8;
    --w-e-toolbar-active-color: #333;
    --w-e-toolbar-active-bg-color: #f1f1f1;
    --w-e-toolbar-disabled-color: #999;
    --w-e-toolbar-border-color: #e8e8e8;

    // modal - css vars
    --w-e-modal-button-bg-color: #fafafa;
    --w-e-modal-button-border-color: #d9d9d9;
}
</style>
