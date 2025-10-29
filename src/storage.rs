use crate::note::Note;
use serde_json;
use std::fs;
use std::path::Path;

const NOTES_DIR: &str = "notes";
const FILE_EXTENSION: &str = "spn";

pub fn save_note(note: &Note) -> Result<(), Box<dyn std::error::Error>> {
    // 确保notes目录存在
    if !Path::new(NOTES_DIR).exists() {
        fs::create_dir_all(NOTES_DIR)?;
    }
    
    // 序列化笔记数据
    let json_data = serde_json::to_string_pretty(note)?;
    
    // 构建文件路径
    let file_name = format!("{}.{}", note.id, FILE_EXTENSION);
    let file_path = Path::new(NOTES_DIR).join(file_name);
    
    // 写入文件
    fs::write(file_path, json_data)?;
    
    Ok(())
}

pub fn load_note(note_id: &str) -> Result<Note, Box<dyn std::error::Error>> {
    let file_name = format!("{}.{}", note_id, FILE_EXTENSION);
    let file_path = Path::new(NOTES_DIR).join(file_name);
    
    if !file_path.exists() {
        return Err(format!("Note file not found: {}", note_id).into());
    }
    
    let json_data = fs::read_to_string(file_path)?;
    let note: Note = serde_json::from_str(&json_data)?;
    
    Ok(note)
}

pub fn list_notes() -> Result<Vec<Note>, Box<dyn std::error::Error>> {
    let notes_dir = Path::new(NOTES_DIR);
    
    if !notes_dir.exists() {
        return Ok(Vec::new());
    }
    
    let mut notes = Vec::new();
    
    for entry in fs::read_dir(notes_dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.extension().and_then(|s| s.to_str()) == Some(FILE_EXTENSION) {
            if let Ok(note) = load_note(
                path.file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("")
            ) {
                notes.push(note);
            }
        }
    }
    
    // 按更新时间排序
    notes.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
    
    Ok(notes)
}

pub fn delete_note(note_id: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file_name = format!("{}.{}", note_id, FILE_EXTENSION);
    let file_path = Path::new(NOTES_DIR).join(file_name);
    
    if file_path.exists() {
        fs::remove_file(file_path)?;
    }
    
    Ok(())
}

pub fn export_note_as_spn(note: &Note, export_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let json_data = serde_json::to_string_pretty(note)?;
    fs::write(export_path, json_data)?;
    Ok(())
}

pub fn import_note_from_spn(import_path: &str) -> Result<Note, Box<dyn std::error::Error>> {
    let json_data = fs::read_to_string(import_path)?;
    let note: Note = serde_json::from_str(&json_data)?;
    Ok(note)
}