#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::Manager;
use std::sync::Mutex;
use tokio::runtime::Runtime;

mod note;
mod pdf;
mod ui;
mod storage;
mod server;

#[derive(Default)]
struct AppState {
    notes: Mutex<Vec<note::Note>>,
    current_note: Mutex<Option<note::Note>>,
}

fn main() {
    // 启动HTTP服务器
    let rt = Runtime::new().unwrap();
    rt.spawn(async {
        if let Err(e) = server::start_server(3000).await {
            eprintln!("HTTP服务器启动失败: {}", e);
        }
    });
    
    tauri::Builder::default()
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![
            create_note,
            save_note,
            load_note,
            export_pdf,
            import_pdf,
            get_notes_list
        ])
        .setup(|app| {
            let window = app.get_window("main").unwrap();
            
            // 设置窗口模糊效果
            #[cfg(target_os = "macos")]
            {
                use window_vibrancy::{apply_vibrancy, NSVisualEffectMaterial};
                apply_vibrancy(&window, NSVisualEffectMaterial::HudWindow, None, None)
                    .expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");
            }
            
            #[cfg(target_os = "windows")]
            {
                use window_vibrancy::apply_blur;
                apply_blur(&window, Some((18, 18, 18, 125)))
                    .expect("Unsupported platform! 'apply_blur' is only supported on Windows");
            }
            
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn create_note(title: String, state: tauri::State<AppState>) -> Result<String, String> {
    let mut notes = state.notes.lock().unwrap();
    let new_note = note::Note::new(title);
    let note_id = new_note.id.clone();
    notes.push(new_note);
    Ok(note_id)
}

#[tauri::command]
fn save_note(note_data: note::Note, state: tauri::State<AppState>) -> Result<(), String> {
    let mut notes = state.notes.lock().unwrap();
    if let Some(index) = notes.iter().position(|n| n.id == note_data.id) {
        notes[index] = note_data;
    }
    Ok(())
}

#[tauri::command]
fn load_note(note_id: String, state: tauri::State<AppState>) -> Result<note::Note, String> {
    let notes = state.notes.lock().unwrap();
    notes.iter()
        .find(|n| n.id == note_id)
        .cloned()
        .ok_or_else(|| "Note not found".to_string())
}

#[tauri::command]
fn get_notes_list(state: tauri::State<AppState>) -> Result<Vec<note::Note>, String> {
    let notes = state.notes.lock().unwrap();
    Ok(notes.clone())
}

#[tauri::command]
fn export_pdf(note_id: String, file_path: String, state: tauri::State<AppState>) -> Result<(), String> {
    let notes = state.notes.lock().unwrap();
    let note = notes.iter()
        .find(|n| n.id == note_id)
        .ok_or_else(|| "Note not found".to_string())?;
    
    pdf::export_to_pdf(note, &file_path)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn import_pdf(file_path: String, state: tauri::State<AppState>) -> Result<String, String> {
    let mut notes = state.notes.lock().unwrap();
    let note = pdf::import_from_pdf(&file_path)
        .map_err(|e| e.to_string())?;
    
    let note_id = note.id.clone();
    notes.push(note);
    Ok(note_id)
}