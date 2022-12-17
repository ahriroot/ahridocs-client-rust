use std::{fs, path::Path};

use tauri::api::dialog::FileDialogBuilder;

use crate::entity;

pub fn read_dir(path: &Path, depth: i32) -> Result<Vec<entity::FileTree>, std::io::Error> {
    let mut result = Vec::new();
    match std::fs::read_dir(path) {
        Ok(dir) => {
            for entry in dir {
                if depth == 0 && entry.as_ref().unwrap().file_name() == ".ahriknow" {
                    continue;
                }
                let entry = entry.unwrap();
                let path = entry.path();
                let type_;
                if path.is_dir() {
                    type_ = 0
                } else {
                    match path.extension() {
                        None => continue,
                        Some(ext) => {
                            let ext = ext.to_str().unwrap();
                            if ext == "md" {
                                type_ = 1
                            } else if ext == "ahtml" {
                                type_ = 2
                            } else {
                                continue;
                            }
                        }
                    }
                }
                let metadata = std::fs::metadata(&path).unwrap();
                let file_tree = entity::FileTree {
                    type_,
                    name: entry.file_name().to_str().unwrap().to_string(),
                    path: path.to_str().unwrap().to_string(),
                    updated: metadata.modified().unwrap().elapsed().unwrap().as_micros() as i64,
                    children: if path.is_dir() {
                        match read_dir(&path, depth + 1) {
                            Ok(children) => Some(children),
                            Err(e) => return Err(e),
                        }
                    } else {
                        None
                    },
                };
                result.push(file_tree);
            }
        }
        Err(e) => return Err(e),
    }

    result.sort_by(|a, b| a.type_.cmp(&b.type_));

    Ok(result)
}

#[tauri::command]
pub fn open(path: String) -> Option<Vec<entity::FileTree>> {
    let path = Path::new(&path);
    match read_dir(path, 0) {
        Ok(filetree) => Some(filetree),
        Err(e) => {
            println!("{:?}", e);
            None
        }
    }
}

#[tauri::command]
pub fn select() -> entity::OpenFolder {
    let (tx, tr) = std::sync::mpsc::channel::<&entity::OpenFolder>();

    FileDialogBuilder::new().pick_folder(move |folder_path| {
        if let Some(path) = folder_path {
            let path = path.to_string_lossy().to_string();
            tx.send(Box::leak(Box::new(entity::OpenFolder {
                type_: 0,
                path: Box::new(path),
            })))
            .unwrap();
        } else {
            tx.send(Box::leak(Box::new(entity::OpenFolder {
                type_: 1,
                path: Box::new("path not found".to_string()),
            })))
            .unwrap();
        }
    });
    let path = tr.recv().unwrap();
    entity::OpenFolder {
        type_: path.type_,
        path: path.path.clone(),
    }
}

#[tauri::command]
pub fn create(
    path: String,
    name: String,
    is_dir: bool,
) -> entity::Response<Option<entity::OpenFile>> {
    let path = Path::new(&path);
    let path = path.join(name);
    if is_dir {
        match std::fs::create_dir(&path) {
            Ok(_) => {
                let metadata = fs::metadata(&path).unwrap();
                let open_file = entity::OpenFile {
                    type_: 0,
                    path: path.to_str().unwrap().to_string(),
                    content: "".to_string(),
                    updated: metadata.modified().unwrap().elapsed().unwrap().as_micros() as i64,
                };
                return entity::Response {
                    code: 10000,
                    msg: "success".to_string(),
                    data: Some(open_file),
                };
            }
            Err(e) => {
                return entity::Response {
                    code: 50000,
                    msg: e.to_string(),
                    data: None,
                };
            }
        }
    } else {
        std::fs::File::create(&path).unwrap();
        match std::fs::File::create(&path) {
            Ok(_) => {
                let metadata = fs::metadata(&path).unwrap();
                let open_file = entity::OpenFile {
                    type_: 0,
                    path: path.to_str().unwrap().to_string(),
                    content: "".to_string(),
                    updated: metadata.modified().unwrap().elapsed().unwrap().as_micros() as i64,
                };
                return entity::Response {
                    code: 10000,
                    msg: "success".to_string(),
                    data: Some(open_file),
                };
            }
            Err(e) => {
                return entity::Response {
                    code: 50000,
                    msg: e.to_string(),
                    data: None,
                };
            }
        }
    }
}

#[tauri::command]
pub fn delete(path: String, is_dir: bool) -> entity::Response<bool> {
    let path = Path::new(&path);
    if is_dir {
        match std::fs::remove_dir_all(&path) {
            Ok(_) => {
                return entity::Response {
                    code: 10000,
                    msg: "success".to_string(),
                    data: true,
                };
            }
            Err(e) => {
                return entity::Response {
                    code: 50000,
                    msg: e.to_string(),
                    data: false,
                };
            }
        }
    } else {
        match std::fs::remove_file(&path) {
            Ok(_) => {
                return entity::Response {
                    code: 10000,
                    msg: "success".to_string(),
                    data: true,
                };
            }
            Err(e) => {
                return entity::Response {
                    code: 50000,
                    msg: e.to_string(),
                    data: false,
                };
            }
        }
    }
}

