use std::io;
use std::io::{Write};
use std::fs;

pub struct App {
    pub path_to_library: String,
}
impl App {
    pub fn new(path_to_library: String) -> App {
        App {
            path_to_library,
        }
    }

    pub fn quik_print(to_print: String) {
        print!("{to_print}");
        
        io::stdout().flush().unwrap();   
    }

    pub fn separator() {
        println!("****")
    }

    pub fn list_contents(&self) {
        let files = fs::read_dir(self.path_to_library.clone()).unwrap();
        for path in files {
            let path = path.unwrap();

            let f_type = match path.file_type().unwrap().is_dir() {
                true => "DIRECTORY",
                _ => "FILE     "
            };

            let f_name = path.file_name();
            let f_name = f_name.to_string_lossy();

            println!("{} § {}", f_type, f_name);
        }
        App::separator();
    }
}