// a rather simplistic implementation of ls
use std::env::args;
use std::env::Args;
use std::ffi::OsStr;
use std::fs;
use std::io;

// Current state took me one hour from scratch

fn print_dir(path: &String, one_arg: bool) -> Result<(), io::Error> {
    //read_dir return Result<Path, Err>
    let entries = match fs::read_dir(path) {
        Ok(entries) => entries,
        Err(e) => return Err(e),
    };
    // Adds path heading
    if one_arg == false {
        println!("{}:", path);
    }
    for each in entries {
        // each has the type Result<DirEntry, Err>
        match each {
            Ok(dir) => {
                let path = dir.path();
                // path.file_name() return Option<&OsStr>
                let file_name = match path.file_name() {
                    Some(file) => file,
                    None => &OsStr::new(""), // Since both arms have to match
                };
                // Since file_name.to_str() return Option<&str>
                let file_name_string: &str = match file_name.to_str() {
                    Some(string) => string,
                    None => "",
                };
                println!("{}", file_name_string);
            } //println!("{:#?}", dir.path()),
            // if there's and error in reading the entry
            Err(e) => return Err(e),
        };
    }
    // After completion of reading the argument -> add a new line.
    println!("");
    // Function ran successfully
    Ok(())
}

fn main() {
    // Starting tests
    //let mut i = 0;
    // for arg in args().next() {
    //      println!("arg {}: {}", i, arg);
    //       i += 1;
    //    }
    let arguments: Args = args();
    // No minimum arguments, but don't need to type out the heading of the file, if it's just ls.
    if arguments.len() < 2 {
        match print_dir(&String::from("."), true) {
            Ok(_) => return,
            Err(e) => {
                println!("There was an error: {}", e);
                return;
            }
        };
    }
    // First argument is the program itself
    for each in arguments.skip(1) {
        let res = print_dir(&each, false);
        // print_dir returns Return<(), io::Error>, match handles the error.
        match res {
            Ok(_) => continue,
            Err(e) => {
                println!("There was an error: {}", e);
                return;
            }
        }
    }
}
