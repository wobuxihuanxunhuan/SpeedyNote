use crate::note::{Note, Page, Background};
use std::path::Path;

pub fn export_to_pdf(note: &Note, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // 使用pdf库创建PDF文档
    let mut doc = pdf::Document::new();
    
    for (page_index, page) in note.pages.iter().enumerate() {
        let mut page_builder = doc.add_page(pdf::Pt(page.width as f64), pdf::Pt(page.height as f64));
        
        // 设置页面背景
        match &page.background {
            Background::Pdf { file_path: pdf_path } => {
                // 如果是PDF背景，可以在这里处理
                // 这里简化处理，实际需要解析PDF页面
            }
            _ => {
                // 对于其他背景类型，创建简单的背景
                let background_color = match &page.background {
                    Background::Lined { .. } => pdf::Color::Rgb(0.95, 0.95, 0.95),
                    Background::Grid { .. } => pdf::Color::Rgb(0.98, 0.98, 0.98),
                    _ => pdf::Color::Rgb(1.0, 1.0, 1.0),
                };
                
                page_builder = page_builder
                    .set_fill_color(background_color)
                    .rect(pdf::Pt(0.0), pdf::Pt(0.0), pdf::Pt(page.width as f64), pdf::Pt(page.height as f64))
                    .fill();
            }
        }
        
        // 绘制笔画
        for stroke in &page.strokes {
            if stroke.points.is_empty() {
                continue;
            }
            
            // 设置笔画颜色
            let color = parse_color(&stroke.color).unwrap_or(pdf::Color::Rgb(0.0, 0.0, 0.0));
            page_builder = page_builder.set_stroke_color(color);
            
            // 设置线宽
            page_builder = page_builder.set_line_width(stroke.thickness as f64);
            
            // 绘制路径
            let mut path_builder = page_builder.path();
            if let Some(first_point) = stroke.points.first() {
                path_builder = path_builder.move_to(
                    pdf::Pt(first_point.x as f64),
                    pdf::Pt((page.height - first_point.y) as f64) // PDF坐标系Y轴从下往上
                );
                
                for point in &stroke.points[1..] {
                    path_builder = path_builder.line_to(
                        pdf::Pt(point.x as f64),
                        pdf::Pt((page.height - point.y) as f64)
                    );
                }
            }
            
            page_builder = path_builder.stroke();
        }
        
        // 添加页面标题
        if page_index == 0 {
            page_builder = page_builder
                .set_fill_color(pdf::Color::Rgb(0.2, 0.2, 0.2))
                .set_font(pdf::Font::Times_Roman, 24.0)
                .begin_text()
                .text_matrix(pdf::TextMatrix::new(1.0, 0.0, 0.0, 1.0, 50.0, page.height as f64 - 50.0))
                .show(&note.title)
                .end_text();
        }
    }
    
    // 保存PDF文件
    doc.save(file_path)?;
    Ok(())
}

pub fn import_from_pdf(file_path: &str) -> Result<Note, Box<dyn std::error::Error>> {
    let path = Path::new(file_path);
    let file_name = path.file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("Imported PDF");
    
    // 创建新的笔记
    let mut note = Note::new(file_name.to_string());
    
    // 设置PDF背景
    note.set_background(Background::Pdf {
        file_path: file_path.to_string(),
    });
    
    Ok(note)
}

fn parse_color(color_str: &str) -> Option<pdf::Color> {
    if color_str.starts_with('#') && color_str.len() == 7 {
        let r = u8::from_str_radix(&color_str[1..3], 16).ok()?;
        let g = u8::from_str_radix(&color_str[3..5], 16).ok()?;
        let b = u8::from_str_radix(&color_str[5..7], 16).ok()?;
        
        Some(pdf::Color::Rgb(
            r as f32 / 255.0,
            g as f32 / 255.0,
            b as f32 / 255.0
        ))
    } else {
        // 处理命名颜色
        match color_str.to_lowercase().as_str() {
            "black" => Some(pdf::Color::Rgb(0.0, 0.0, 0.0)),
            "red" => Some(pdf::Color::Rgb(1.0, 0.0, 0.0)),
            "green" => Some(pdf::Color::Rgb(0.0, 1.0, 0.0)),
            "blue" => Some(pdf::Color::Rgb(0.0, 0.0, 1.0)),
            _ => None,
        }
    }
}