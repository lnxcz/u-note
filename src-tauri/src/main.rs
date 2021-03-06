#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

extern crate serde;

use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use std::fs;
use std::path::Path;
use std::sync::Mutex;
use std::{convert::TryFrom, error::Error};
use tauri::{Manager, State};

#[derive(serde::Serialize)]
struct File {
    name: String,
    path: String,
    content: Option<String>,
    preview: Option<String>,
}

#[derive(serde::Serialize)]
struct Directory {
    name: String,
    path: String,
    children_count: i32,
}

#[derive(serde::Serialize)]
enum FsElement {
    File(File),
    Directory(Directory),
}

//This function reads the contents of the directory, and for each file or
//directory in the directory, it returns an FsElement enum variant representing
//the file or directory.
#[tauri::command]
async fn list_dir_files(path: String) -> Vec<FsElement> {
    let paths = fs::read_dir(path).unwrap();
    let files: Vec<FsElement> = paths
        .map(|e| e.unwrap())
        .filter(|p| !p.file_name().to_str().unwrap().starts_with("."))
        .map(|this_path| -> Result<FsElement, Box<dyn Error>> {
            if this_path.metadata()?.is_dir() {
                let children_count = i32::try_from(
                    fs::read_dir(this_path.path())?
                        .filter(|p| {
                            !p.as_ref()
                                .unwrap()
                                .file_name()
                                .to_str()
                                .unwrap()
                                .starts_with('.')
                        })
                        .count(),
                )?;

                Ok(FsElement::Directory(Directory {
                    name: this_path.file_name().to_str().unwrap().to_string(),
                    path: this_path.path().to_str().unwrap().to_string(),
                    children_count,
                }))
            } else {
                let name = this_path.file_name().to_str().expect("error").to_string();
                let file_path = this_path.path().to_str().expect("error").to_string();

                let content = match fs::read_to_string(&file_path) {
                    Ok(content) => content,
                    Err(_) => String::from(""),
                };

                return Ok(FsElement::File(File {
                    name,
                    path: file_path,
                    content: None,
                    preview: Some(content.chars().take(100).collect()),
                }));
            }
        })
        .map(|res| res.unwrap())
        .collect();
    files
}

//Lists all paths recursively, and returns a vector containing them
//If deep == false, only lists files in the current folder
fn list_path(path: String, deep: bool) -> Vec<String> {
    let paths = fs::read_dir(path).unwrap();
    let mut all_path = vec![];
    paths.map(|e| e.unwrap()).for_each(|p| {
        let path_string = p.path().to_str().unwrap().to_string();
        if p.metadata().unwrap().is_dir() && deep {
            all_path.extend(list_path(path_string, deep))
        } else {
            all_path.push(path_string);
        }
    });
    all_path
}

#[tauri::command]
fn list_path_deep(path: String, deep: bool) -> Vec<String> {
    list_path(path, deep)
}

#[tauri::command]
fn is_dir(path: String) -> bool {
    println!("{path}");
    let path_loc = Path::new(path.as_str());
    if Path::new(path.as_str()).exists() {
        path_loc.is_dir()
    } else {
        false
    }
}

#[tauri::command]
async fn open_file(path: String) -> File {
    return |path| -> Result<File, Box<dyn Error>> {
        let content = fs::read_to_string(&path)?;

        Ok(File {
            name: String::from(&path),
            path,
            content: Some(content),
            preview: None,
        })
    }(path)
    .unwrap();
}
struct Watch(Mutex<RecommendedWatcher>);

//Set watcher for given directory
#[tauri::command]
async fn watch(path: String, watcher: State<'_, Watch>) -> Result<(), ()> {
    println!("Watching {}", &path);

    let res_path = check_path(path);

    watcher
        .0
        .lock()
        .unwrap()
        .watch(Path::new(&res_path), RecursiveMode::Recursive)
        .unwrap();

    Ok(())
}
//Stop watcher for given directory
#[tauri::command]
async fn unwatch(path: String, watcher: State<'_, Watch>) -> Result<(), ()> {
    println!("Stop watching {}", &path);
    let res_path = check_path(path);

    watcher
        .0
        .lock()
        .unwrap()
        .unwatch(Path::new(&res_path))
        .unwrap();

    Ok(())
}

// look up if path exists
fn check_path(path: String) -> String {
    println!("Checking {}", &path);

    let path_loc = Path::new(path.as_str());
    let mut path_buf = path_loc.to_path_buf();

    if !path_loc.exists() {
        path_buf.pop();
        check_path(path_buf.to_str().unwrap().to_string());
    }
    return path_buf.to_str().unwrap().to_string();
}

//Main script
fn main() -> notify::Result<()> {
    tauri::Builder::default()
        .setup(|app| {
            // attach the notify watcher to the app
            let handle = app.handle();

            //Setup watcher
            let w =
                notify::recommended_watcher(move |res: Result<notify::Event, notify::Error>| {
                    match res {
                        Ok(event) => {
                            println!("{:?}", event);
                            handle
                                .emit_all(
                                    "file_changed",
                                    event.paths[0].to_str().unwrap().to_string(),
                                )
                                .unwrap();
                        }
                        Err(e) => eprintln!("watch error: {:?}", e),
                    }
                })?;

            app.manage(Watch(Mutex::new(w)));

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            list_dir_files,
            open_file,
            watch,
            unwatch,
            list_path_deep,
            is_dir
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
