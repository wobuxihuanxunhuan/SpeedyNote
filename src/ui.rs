use eframe::egui;
use crate::note::{Note, Stroke, Point, Background};

pub struct SpeedyNoteApp {
    notes: Vec<Note>,
    current_note: Option<Note>,
    drawing: bool,
    current_stroke: Option<Stroke>,
    brush_color: String,
    brush_thickness: f32,
    show_dial: bool,
}

impl Default for SpeedyNoteApp {
    fn default() -> Self {
        Self {
            notes: Vec::new(),
            current_note: None,
            drawing: false,
            current_stroke: None,
            brush_color: "#000000".to_string(),
            brush_thickness: 2.0,
            show_dial: false,
        }
    }
}

impl eframe::App for SpeedyNoteApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // 设置窗口模糊背景
        self.setup_blur_background(ctx);
        
        // 主界面布局
        egui::CentralPanel::default().show(ctx, |ui| {
            self.render_main_interface(ui);
        });
        
        // 侧边栏
        egui::SidePanel::left("sidebar").show(ctx, |ui| {
            self.render_sidebar(ui);
        });
        
        // 顶部工具栏
        egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
            self.render_toolbar(ui);
        });
        
        // Dial UI
        if self.show_dial {
            self.render_dial_ui(ctx);
        }
        
        ctx.request_repaint();
    }
}

impl SpeedyNoteApp {
    fn setup_blur_background(&self, ctx: &egui::Context) {
        // 设置现代化模糊玻璃效果
        let style = ctx.style();
        let mut visuals = style.visuals.clone();
        
        // 设置半透明背景
        visuals.panel_fill = egui::Color32::from_rgba_premultiplied(255, 255, 255, 30);
        visuals.window_fill = egui::Color32::from_rgba_premultiplied(255, 255, 255, 20);
        visuals.extreme_bg_color = egui::Color32::from_rgba_premultiplied(255, 255, 255, 10);
        
        ctx.set_visuals(visuals);
    }
    
    fn render_main_interface(&mut self, ui: &mut egui::Ui) {
        if let Some(note) = &mut self.current_note {
            if let Some(page) = note.pages.get_mut(note.current_page) {
                // 创建绘图区域
                let (rect, response) = ui.allocate_exact_size(
                    egui::Vec2::new(page.width, page.height),
                    egui::Sense::drag()
                );
                
                // 绘制背景
                self.draw_background(ui, &page.background, rect);
                
                // 绘制已有笔画
                self.draw_existing_strokes(ui, &page.strokes, rect);
                
                // 处理绘图输入
                self.handle_drawing_input(&response, rect, page);
                
                // 绘制当前笔画
                if let Some(stroke) = &self.current_stroke {
                    self.draw_stroke(ui, stroke, rect);
                }
            }
        } else {
            // 欢迎界面
            ui.vertical_centered(|ui| {
                ui.heading("欢迎使用 SpeedyNote");
                ui.label("轻量级、快速、支持手写笔的笔记应用");
                
                if ui.button("新建笔记").clicked() {
                    self.create_new_note();
                }
            });
        }
    }
    
    fn render_sidebar(&mut self, ui: &mut egui::Ui) {
        ui.heading("笔记列表");
        
        for note in &self.notes {
            if ui.selectable_label(
                self.current_note.as_ref().map(|n| n.id.as_str()) == Some(&note.id),
                &note.title
            ).clicked() {
                self.current_note = Some(note.clone());
            }
        }
        
        ui.separator();
        
        if ui.button("新建笔记").clicked() {
            self.create_new_note();
        }
    }
    
