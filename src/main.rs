use walkdir::WalkDir;
use std::io;

fn main() {
    let mut searchdir = String::new();
    
    loop{

        println!("Enter the directory you want to search");

        io::stdin().read_line(&mut searchdir).expect("Failed to read line");
        if searchdir.ends_with('\n') || searchdir.ends_with('\r') {
            searchdir.pop();
        }


        for file in WalkDir::new(&searchdir).into_iter().filter_map(|e| e.ok()) {
            if file.metadata().unwrap().is_file(){
                println!("{}", file.path().display());
            }
        }
    }
}
