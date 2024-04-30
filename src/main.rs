use std::env;
use std::fs;
use std::io;
use std::io::Write;
use std::path::Path;

fn main() {
    let mut shell = true;
    while shell {
        let mut input = String::new();
        print!(">> ");
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
            println!("files/folders {:?}", ls.execute());
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
        println!("entries: {:?}", entries);

        for entry in &entries {
            let new_path = Path::new(entry);
            let file_stem = new_path.file_stem();
            print!("{:?}", file_stem.unwrap());
        }
        // todo: change return type and remove quotes from result

        Ok(())
    }
}

struct CatCommand;
impl CatCommand {
    fn execute(&self) {
        println!("Cat command execute")
    }
}
