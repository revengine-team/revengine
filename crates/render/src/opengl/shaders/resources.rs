use std::{path::Path, fs::File, io::Read};

pub fn read_from_file(path: &Path) -> String {
    let maybe_shader_file = File::open(path);
    
    match maybe_shader_file {
        Ok(mut shader_file) => {
            let mut shader_source = String::new();
            let read_result = shader_file.read_to_string(&mut shader_source);

            match read_result {
                Ok(_) => {
                    shader_source
                }
                
                Err(error) => {
                    panic!("Can't read file: {:?}", error);
                }
            }
        }

        Err(error) => {
            panic!("Can't open file: {:?}", error);
        }
    }
}