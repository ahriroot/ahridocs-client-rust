import type { Config } from '@/types'
import { defineStore } from 'pinia'

const defaultConfig = (): Config => {
    let config: Config = { theme: 'dark', mdMode: 'wysiwyg' }
    let configStr = localStorage.getItem('config')
    if (configStr) {
        try {
            let tmp = JSON.parse(configStr) as Config
            config = { ...config, ...tmp }
        } catch (e) {
            console.error(e)
        }
    }
    localStorage.setItem('config', JSON.stringify(config))
    return config
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
