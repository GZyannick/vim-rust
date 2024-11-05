use std::path::PathBuf;

pub trait LineLen {
    fn len(&self) -> usize;
}

#[derive(Debug, Clone)]
pub enum AppLines {
    Explorer((String, PathBuf)),
    File(String),
}

impl ToString for AppLines {
    fn to_string(&self) -> String {
        match self {
            AppLines::Explorer((str, _)) => str.to_string(),
            AppLines::File(str) => str.to_string(),
        }
    }
}

/// Only give the len of str in enum (not pathbuf)
impl LineLen for AppLines {
    fn len(&self) -> usize {
        match self {
            AppLines::Explorer((str, _)) => str.len(),
            AppLines::File(str) => str.len(),
        }
    }
}
