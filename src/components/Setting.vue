<script setup lang="ts">
import { ref, onBeforeMount } from "vue"
import { useIndexStore } from "@/store"
import { invoke } from "@tauri-apps/api/tauri"

import type { Config } from "@/types"

const props = defineProps<{ value: undefined; mode: undefined }>()
const emits = defineEmits<{
    (e: "handleUpdateFile", val: { path: string; content: string }): void
}>()

const indexStore = useIndexStore()

const config = ref<Config>({ ...indexStore.config })

const project_config = ref<{ project: string; token: string }>({
    token: "",
    project: "",
})

onBeforeMount(async () => {
    let folder: string = localStorage.getItem("folder") || ""
    let base = folder
    project_config.value = await invoke<{ project: string; token: string }>("get_config", {
        path: base,
    })
})

const handleSetTheme = async (theme: string) => {
    config.value.theme = theme
    await indexStore.updateConfig(config.value)
}

const handleSetMdToolbar = async (key: string) => {
    config.value.mdToolbar[key] = !config.value.mdToolbar[key]
    await indexStore.updateConfig(config.value)
}

const handleSetAHtmlToolbar = async (key: string) => {
    config.value.ahtmlToolbar[key] = !config.value.ahtmlToolbar[key]
    await indexStore.updateConfig(config.value)
}

const handleSetMdMode = async (mode: "ir" | "sv" | "wysiwyg") => {
    config.value.mdMode = mode
    await indexStore.updateConfig(config.value)
}

const handlePrjojectChanged = async () => {
    await invoke("set_config", {
        path: localStorage.getItem("folder") || "",
        config: {
            project: project_config.value.project,
            token: project_config.value.token,
        },
    })
}
</script>

<template>
    <div id="settings" class="nocopy">
        <h2>Project:</h2>
        <div class="config-value">
            <h3>Token: &nbsp;</h3>
            <div class="config-input">
                <input type="text" v-model="project_config.token" @blur="handlePrjojectChanged" />
            </div>
        </div>
        <div class="config-value">
            <h3>Project: &nbsp;</h3>
            <div class="config-input">
                <input type="text" v-model="project_config.project" @blur="handlePrjojectChanged" />
            </div>
        </div>
        <h2>Theme:</h2>
        <div class="config-value">
            <div
                class="config-radio"
                :class="{ active: config.theme === 'dark' }"
                @click="handleSetTheme('dark')"
            >
                <p>Dark</p>
            </div>
            <div
                class="config-radio"
                :class="{ active: config.theme === 'light' }"
                @click="handleSetTheme('light')"
            >
                <p>Light</p>
            </div>
        </div>
        <h2>Markdown:</h2>
        <div class="config-value">
            <h3>Mode: &nbsp;</h3>
            <div
                class="config-radio"
                :class="{ active: config.mdMode === 'ir' }"
                @click="handleSetMdMode('ir')"
            >
                <p>ir</p>
            </div>
            <div
                class="config-radio"
                :class="{ active: config.mdMode === 'sv' }"
                @click="handleSetMdMode('sv')"
            >
                <p>sv</p>
            </div>
            <div
                class="config-radio"
                :class="{ active: config.mdMode === 'wysiwyg' }"
                @click="handleSetMdMode('wysiwyg')"
            >
                <p>wysiwyg</p>
            </div>
        </div>
        <div class="config-value">
            <h3>Toolbar: &nbsp;</h3>
            <div
                v-for="(item, key) in config.mdToolbar"
                class="config-radio"
                :class="{ active: item }"
                @click="handleSetMdToolbar(key as string)"
            >
                <p>{{ key }}</p>
            </div>
        </div>
        <h2>AHtml:</h2>
        <div class="config-value">
            <h3>Toolbar: &nbsp;</h3>
            <div
                v-for="(item, key) in config.ahtmlToolbar"
                class="config-radio"
                :class="{ active: item }"
                @click="handleSetAHtmlToolbar(key as string)"
            >
                <p>{{ key }}</p>
            </div>
        </div>
    </div>
</template>

<style scoped lang="scss">
#settings {
    position: absolute;
    top: 0;
    left: 0;
    right: -17px;
    bottom: 0;
    overflow: hidden;
    display: flex;
    flex-direction: column;
    padding: 10px 40px;

    h3 {
        margin: 0;
        margin-left: 20px;
    }

    .config-value {
        display: flex;
        flex-wrap: wrap;
        justify-content: flex-start;
        align-items: center;
        padding-left: 20px;
        margin-left: 20px;

        .config-radio {
            padding: 5px 10px;
            border-radius: 5px;
            cursor: pointer;
            transition: all 0.2s ease-in-out;
        }
        .config-input {
            input {
                width: 200px;
                height: 26px;
                padding: 0 10px;
                outline: none;
                background: none;
                border: none;
            }
        }
    }
}

.dark {
    #settings {
        .config-radio {
            &:hover {
                color: #fff;
            }

            &.active {
                background-color: #1d2125;
            }
        }
        .config-input {
            input {
                color: #d7d8d9;
                border-bottom: 2px solid #d7d8d9;
            }
        }
    }
}

.light {
    #settings {
        .config-radio {
            &:hover {
                color: #000;
            }

            &.active {
                background-color: #f3f6f8;
            }
        }
        .config-input {
            input {
                color: #000;
                border-bottom: 2px solid #000;
            }
        }
    }
}
</style>
