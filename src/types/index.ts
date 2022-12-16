export interface Config {
    theme: string
    watch: boolean
    mdMode: "ir" | "sv" | "wysiwyg"
    mdToolbar: { [x: string]: boolean }
    ahtmlToolbar: { [x: string]: boolean }
}
export interface FileTree {
    type_: number
    name: string
    path: string
    updated: number
    children: FileTree[]
}

export interface DocFile {
    type_: number
    name: string
    path: string
    updated: number
    content: string
    changed: boolean
}
