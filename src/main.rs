use std::{path::PathBuf, sync::Arc};
use eframe::{NativeOptions};

mod app;


// ponto de entrada do programa
fn main() {
    let d = eframe::icon_data::from_png_bytes(include_bytes!("../assets/OI.ico"))
    .expect("The icon data must be valid");
    let mut native_options = NativeOptions::default();
    native_options.viewport.icon = Some(Arc::new(d));

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




