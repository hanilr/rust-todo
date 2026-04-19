use serde::{Serialize, Deserialize};
use std::fs;
use std::io::Write;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileType {
    name: String,
    extension: String,
    content: String,
    created_at: String,
}

pub struct FolderType {
    name: String,
    base_path: PathBuf,
}

impl FolderType {
    pub fn new(name: &str) -> Self {
        let mut path = PathBuf::from("data");
        path.push(name);
        fs::create_dir_all(&path).expect("Folder not created.");

        FolderType {
            name: name.to_string(),
            base_path: path,
        }
    }

    pub fn add_file(&self, name: &str, extension: &str, content: &str) {
        let file_data = FileType {
            name: name.to_string(),
            extension: extension.to_string(),
            content: content.to_string(),
            created_at: "2026-04-15".to_string(),
        };

        let mut file_path = self.base_path.clone();
        file_path.push(format!("{}.json", name));

        let json_data = serde_json::to_string_pretty(&file_data).expect("JSON not created.");

        let mut f = fs::File::create(file_path).expect("File not created.");
        f.write_all(json_data.as_bytes()).expect("File not wrote.");
        
        println!("'{}' file saved as JSON.", name);
    }

    pub fn read_file(&self, name: &str) -> Option<FileType> {
        let mut file_path = self.base_path.clone();
        file_path.push(format!("{}.json", name));

        if let Ok(json_str) = fs::read_to_string(file_path) {
            let file_data: FileType = serde_json::from_str(&json_str).expect("JSON deserialize error.");
            Some(file_data)
        } else {
            None
        }
    }

    pub fn remove_file(&self, name: &str) {
        let mut file_path = self.base_path.clone();
        file_path.push(format!("{}.json", name));
        
        let _ = fs::remove_file(file_path);
        println!("'{}' deleted.", name);
    }
}