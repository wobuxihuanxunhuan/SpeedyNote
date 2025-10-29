use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stroke {
    pub points: Vec<Point>,
    pub color: String,
    pub thickness: f32,
    pub pressure: Vec<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Page {
    pub strokes: Vec<Stroke>,
    pub background: Background,
    pub width: f32,
    pub height: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Background {
    Blank,
    Lined { spacing: f32 },
    Grid { spacing: f32 },
    Pdf { file_path: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Note {
    pub id: String,
    pub title: String,
    pub pages: Vec<Page>,
    pub created_at: u64,
    pub updated_at: u64,
    pub current_page: usize,
}

impl Note {
    pub fn new(title: String) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let default_page = Page {
            strokes: Vec::new(),
            background: Background::Blank,
            width: 800.0,
            height: 1000.0,
        };
        
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            title,
            pages: vec![default_page],
            created_at: now,
            updated_at: now,
            current_page: 0,
        }
    }
    
    pub fn add_stroke(&mut self, stroke: Stroke) {
        if let Some(page) = self.pages.get_mut(self.current_page) {
            page.strokes.push(stroke);
            self.updated_at = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();
        }
    }
    
    pub fn add_page(&mut self, background: Background) {
        let new_page = Page {
            strokes: Vec::new(),
            background,
            width: 800.0,
            height: 1000.0,
        };
        self.pages.push(new_page);
        self.current_page = self.pages.len() - 1;
    }
    
    pub fn remove_page(&mut self, page_index: usize) -> Result<(), String> {
        if self.pages.len() <= 1 {
            return Err("Cannot remove the last page".to_string());
        }
        
        if page_index >= self.pages.len() {
            return Err("Page index out of bounds".to_string());
        }
        
        self.pages.remove(page_index);
        if self.current_page >= page_index && self.current_page > 0 {
            self.current_page -= 1;
        }
        
        Ok(())
    }
    
    pub fn set_background(&mut self, background: Background) {
        if let Some(page) = self.pages.get_mut(self.current_page) {
            page.background = background;
        }
    }
}