// rename
#[tauri::command]
pub fn rename(path: String, name: String) -> bool {
    let old_path = Path::new(&path);
    let new_path = old_path.with_file_name(name);
    std::fs::rename(&old_path, &new_path).is_ok()
}

#[tauri::command]
pub fn read(path: String) -> Option<entity::OpenFile> {
    let path = Path::new(&path);
    if path.exists() && path.is_file() {
        let metadata = std::fs::metadata(&path).unwrap();
        let content = std::fs::read_to_string(&path).unwrap();
        let open_file = entity::OpenFile {
            type_: 0,
            path: path.to_str().unwrap().to_string(),
            content,
            updated: metadata.modified().unwrap().elapsed().unwrap().as_micros() as i64,
        };
        Some(open_file)
    } else {
        None
    }
}

#[tauri::command]
pub fn reads(paths: Vec<String>) -> Option<Vec<entity::OpenFile>> {
    let mut result = Vec::new();
    for path in paths {
        let path = Path::new(&path);
        if path.exists() && path.is_file() {
            let metadata = std::fs::metadata(&path).unwrap();
            let content = std::fs::read_to_string(&path).unwrap();
            let open_file = entity::OpenFile {
                type_: 0,
                path: path.to_str().unwrap().to_string(),
                content,
                updated: metadata.modified().unwrap().elapsed().unwrap().as_micros() as i64,
            };
            result.push(open_file);
        }
    }
    Some(result)
}

#[tauri::command]
pub fn write(path: String, content: String) -> Option<entity::OpenFile> {
    let path = Path::new(&path);
    if path.exists() && path.is_file() {
        std::fs::write(&path, content).unwrap();
        let metadata = std::fs::metadata(&path).unwrap();
        let content = std::fs::read_to_string(&path).unwrap();
        let open_file = entity::OpenFile {
            type_: 0,
            path: path.to_str().unwrap().to_string(),
            content,
            updated: metadata.modified().unwrap().elapsed().unwrap().as_micros() as i64,
        };
        Some(open_file)
    } else {
        None
    }
}

#[tauri::command]
pub fn get_config(path: String) -> entity::Response<entity::Config> {
    // path + ".ahriknow" + "config.json"
    let path = Path::new(&path);
    let config_path = path.join(".ahriknow").join("config.json");
    if !config_path.exists() {
        match config_path.parent() {
            Some(parent) => match std::fs::create_dir_all(parent) {
                Ok(_) => (),
                Err(e) => {
                    return entity::Response {
                        code: 50000,
                        msg: e.to_string(),
                        data: entity::Config {
                            token: "".to_string(),
                            project: "".to_string(),
                        },
                    }
                }
            },
            None => {
                return entity::Response {
                    code: 50000,
                    msg: "config path not found".to_string(),
                    data: entity::Config {
                        token: "".to_string(),
                        project: "".to_string(),
                    },
                }
            }
        }
    }
    match std::fs::write(
        &config_path,
        "{\n  \"token\": \"\",\n  \"project\": \"\"\n}",
    ) {
        Ok(_) => (),
        Err(e) => {
            return entity::Response {
                code: 50000,
                msg: e.to_string(),
                data: entity::Config {
                    token: "".to_string(),
                    project: "".to_string(),
                },
            }
        }
    }
    match std::fs::read_to_string(&config_path) {
        Ok(content) => match serde_json::from_str(&content) {
            Ok(config) => entity::Response {
                code: 10000,
                msg: "success".to_string(),
                data: config,
            },
            Err(e) => entity::Response {
                code: 50000,
                msg: e.to_string(),
                data: entity::Config {
                    token: "".to_string(),
                    project: "".to_string(),
                },
            },
        },
        Err(e) => entity::Response {
            code: 50000,
            msg: e.to_string(),
            data: entity::Config {
                token: "".to_string(),
                project: "".to_string(),
            },
        },
    }
}

#[tauri::command]
pub fn set_config(path: String, config: entity::Config) -> entity::Response<bool> {
    // path + ".ahriknow" + "config.json"
    let path = Path::new(&path);
    let config_path = path.join(".ahriknow").join("config.json");
    match serde_json::to_string_pretty(&config) {
        Ok(content) => match std::fs::write(&config_path, content) {
            Ok(_) => entity::Response {
                code: 10000,
                msg: "success".to_string(),
                data: true,
            },
            Err(e) => entity::Response {
                code: 50000,
                msg: e.to_string(),
                data: false,
            },
        },
        Err(e) => entity::Response {
            code: 50000,
            msg: e.to_string(),
            data: false,
        },
    }
}
