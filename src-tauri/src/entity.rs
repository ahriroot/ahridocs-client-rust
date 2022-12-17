use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Response<T> {
    pub code: i32,
    pub msg: String,
    pub data: T,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FileTree {
    pub type_: i32,
    pub name: String,
    pub path: String,
    pub updated: i64,
    pub children: Option<Vec<FileTree>>,
}

#[derive(Clone, serde::Serialize)]
pub struct Message {
    pub type_: i32,
    pub path: Box<String>,
    pub path2: Box<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct OpenFolder {
    pub type_: i32,
    pub path: Box<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenFile {
    pub type_: i32,
    pub path: String,
    pub content: String,
    pub updated: i64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Config {
    pub token: String,
    pub project: String,
}
