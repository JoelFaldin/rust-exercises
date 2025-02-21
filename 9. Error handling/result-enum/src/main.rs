use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let greeting_file_result = File::open("test.txt");

    let _reeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("test.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };
}

fn _main2() {
    let _greeting_file_result = File::open("test.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("test.txt").unwrap_or_else(|error| {
                panic!("Error creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {error:?}");
        }
    });
}

fn _main3() {
    let _greeting_file = File::open("hello.txt").unwrap();
    
    let _greeting_file2 = File::open("test.txt")
        .expect("test.txt should be included in this project");
}