import type { Config } from '@/types'
import { defineStore } from 'pinia'

const defaultConfig = () => {
    let config = localStorage.getItem('config')
    if (config) {
        return JSON.parse(config) as Config
    }
    let default_config = { theme: 'dark' }
    localStorage.setItem('config', JSON.stringify(default_config))
    return default_config
}

export const useIndexStore = defineStore<
    'index',
    {
        config: Config
    },
    {
        theme(state: any): string
    },
    {
        updateConfig(config: { [x: string]: any }): Promise<void>
    }
>({
    id: 'index',
    state: () => {
        return {
            config: defaultConfig()
        }
    },
    getters: {
        theme(state) {
            return state.config.theme || 'dark'
        }
    },
    actions: {
        async updateConfig(config: Config) {
            let c = { ...this.config, ...config }
            localStorage.setItem('config', JSON.stringify(c))
            this.config = c
        }
    }
})
