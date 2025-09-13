use std::{path::PathBuf, sync::Arc};
use eframe::{App, NativeOptions};
use egui::{CentralPanel, FontData, FontDefinitions};
use egui_code_editor::ColorTheme;

mod app;


// ponto de entrada do programa
fn main() {    
    let native_options = NativeOptions::default();
    _ = eframe::run_native(
        "Text Editor",
        native_options,
        Box::new(|cc| Ok(Box::new(Editor::new(cc)))));
}


// struct principal da aplicação
#[derive()]
pub struct Editor{
    content: String,
    root: Option<FileNode>,
    current_file: Option<PathBuf>,
    auto_save_is_active: bool, 
    syntax_theme: ColorTheme,
    window_open: bool,
}

impl Editor {
    pub fn new(_cc: &eframe::CreationContext) -> Self {
        Self {
            content: String::new(),
            root: None,
            current_file: None,
            auto_save_is_active: false,
            syntax_theme: ColorTheme::SONOKAI,
            window_open: false
        }
    }
}

// struct da arvore de arquivos
#[derive(Clone)]
pub struct FileNode {
    name: String,
    path: PathBuf,
    is_dir: bool,
    children: Vec<FileNode>    
}

