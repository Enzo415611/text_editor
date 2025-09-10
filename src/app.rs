use std::{fs::{read_dir, read_to_string, write}, path::{Path, PathBuf}, thread, time::Duration};
use eframe::{App, Frame};
use egui::{CentralPanel, Context, SidePanel, TextBuffer, TopBottomPanel, Ui};
use egui_code_editor::{CodeEditor, ColorTheme, Syntax};
use rfd::FileDialog;

use crate::{Editor, FileNode};


impl App for Editor {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        let clone_auto_save_file = self.auto_save_file().clone(); 
        let clone_detect_current_language = self.detect_current_language().clone();
        thread::spawn(move || {
            _ = clone_auto_save_file;
            _ = clone_detect_current_language;
            thread::sleep(Duration::from_millis(1));
        });

        
        TopBottomPanel::top("top").resizable(false).show(ctx, |ui| {

            ui.horizontal(|ui| {
                
                ui.menu_button("File", |ui| {
                    if ui.button("Open File").clicked() {
                        self.open_explorer_file()
                }

                
                if ui.button("Open Folder").clicked() {
                        self.open_explorer_folder()
                }

                let auto_save_is_active_str = if self.auto_save_is_active {"ON"} else {"OFF"}; 
                if ui.button(format!("Auto Save: {:?}", auto_save_is_active_str)).clicked() {
                        self.auto_save_is_active = !self.auto_save_is_active;
                }

                if ui.button("Save").clicked() {
                    if let Some(path) = &self.current_file {
                        if let Err(e) = write(path, &self.content) {
                            eprintln!("Error Saving: {:?}",e);
                        }
                    }
                }
                });
                
            });

            ui.add_space(2.0)
        });

        SidePanel::left("side")
            .resizable(true)
            .default_width(200.0)
            .min_width(50.0)
            .max_width(700.0)
            .show(ctx, |ui| {
                if self.root.is_none() {
                    ui.label("Select The Folder");
                    return;
                }
                let root: FileNode = self.root.as_ref().unwrap().clone();
                self.render_tree(ui, &root);
                ui.cursor();
                ui.allocate_space(ui.available_size());

        });

        CentralPanel::default().show(ctx, |ui| {
            CodeEditor::default()
                        .id_source("Code Editor")
                        .with_rows(20)
                        .with_fontsize(14.0)
                        .with_theme(ColorTheme::SONOKAI)
                        .with_syntax(self.detect_current_language())
                        .with_numlines(true)
                        .show(ui, &mut self.content);            
        });
    }
}





impl Editor {
    //
    fn render_tree(&mut self, ui: &mut Ui, node: &FileNode) {
        if node.is_dir {
            egui::CollapsingHeader::new(&node.name)
                .default_open(false)
                .show(ui, |ui| {
                    for child in &node.children {
                        self.render_tree(ui,child)
                    }
                });
        } else {
            if ui.button(&node.name).clicked() {
                self.open_file_content(&node.path);
            }
        }
    }


    // fn que abre o explorador de arquivo para selecionar uma pasta
    fn open_explorer_folder(&mut self){
        if let Some(path) = FileDialog::new().pick_folder() {
            self.root = Some(Self::build_tree(path));
        }
    }

    // fn que abre o explorador de arquivo para selecionar um arquivo
    fn open_explorer_file(&mut self){
        if let Some(path) = FileDialog::new().pick_file() {
            self.root = Some(Self::build_tree(path));
        }
    }

    // construção da arvore de arquivos
    fn build_tree(path: PathBuf) -> FileNode {
        let is_dir = path.is_dir();
        let mut children = Vec::new();


        // pego todos os arquivos de dentro da pasta selecionada
        if is_dir {
            if let Ok(entries) = read_dir(&path) {
                for entry in entries.flatten() {
                    children.push(Self::build_tree(entry.path()));
                }
            }
        }

        FileNode {
            name: path.file_name().unwrap_or_default().to_string_lossy().to_string(),
            path,
            is_dir,
            children
        }
    }
    //
    // fn que faz a leitura do conteudo do arquivo

    // passo o path do arquivo a fn read_to_string que me retorna o conteudo do arquivo no formato de uma string
    // atualizo o content com o retorno da fn read_to_string
    // atualizo o current_file para saber qual arquivo esta sendo editado no momento
    fn open_file_content(&mut self, path: &Path) {
        if let Ok(text) = read_to_string(path) {
            self.content = text;
            self.current_file = Some(path.to_path_buf());
        }
    }


    // fn que salva o arquivo toda vez que o usuário modifica o conteúdo
    // Verifica se o auto_save_is_active está ativo
    // Obtém o path do arquivo que está sendo editado
    // Obtém o conteúdo atual do arquivo que está sendo editado
    // faço uma comparação se o arquivo aberto no Editor tem o conteudo diferente do arquivo original
    // se for diferente eu chamo a fn [write] que escreve a minha modificação no arquivo original 
    fn auto_save_file(&self) {
        if self.auto_save_is_active {
            if let Some(path) = &self.current_file {
                if let Ok(file) =  read_to_string(path){
                    if self.content != file {
                        if let Err(e) = write(path, self.content.clone()) {
                            eprintln!("Error Saving: {:?}", e)
                        }
                    }
                }
            }
        }        
    }

    // fn que detecta o tipo do arquivo que está sendo editado ex: .rs .txt .py
    fn detect_current_language(&self) -> Syntax {
        if let Some(path) = &self.current_file {
            let type_file = path.extension();
            match type_file {
                Some(extension) => match extension.to_string_lossy().as_str() {
                   "rs" => {println!("rs"); Syntax::rust() },
                   "py" => {println!("py"); Syntax::python() },
                   "txt" => {println!("txt"); Syntax::new("txt")},
                    _ => {println!("default"); Syntax::default()}
                },
                _ => Syntax::default()
            }
        } else {
            Syntax::default()
        }     
    }
}
