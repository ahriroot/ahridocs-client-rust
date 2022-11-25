export interface Config {
    theme: string
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
