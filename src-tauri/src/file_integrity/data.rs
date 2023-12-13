use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct FileList {
    pub date: String,
    pub files: Vec<FileInfo>,
}

#[derive(Serialize, Deserialize)]
pub struct FileInfo {
    pub filename: String,
    pub md5_hash: String,
}
