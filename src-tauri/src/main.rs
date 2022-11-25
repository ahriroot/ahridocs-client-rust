#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use hotwatch::{Event, Hotwatch};
use serde_json;
use tauri::api::dialog::FileDialogBuilder;
use tauri::Manager;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

#[derive(Clone, serde::Serialize)]
struct Message {
    type_: i32,
    path: Box<String>,
    path2: Box<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct OpenFolder {
    type_: i32,
    path: Box<String>,
}

#[tauri::command]
fn open() -> OpenFolder {
    let (tx, tr) = std::sync::mpsc::channel::<&OpenFolder>();

    FileDialogBuilder::new().pick_folders(move |file_paths| match file_paths {
        Some(paths) => {
            if let Some(path) = paths.first() {
                let path = path.to_string_lossy().to_string();
                tx.send(Box::leak(Box::new(OpenFolder {
                    type_: 0,
                    path: Box::new(path),
                })))
                .unwrap();
            } else {
                tx.send(Box::leak(Box::new(OpenFolder {
                    type_: 1,
                    path: Box::new("path not found".to_string()),
                })))
                .unwrap();
            }
        }
        None => {
            tx.send(Box::leak(Box::new(OpenFolder {
                type_: 2,
                path: Box::new("path not found".to_string()),
            })))
            .unwrap();
        }
    });
    let path = tr.recv().unwrap();
    OpenFolder {
        type_: path.type_,
        path: path.path.clone(),
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_window("main").unwrap();
            let (tx, tr) = std::sync::mpsc::channel::<&Message>();

            let tx2 = tx.clone();
            let _ = window.listen("watch-path-changed", move |event| {
                let payload = event.payload().unwrap();
                let open: OpenFolder = serde_json::from_str(payload).unwrap();
                if open.type_ == 0 {
                    let path = open.path.clone();
                    if std::path::Path::new(path.as_str()).is_dir() {
                        tx2.send(Box::leak(Box::new(Message {
                            type_: 100,
                            path: Box::new(path.to_string()),
                            path2: Box::new("".to_string()),
                        })))
                        .unwrap();
                    } else {
                        tx2.send(Box::leak(Box::new(Message {
                            type_: 0,
                            path: Box::new(path.to_string()),
                            path2: Box::new("".to_string()),
                        })))
                        .unwrap();
                    }
                } else {
                    tx2.send(Box::leak(Box::new(Message {
                        type_: -100,
                        path: Box::new("".to_string()),
                        path2: Box::new("".to_string()),
                    })))
                    .unwrap();
                }
            });

            let _ = std::thread::spawn(move || {
                let mut hotwatch = Hotwatch::new().expect("hotwatch failed to initialize!");
                let mut old_watch = "".to_string();
                loop {
                    let txx = tx.clone();
                    let msg = tr.recv().unwrap();
                    if msg.type_ == -100 {
                        hotwatch.unwatch(old_watch.clone()).unwrap_or(());
                    } else if msg.type_ == 100 {
                        hotwatch.unwatch(old_watch).unwrap_or(());
                        old_watch = msg.path.to_string();
                        hotwatch
                            .watch(
                                msg.path.as_str().to_string(),
                                move |event: Event| match event {
                                    Event::Create(path) => {
                                        if path.is_dir() {
                                            txx.send(Box::leak(Box::new(Message {
                                                type_: 1,
                                                path: Box::new(path.to_str().unwrap().to_string()),
                                                path2: Box::new("".to_string()),
                                            })))
                                            .unwrap();
                                        } else {
                                            let ext = path.extension().unwrap().to_str().unwrap();
                                            if ext == "md" || ext == "json" {
                                                txx.send(Box::leak(Box::new(Message {
                                                    type_: 2,
                                                    path: Box::new(
                                                        path.to_str().unwrap().to_string(),
                                                    ),
                                                    path2: Box::new("".to_string()),
                                                })))
                                                .unwrap();
                                            }
                                        }
                                    }
                                    Event::Write(path) => {
                                        if path.is_dir() {
                                            txx.send(Box::leak(Box::new(Message {
                                                type_: 3,
                                                path: Box::new(path.to_str().unwrap().to_string()),
                                                path2: Box::new("".to_string()),
                                            })))
                                            .unwrap();
                                        } else {
                                            let ext = path.extension().unwrap().to_str().unwrap();
                                            if ext == "md" || ext == "json" {
                                                txx.send(Box::leak(Box::new(Message {
                                                    type_: 4,
                                                    path: Box::new(
                                                        path.to_str().unwrap().to_string(),
                                                    ),
                                                    path2: Box::new("".to_string()),
                                                })))
                                                .unwrap();
                                            }
                                        }
                                    }
                                    Event::Remove(path) => {
                                        if path.is_dir() {
                                            txx.send(Box::leak(Box::new(Message {
                                                type_: -1,
                                                path: Box::new(path.to_str().unwrap().to_string()),
                                                path2: Box::new("".to_string()),
                                            })))
                                            .unwrap();
                                        } else {
                                            let path: String = path.to_str().unwrap().to_string();
                                            if path.ends_with(".md") || path.ends_with(".json") {
                                                txx.send(Box::leak(Box::new(Message {
                                                    type_: -2,
                                                    path: Box::new(path),
                                                    path2: Box::new("".to_string()),
                                                })))
                                                .unwrap();
                                            }
                                        }
                                    }
                                    Event::Rename(from, to) => {
                                        if from.is_dir() {
                                            txx.send(Box::leak(Box::new(Message {
                                                type_: 5,
                                                path: Box::new(from.to_str().unwrap().to_string()),
                                                path2: Box::new(to.to_str().unwrap().to_string()),
                                            })))
                                            .unwrap();
                                        } else {
                                            let ext = from.extension().unwrap().to_str().unwrap();
                                            let ext2 = to.extension().unwrap().to_str().unwrap();
                                            if (ext == "md" && ext2 == "md")
                                                || (ext == "json" && ext2 == "json")
                                            {
                                                txx.send(Box::leak(Box::new(Message {
                                                    type_: 6,
                                                    path: Box::new(
                                                        from.to_str().unwrap().to_string(),
                                                    ),
                                                    path2: Box::new(
                                                        to.to_str().unwrap().to_string(),
                                                    ),
                                                })))
                                                .unwrap();
                                            }
                                        }
                                    }
                                    _ => {}
                                },
                            )
                            .expect("failed to watch file!");
                    } else {
                        window.emit("file-system-changed", msg).unwrap();
                    }
                }
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![open])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    Ok(())
}

#[cfg(test)]
mod tests {
    use hotwatch::{Event, Hotwatch};
    use tokio::time::sleep;

    #[tokio::test]
    async fn test() {
        let mut hotwatch = Hotwatch::new().expect("hotwatch failed to initialize!");
        hotwatch
            .watch(
                "\\\\?\\C:\\Users\\ahrik\\Desktop\\ahridocs-client-rust\\documents",
                |event: Event| {
                    println!("get some event {:?}", event);
                    if let Event::Write(path) = event {
                        let ext = path.extension().unwrap().to_str().unwrap();
                        // if file endwith md or json
                        if ext == "md" || ext == "json" {
                            println!("test.txt has changed.{:?}", path);
                        } else {
                            println!("other file has been changed!{:?}", path);
                        }
                    }
                },
            )
            .expect("failed to watch file!");

        loop {
            println!("loop");
            sleep(tokio::time::Duration::from_secs(2)).await;
        }
    }

    #[tokio::test]
    async fn test2() {
        // let path = "";

        // write to path
        // std::fs::write(path, "hello world").unwrap();
    }
}
