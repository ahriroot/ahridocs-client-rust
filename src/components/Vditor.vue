<script setup lang="ts">
import { ref, watch, onMounted, computed } from "vue"
import { useIndexStore } from "@/store"

import { convertFileSrc } from "@tauri-apps/api/tauri"

import Vditor from "vditor"
import "vditor/dist/index.css"

import { DocFile } from "@/types"

const props = defineProps<{ value: DocFile; mode: "ir" | "wysiwyg" | "sv" | undefined }>()
const emits = defineEmits<{
    (e: "handleUpdateFile", val: { path: string; content: string }): void
}>()

const indexStore = useIndexStore()
const vditor = ref<Vditor | null>(null)
const display = computed(() => (indexStore.showMdToolbar ? "block" : "none"))

const getTheme = () => {
    let theme: "dark" | "classic" = indexStore.theme == "dark" ? "dark" : "classic"
    let content = indexStore.theme == "dark" ? "dark" : "light"
    let code = indexStore.theme == "dark" ? "native" : "github"
    return {
        theme,
        content,
        code,
    }
}

watch(
    () => indexStore.theme,
    () => {
        let { theme, content, code } = getTheme()
        vditor?.value?.setTheme(theme, content, code)
    }
)

watch(
    () => props.value.path,
    () => {
        vditor?.value?.setValue(props.value.content)
    }
)

const elementRef = ref<HTMLElement | null>(null)
onMounted(() => {
    if (elementRef.value) {
        let toolbar: string[] = []
        for (let key in indexStore.config.mdToolbar) {
            if (indexStore.config.mdToolbar[key]) {
                toolbar.push(key)
            }
        }

        let mode: "ir" | "wysiwyg" | "sv" | undefined = props.mode
        if (mode == undefined) {
            mode = "ir"
        }
        let { theme, content, code } = getTheme()
        vditor.value = new Vditor(elementRef.value, {
            mode: mode,
            placeholder: "请输入内容",
            toolbar: toolbar,
            tab: "    ",
            input(value: string) {
                emits("handleUpdateFile", { path: props.value.path, content: value })
            },
            height: "100%",
            width: "100%",
            theme: theme,
            upload: {
                handler(files: File[]) {
                    console.log(files)
                    // const assetUrl = convertFileSrc(filePath)
                    return ""
                },
            },
            preview: {
                mode: "both",
                theme: {
                    current: content,
                    list: {
                        "ant-design": "Ant Design",
                        dark: "Dark",
                        light: "Light",
                        wechat: "WeChat",
                    },
                    path: "/content-theme",
                },
                hljs: {
                    enable: true,
                    style: code,
                    lineNumber: true,
                },
            },
            cache: {
                enable: false,
                id: "cache",
            },
            cdn: "/vditor",
            after() {
                vditor.value?.setValue(props.value.content)
            },
        })
    }
})
</script>

<template>
    <div id="vditor">
        <div ref="elementRef" />
    </div>
</template>

<style scoped>
#vditor {
    position: absolute;
    top: 0;
    left: 0;
    right: -17px;
    bottom: 0;
    overflow: hidden;
}

:deep(.vditor-toolbar) {
    display: v-bind(display);
}
</style>
