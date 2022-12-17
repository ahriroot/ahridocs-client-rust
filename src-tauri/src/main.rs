#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::path::Path;

use hotwatch::{Event, Hotwatch};
use serde_json;
use tauri::Manager;

use ahridocs::{api, entity};

#[tauri::command]
async fn close_splashscreen(window: tauri::Window) {
    // Close splashscreen
    if let Some(splashscreen) = window.get_window("splashscreen") {
        splashscreen.close().unwrap();
    }
    // Show main window
    window.get_window("main").unwrap().show().unwrap();
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_window("main").unwrap();
            let (tx, tr) = std::sync::mpsc::channel::<&entity::Message>();

            let tx2 = tx.clone();
            let _ = window.listen("watch-path-changed", move |event| {
                let payload = event.payload().unwrap();
                let open: entity::OpenFolder = serde_json::from_str(payload).unwrap();
                if open.type_ == 0 {
                    let path = open.path.clone();
                    if std::path::Path::new(path.as_str()).is_dir() {
                        tx2.send(Box::leak(Box::new(entity::Message {
                            type_: 100,
                            path: Box::new(path.to_string()),
                            path2: Box::new("".to_string()),
                        })))
                        .unwrap();
                    } else {
                        tx2.send(Box::leak(Box::new(entity::Message {
                            type_: 0,
                            path: Box::new(path.to_string()),
                            path2: Box::new("".to_string()),
                        })))
                        .unwrap();
                    }
                } else {
                    tx2.send(Box::leak(Box::new(entity::Message {
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
                                            txx.send(Box::leak(Box::new(entity::Message {
                                                type_: 1,
                                                path: Box::new(path.to_str().unwrap().to_string()),
                                                path2: Box::new("".to_string()),
                                            })))
                                            .unwrap();
                                        } else {
                                            let ext = path.extension().unwrap().to_str().unwrap();
                                            if ext == "md" || ext == "json" {
                                                txx.send(Box::leak(Box::new(entity::Message {
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
                                            txx.send(Box::leak(Box::new(entity::Message {
                                                type_: 3,
                                                path: Box::new(path.to_str().unwrap().to_string()),
                                                path2: Box::new("".to_string()),
                                            })))
                                            .unwrap();
                                        } else {
                                            let ext = path.extension().unwrap().to_str().unwrap();
                                            if ext == "md" || ext == "json" {
                                                txx.send(Box::leak(Box::new(entity::Message {
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
                                            txx.send(Box::leak(Box::new(entity::Message {
                                                type_: -1,
                                                path: Box::new(path.to_str().unwrap().to_string()),
                                                path2: Box::new("".to_string()),
                                            })))
                                            .unwrap();
                                        } else {
                                            let path: String = path.to_str().unwrap().to_string();
                                            if path.ends_with(".md") || path.ends_with(".json") {
                                                txx.send(Box::leak(Box::new(entity::Message {
                                                    type_: -2,
                                                    path: Box::new(path),
                                                    path2: Box::new("".to_string()),
                                                })))
                                                .unwrap();
                                            }
                                        }
                                    }
                                    Event::Rename(from, to) => {
                                        if to.is_dir() {
                                            txx.send(Box::leak(Box::new(entity::Message {
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
                                                txx.send(Box::leak(Box::new(entity::Message {
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
                        let p = msg.path.as_str();
                        // if startwith \\?\, remove it
                        let p = if p.starts_with("\\\\?\\") { &p[4..] } else { p };
                        let path: &Path = Path::new(p);
                        window
                            .emit(
                                "file-system-changed",
                                entity::Message {
                                    type_: msg.type_,
                                    path: Box::new(path.to_str().unwrap().to_string()),
                                    path2: msg.path2.clone(),
                                },
                            )
                            .unwrap();
                    }
                }
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            close_splashscreen,
            api::select,
            api::open,
            api::create,
            api::delete,
            api::rename,
            api::read,
            api::reads,
            api::write,
            api::get_config,
            api::set_config,
        ])
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
