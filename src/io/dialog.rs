use crate::model::dia::diafile::DiaFile;

use super::file;

#[allow(unused)]
pub fn open_diagram_dialog() -> DiaFile {
    let response = rfd::FileDialog::new().pick_files();
    let path = &response.unwrap()[0];
    let bytes = std::fs::read(path.as_path().to_str().unwrap()).unwrap();
    file::deserialize(&String::from_utf8(bytes).unwrap()).unwrap()
}
