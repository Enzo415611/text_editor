use std::{collections::HashMap, hash::Hash, path::{Path, PathBuf}};
use eframe::{NativeOptions};
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
    tabs: HashMap<String, Tabs>,
}

pub struct Tabs {
    current_path: PathBuf,    
    is_open: bool,
}
impl Tabs {
    fn new(current_path: PathBuf, is_open: bool) -> Self {
        Self {
            current_path,
            is_open
        }
    }
}

impl Editor {
    pub fn new(_cc: &eframe::CreationContext) -> Self {
        Self {
            content: String::new(),
            root: None,
            current_file: None,
            auto_save_is_active: false,
            syntax_theme: ColorTheme::SONOKAI,
            window_open: false,
            tabs: HashMap::new()                      
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

