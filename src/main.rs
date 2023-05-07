mod app;
mod command;

use std::io;

fn main() {
    let app = app::App::new(String::from("./"));
    println!("Your current library path: {}", &app.path_to_library);

    loop {
        let mut query = String::new();
        
        app::App::quik_print("query: ".to_string());
        io::stdin().read_line(&mut query).expect("Read failure");
        let query = query.trim();

        let query = command::Command::from_string(query);

        match query {
            Some(cont) => {
                match cont {
                    command::Command::List => app.list_contents(),
                    _ => (),
                }
            },
            None => {}
        }
    }
}