use std::{path::PathBuf};
use eframe::{ NativeOptions};

mod app;


// ponto de entrada do programa
fn main() {    
   _ = eframe::run_native(
        "Editor",
         NativeOptions::default(),
          Box::new(|cc| Ok(Box::new(Editor::new(cc)))));
}


// struct principal da aplicação
#[derive()]
pub struct Editor{
    content: String,
    root: Option<FileNode>,
    current_file: Option<PathBuf>,
    auto_save_is_active: bool,
}


impl Editor {
    pub fn new(_cc: &eframe::CreationContext) -> Self {
        Self {
            content: String::new(),
            root: None,
            current_file: None,
            auto_save_is_active: false,                           
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




