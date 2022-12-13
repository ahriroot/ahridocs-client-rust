export interface Config {
    theme: string
    mdMode: "ir" | "sv" | "wysiwyg"
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
