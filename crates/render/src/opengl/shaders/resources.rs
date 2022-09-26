use std::{fs::File, io::Read, path::Path};

pub fn read_from_file(path: &Path) -> String {
    let mut shader_file =
        File::open(path).unwrap_or_else(|_| panic!("Can't read file {:?}", path.to_str()));

    let mut shader_source = String::new();
    shader_file
        .read_to_string(&mut shader_source)
        .unwrap_or_else(|_| panic!("Can't read file {:?}", path.to_str()));

    shader_source
}