    fn render_toolbar(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            // 画笔设置
            ui.color_edit_button_srgba(&mut self.brush_color.parse().unwrap_or(egui::Color32::BLACK));
            ui.add(egui::Slider::new(&mut self.brush_thickness, 1.0..=10.0).text("粗细"));
            
            ui.separator();
            
            // 页面控制
            if let Some(note) = &mut self.current_note {
                if ui.button("上一页").clicked() && note.current_page > 0 {
                    note.current_page -= 1;
                }
                
                ui.label(format!("第 {} 页 / 共 {} 页", note.current_page + 1, note.pages.len()));
                
                if ui.button("下一页").clicked() && note.current_page < note.pages.len() - 1 {
                    note.current_page += 1;
                }
                
                if ui.button("新增页面").clicked() {
                    note.add_page(Background::Blank);
                }
            }
            
            ui.separator();
            
            // Dial控制
            if ui.button("Dial").clicked() {
                self.show_dial = !self.show_dial;
            }
        });
    }
    
    fn render_dial_ui(&mut self, ctx: &egui::Context) {
        egui::Window::new("Magic Dial")
            .collapsible(false)
            .resizable(false)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("Magic Dial");
                    
                    // 模拟拨盘控制
                    ui.horizontal(|ui| {
                        if ui.button("←").clicked() {
                            // 上一页或减小笔刷
                        }
                        
                        ui.label("○");
                        
                        if ui.button("→").clicked() {
                            // 下一页或增加笔刷
                        }
                    });
                    
                    ui.label("旋转控制: 缩放/翻页/笔刷大小");
                });
            });
    }
    
    fn draw_background(&self, ui: &mut egui::Ui, background: &Background, rect: egui::Rect) {
        match background {
            Background::Blank => {
                // 空白背景 - 轻微模糊效果
                ui.painter().rect_filled(rect, 0.0, egui::Color32::from_rgba_premultiplied(255, 255, 255, 50));
            }
            Background::Lined { spacing } => {
                // 横线背景
                let painter = ui.painter();
                painter.rect_filled(rect, 0.0, egui::Color32::from_rgba_premultiplied(255, 255, 255, 50));
                
                let mut y = rect.top() + *spacing;
                while y < rect.bottom() {
                    painter.line_segment(
                        [egui::Pos2::new(rect.left(), y), egui::Pos2::new(rect.right(), y)],
                        (1.0, egui::Color32::from_rgba_premultiplied(200, 200, 200, 100))
                    );
                    y += spacing;
                }
            }
            Background::Grid { spacing } => {
                // 网格背景
                let painter = ui.painter();
                painter.rect_filled(rect, 0.0, egui::Color32::from_rgba_premultiplied(255, 255, 255, 50));
                
                // 水平线
                let mut y = rect.top() + *spacing;
                while y < rect.bottom() {
                    painter.line_segment(
                        [egui::Pos2::new(rect.left(), y), egui::Pos2::new(rect.right(), y)],
                        (0.5, egui::Color32::from_rgba_premultiplied(200, 200, 200, 80))
                    );
                    y += spacing;
                }
                
                // 垂直线
                let mut x = rect.left() + *spacing;
                while x < rect.right() {
                    painter.line_segment(
                        [egui::Pos2::new(x, rect.top()), egui::Pos2::new(x, rect.bottom())],
                        (0.5, egui::Color32::from_rgba_premultiplied(200, 200, 200, 80))
                    );
                    x += spacing;
                }
            }
            Background::Pdf { .. } => {
                // PDF背景 - 显示占位符
                ui.painter().rect_filled(rect, 0.0, egui::Color32::from_rgba_premultiplied(240, 240, 240, 100));
                ui.painter().text(
                    rect.center(),
                    egui::Align2::CENTER_CENTER,
                    "PDF背景",
                    egui::FontId::proportional(24.0),
                    egui::Color32::from_rgba_premultiplied(150, 150, 150, 150)
                );
            }
        }
    }
    
    fn draw_existing_strokes(&self, ui: &mut egui::Ui, strokes: &[Stroke], rect: egui::Rect) {
        for stroke in strokes {
            self.draw_stroke(ui, stroke, rect);
        }
    }
    
    fn draw_stroke(&self, ui: &mut egui::Ui, stroke: &Stroke, rect: egui::Rect) {
        if stroke.points.len() < 2 {
            return;
        }
        
        let painter = ui.painter();
        let color = self.parse_color(&stroke.color).unwrap_or(egui::Color32::BLACK);
        
        for i in 0..stroke.points.len() - 1 {
            let start = stroke.points[i];
            let end = stroke.points[i + 1];
            
            let start_pos = egui::Pos2::new(
                rect.left() + start.x,
                rect.top() + start.y
            );
            
            let end_pos = egui::Pos2::new(
                rect.left() + end.x,
                rect.top() + end.y
            );
            
            painter.line_segment(
                [start_pos, end_pos],
                (stroke.thickness, color)
            );
        }
    }
    
    fn handle_drawing_input(&mut self, response: &egui::Response, rect: egui::Rect, page: &mut crate::note::Page) {
        if response.dragged() {
            if let Some(pointer_pos) = response.interact_pointer_pos() {
                let point = Point {
                    x: pointer_pos.x - rect.left(),
                    y: pointer_pos.y - rect.top(),
                    timestamp: std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_millis() as u64,
                };
                
                if self.drawing {
                    if let Some(stroke) = &mut self.current_stroke {
                        stroke.points.push(point);
                    }
                } else {
                    self.drawing = true;
                    self.current_stroke = Some(Stroke {
                        points: vec![point],
                        color: self.brush_color.clone(),
                        thickness: self.brush_thickness,
                        pressure: vec![1.0], // 简化压力感应
                    });
                }
            }
        } else if response.drag_released() {
            if let Some(stroke) = self.current_stroke.take() {
                page.strokes.push(stroke);
            }
            self.drawing = false;
        }
    }
    
    fn create_new_note(&mut self) {
        let new_note = Note::new(format!("新笔记 {}", self.notes.len() + 1));
        self.notes.push(new_note.clone());
        self.current_note = Some(new_note);
    }
    
    fn parse_color(&self, color_str: &str) -> Option<egui::Color32> {
        if color_str.starts_with('#') && color_str.len() == 7 {
            let r = u8::from_str_radix(&color_str[1..3], 16).ok()?;
            let g = u8::from_str_radix(&color_str[3..5], 16).ok()?;
            let b = u8::from_str_radix(&color_str[5..7], 16).ok()?;
            
            Some(egui::Color32::from_rgb(r, g, b))
        } else {
            None
        }
    }
}