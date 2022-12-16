<script setup lang="ts">
import { ref, watch, onMounted } from 'vue'
import { NInput } from 'naive-ui'

const props = withDefaults(defineProps<{
    value: string | null
    bg?: string
    onUpdateValue: (val: string | null) => void
    onBlur?: () => void
}>(), {
    bg: 'none'
})
const emits = defineEmits<{
    // (e: 'update:value', val: string | null): void
}>()

const inputRef = ref<any>(null)
const value = ref<string | null>(props.value)
const handleChange = async (val: string | null) => {
    value.value = val
    // emits('update:value', val)
    props.onUpdateValue(val)
}
const handleClear = async () => {
    value.value = null
    // emits('update:value', null)
    props.onUpdateValue(null)
}
const handleBlur = async () => {
    if (props.onBlur) {
        props.onBlur()
    }
}
const handleFocus = async () => {
    inputRef.value?.select()
}

watch(() => props.value, (val) => {
    value.value = val
})

const handleToBlur = async () => {
    inputRef.value?.blur()
}

onMounted(() => {
    inputRef.value?.focus()
})
</script>
    
<template>
    <n-input ref="inputRef" size="small" :value="value" @update:value="handleChange" @clear="handleClear" @focus="handleFocus"
        :placeholder="value === null ? 'NULL' : ''" @blur="handleBlur" @keyup.enter.native="handleToBlur" clearable />
</template>
    
<style scoped>

</style>
    