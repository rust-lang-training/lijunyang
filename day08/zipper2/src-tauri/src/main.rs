// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{fs::File, path, thread};

use serde::Serialize;
use tauri::Window;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[derive(Serialize, Clone, Copy, Debug)]
enum ZipEntryType {
    File,
    Directory,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
struct ZipEntry {
    name: String,
    entry_type: ZipEntryType,
    size: u64,
    compressed_size: u64,
}

#[derive(Debug, Serialize, Clone)]
struct ResultMessage {
    result: bool,
    message: String,
    entries: Option<Vec<ZipEntry>>,
}

#[tauri::command]
fn list_entries(window: Window, file_name: String) -> Result<bool, String> {
    let p = path::Path::new(&file_name);
    if !p.exists() {
        return Err(format!("File not found: {}", &file_name));
    }
    thread::spawn(move || {
        let f = match File::open(&file_name) {
            Ok(f) => f,
            Err(e) => {
                window
                    .emit(
                        "list-entries-result",
                        ResultMessage {
                            result: false,
                            message: e.to_string(),
                            entries: None,
                        },
                    )
                    .unwrap();
                return;
            }
        };
        let mut zip = match zip::read::ZipArchive::new(f) {
            Ok(z) => z,
            Err(e) => {
                window
                    .emit(
                        "list-entries-result",
                        ResultMessage {
                            result: false,
                            message: e.to_string(),
                            entries: None,
                        },
                    )
                    .unwrap();
                return;
            }
        };

        let mut entries = Vec::<ZipEntry>::new();
        for i in 0..zip.len() {
            let zip_file = match zip.by_index(i) {
                Ok(f) => f,
                Err(e) => {
                    window
                        .emit(
                            "list-entries-result",
                            ResultMessage {
                                result: false,
                                message: e.to_string(),
                                entries: None,
                            },
                        )
                        .unwrap();
                    return;
                }
            };
            entries.push(ZipEntry {
                name: String::from_utf8_lossy(zip_file.name_raw()).to_string(),
                // name: encoding_rs::GB18030
                //     .decode(zip_file.name_raw())
                //     .0
                //     .to_string(),
                entry_type: if zip_file.is_dir() {
                    ZipEntryType::Directory
                } else {
                    ZipEntryType::File
                },
                compressed_size: zip_file.compressed_size(),
                size: zip_file.size(),
            });
        }
        window
            .emit(
                "list-entries-result",
                ResultMessage {
                    result: true,
                    message: "OK".to_string(),
                    entries: Some(entries),
                },
            )
            .unwrap();
    });

    Ok(true)
}
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, list_entries])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
