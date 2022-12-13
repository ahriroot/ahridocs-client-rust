<script setup lang="ts">
import { ref } from "vue"
import { useIndexStore } from "@/store"
import type { Config } from "@/types"

const props = defineProps<{ value: undefined; mode: undefined }>()
const emits = defineEmits<{
    (e: "handleUpdateFile", val: { path: string; content: string }): void
}>()

const indexStore = useIndexStore()

const config = ref<Config>({ ...indexStore.config })

const handleSetTheme = async (theme: string) => {
    config.value.theme = theme
    await indexStore.updateConfig(config.value)
}

const handleSetMdMode = async (mode: "ir" | "sv" | "wysiwyg") => {
    config.value.mdMode = mode
    await indexStore.updateConfig(config.value)
}
</script>

<template>
    <div id="settings" class="nocopy">
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
    }
}
</style>
