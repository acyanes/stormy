use std::io;
//use std::io::*;

fn main() {
    let mut shell = true;
    let mut input = String::new();
    while shell {
        // if user hits enter, then read in whats on current line

        io::stdin()
            .read_line(&mut input)
            .expect("error: unable to read user input");

        // create an enum that has potential commands
        // take in user input
        // print user input

        // if a key is pressed, exit program
        //
        //if run
    }
}

// fn main() {
//     let mut input = String::new();
//     io::stdin()
//         .read_line(&mut input)
//         .expect("error: unable to read user input");
//     println!("{}", input);
// }
