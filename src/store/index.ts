import type { Config } from '@/types'
import { defineStore } from 'pinia'

const defaultConfig = (): Config => {
    let config: Config = {
        theme: 'dark',
        mdMode: 'wysiwyg',
        mdToolbar: {
            "headings": false,
            "bold": false,
            "italic": false,
            "strike": false,
            "emoji": false,
            "line": false,
            "quote": false,
            "list": false,
            "ordered-list": false,
            "check": false,
            "outdent": false,
            "indent": false,
            "code": false,
            "inline-code": false,
            "insert-after": false,
            "insert-before": false,
            "undo": false,
            "redo": false,
            "upload": false,
            "link": false,
            "table": false,
            "both": false,
            "preview": false,
            "fullscreen": false,
            "outline": false,
            "code-theme": false,
        },
        ahtmlToolbar: {
            "headerSelect": true,
            "bold": true,
            "underline": true,
            "italic": true,
            "through": true,
            "clearStyle": true,
            "color": true,
            "bgColor": true,
            "fontSize": true,
            "fontFamily": true,
            "indent": true,
            "delIndent": true,
            "justifyLeft": true,
            "justifyRight": true,
            "justifyCenter": true,
            "justifyJustify": true,
            "lineHeight": true,
            "sub": true,
            "sup": true,
            "code": true,
            "insertImage": false,
            "deleteImage": false,
            "editImage": false,
            "divider": false,
            "emotion": false,
            "insertLink": false,
            "editLink": false,
            "unLink": false,
            "viewLink": false,
            "codeBlock": false,
            "blockquote": false,
            "todo": false,
            "redo": false,
            "undo": false,
            "enter": false,
            "bulletedList": false,
            "numberedList": false,
            "insertTable": false,
            "deleteTable": false,
            "insertTableRow": false,
            "deleteTableRow": false,
            "insertTableCol": false,
            "deleteTableCol": false,
            "tableHeader": false,
            "tableFullWidth": false,
            "insertVideo": false,
            "uploadVideo": false,
            "editVideoSize": false,
            "uploadImage": false,
            "codeSelectLang": false,
        }
    }
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
        showMdToolbar(state: any): boolean
        showAhtmlToolbar(state: any): boolean
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
        },
        showMdToolbar(state) {
            return Object.values(state.config.mdToolbar).some((v) => v)
        },
        showAhtmlToolbar(state) {
            return Object.values(state.config.ahtmlToolbar).some((v) => v)
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
