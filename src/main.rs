use std::env;
use std::fs;
use std::io;
use std::io::Write;
use std::path::Path;

fn main() {
    let mut shell = true;
    while shell {
        let mut input = String::new();
        print!("stormy> ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut input)
            .expect("error: unable to read user input");

        // clean the input string
        let input = input.trim();

        // create an enum that has potential commands
        // take in user input
        // print user input
        if input == "ls" {
            let ls = LsCommand;
            println!("{:?}", ls.execute());
        }

        if input == "pwd" {
            let pwd = PwdCommand;
            println!("{:?}", pwd.execute());
        }

        if input == "exit" {
            shell = false;
        }
    }
}

struct LsCommand;
impl LsCommand {
    fn execute(&self) -> io::Result<()> {
        let path = env::current_dir()?;
        let mut entries = fs::read_dir(path)?
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, io::Error>>()?;

        entries.sort();

        for entry in &entries {
            let new_path = Path::new(entry);
            let file_stem = new_path.file_stem();
            let file_extension = new_path.extension();
            let name = file_stem.unwrap().to_string_lossy();

            if let Some(extension) = file_extension {
                print!(
                    "{}.{} ",
                    name.replace(' ', r"\ "),
                    extension.to_str().unwrap()
                );
            } else {
                print!("{} ", name.replace(' ', r"\ "));
            }
        }
        Ok(())
    }
}

struct CatCommand;
impl CatCommand {
    fn execute(&self) {
        println!("cat command");
    }
}

struct PwdCommand;
impl PwdCommand {
    fn execute(&self) {
        let working_dir = env::current_dir().unwrap();
        println!("{:?}", working_dir.to_str().unwrap());
    }
}
struct ChangeDirectoryCommand;
struct RemoveCommand;
