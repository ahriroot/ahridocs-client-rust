<script setup lang="ts">
import { ref } from "vue"
import { NSelect } from "naive-ui"

const config = ref({
    mdMode: "ir",
})

const handleChanged = () => {
    console.log(config.value)
}
</script>

<template>
    <div id="setting">
        <div class="item">
            <div class="name">Markdown</div>
            <div class="children-item">
                <div class="label">Mode</div>
                {{ config }}
                <div class="value">
                    <n-select
                        @update:value="handleChanged"
                        v-model:value="config.mdMode"
                        :options="[
                            { label: 'IR', value: 'ir' },
                            { label: 'SV', value: 'sv' },
                            { label: 'WYSIWYG', value: 'wysiwyg' },
                        ]"
                    />
                </div>
            </div>
        </div>
    </div>
</template>

<style scoped>
#setting {
    position: absolute;
    top: 0;
    left: 0;
    right: -17px;
    bottom: 0;
    overflow: hidden;
    display: flex;
    flex-direction: column;
    padding: 10px 40px;
}
</style>
