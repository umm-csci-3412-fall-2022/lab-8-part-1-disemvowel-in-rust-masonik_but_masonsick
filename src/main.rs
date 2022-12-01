use std::env;
use std::fs;
use std::path::Path;

//The following crates are used for testing
extern crate tempfile; //Creates temp files and directories
use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*;
use std::process::Command; // Run programs // Used for writing assertions

fn main() {
    // This should be called with two command line arguments,
    // both file names. The first should be the name of a file
    // containing the text to disemvowel, and the second should
    // be the file we want to write the disemvoweled text to.
    let args: Vec<String> = env::args().collect();

   //Panic if not enough arguments are provided
    //Panic should output the string "Not enough arguments"
    if args.len() < 3{
        panic!("Not enough arguments");
    } //I don't know if there should be an else statement, lets try without one for now
    

    //TODO:
    //  * Pass an argument to read_file to read the original text
    //  * Pass that to disemvowel to remove the vowels
    //  * Write the disemvoweled text using write_file
    
    // Convert string arguments to pathname object 
    let file_name = Path::new(&args[1]); 

    // Replace String::from("dummy text") with what you get from read_file
    let s = String::from(read_file(file_name));
    let s_disemvowel = disemvowel(&s);

    // Use command-line arguments for the name of the file,
    // and s_disemvowel for the text to write out.
    write_file(Path::new(&args[2]), &s_disemvowel);
}

fn read_file(path: &Path) -> String {
    fs::read_to_string(path).expect("Could not read the file")
}
fn write_file(path: &Path, s: &str) {
    fs::write(path, s).expect("Unable to write file");
}

//Return the input string without vowels.
fn disemvowel(s: &str) -> String {
    String::from(s);
    let vowels = "aeiouAEIOU";
    let mut disemvowel_string = String::from("");

    for i in s.chars() {
        if vowels.contains(i) {
            continue;
        } else {
            disemvowel_string.push(i);
        }
    }
    return disemvowel_string;
}

// Everything from here down is Rust test code. You shouldn't need to
// change any of this.
//
// Use `cargo test` to run all these tests. All the tests will initially
// fail because there's no definition for the `disemvowel` function. Add
// that up above and work to get the tests to pass. See the lab write-up
// for some tips.

#[cfg(test)]
mod tests {
    use super::*;
    mod disemvowel {
        use super::*;
        #[test]
        fn hello_world() {
            let input = "Hello, world!";
            let expected = "Hll, wrld!";

            assert_eq!(expected, disemvowel(input));
        }

        #[test]
        fn empty() {
            assert_eq!("", disemvowel(""));
        }

        #[test]
        fn no_vowels() {
            assert_eq!("pqrst", disemvowel("pqrst"));
        }

        #[test]
        fn all_vowels() {
            assert_eq!("", disemvowel("aeiouAEIOUOIEAuoiea"));
        }

        #[test]
        fn morris_minnesota() {
            assert_eq!("Mrrs, Mnnst", disemvowel("Morris, Minnesota"));
        }

        #[test]
        fn handle_punctuation() {
            assert_eq!(
                "n (nxplnd) lphnt!",
                disemvowel("An (Unexplained) Elephant!")
            );
        }

        #[test]
        fn handle_unicode() {
            assert_eq!("Sm hrglyphs: 𒐁	𒐌	𒐥	𒑳", disemvowel("Some hieroglyphs: 𒐁	𒐌	𒐥	𒑳"));
            assert_eq!("Sm Lnr B: 	𐂀	𐂚	𐃃	𐃺", disemvowel("Some Linear B: 	𐂀	𐂚	𐃃	𐃺"));
            assert_eq!(" lttl Phncn: 𐤀	𐤈	𐤔	𐤕", disemvowel("A little Phoenician: 𐤀	𐤈	𐤔	𐤕"));
            assert_eq!(
                "W cn hndl mj s wll! 🤣😃👍",
                disemvowel("We can handle emoji as well! 🤣😃👍")
            )
        }
    }

    // Tests that check that the correct panics are generated when
    // there aren't the correct number of command line arguments
    // or the input file isn't readable.
    mod panic_tests {
        use super::*;
        #[test]
        fn requires_two_arguments() {
            let mut cmd = Command::cargo_bin("disemvowel-in-rust").unwrap();
            cmd.arg("1");
            cmd.assert()
                .failure()
                .stderr(predicate::str::contains("Not enough arguments"));
        }
        #[test]
        fn requires_read_file() {
            let mut cmd = Command::cargo_bin("disemvowel-in-rust").unwrap();
            cmd.arg("/this/path/does/not/exist")
                .arg("output/path/doesnt/matter");
            cmd.assert()
                .failure()
                .stderr(predicate::str::contains("Could not read the file"));
        }
    }
}
