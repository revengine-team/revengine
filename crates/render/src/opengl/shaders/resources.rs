use std::{fs::File, io::Read, path::Path};

pub fn read_from_file(path: &Path) -> String {
    let mut shader_file =
        File::open(path).expect(format!("Can't open file {:?}", path.to_str()).as_str());

    let mut shader_source = String::new();
    shader_file
        .read_to_string(&mut shader_source)
        .expect(format!("Can't read file {:?}", path.to_str()).as_str());

    shader_source
}
