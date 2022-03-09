#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::io::{Read, Write};
mod pixiv;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![startp, write_key, get_key])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn startp(appid: String, key: String) {
    // Call another async function and wait for it to finish
    pixiv::start(appid, key).await;
}

#[tauri::command]
fn get_key() -> Vec<String> {
    println!("reading!");
    let mut s = String::new();
    let f = std::fs::File::open("save.txt");
    if let Ok(mut f) = f {
        if let Err(e) = f.read_to_string(&mut s) {
            return vec![format!("{:?}", e), format!("{:?}", e)];
        }
        println!("{}", s);
        let r: Vec<&str> = s.split(" ").collect();
        if r.len() < 2 {
            return vec!["null".into(), "null".into()];
        }
        return vec![r[0].into(), r[1].into()];
    } else {
        return vec!["null".into(), "null".into()];
    }
}

#[tauri::command]
fn write_key(appid: String, key: String) -> String {
    println!("writing!{} {}", appid, key);
    let f = std::fs::File::create("save.txt");
    if let Ok(mut f) = f {
        if let Err(e) = f.write_all(format!("{} {}", appid, key).as_bytes()) {
            return format!("{:?}", e);
        } else {
            return String::from("存了");
        }
    }
    return String::from("好像没存上");
}
