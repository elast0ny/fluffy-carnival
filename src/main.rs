use std::fs;

fn main() {
    let contents = fs::read_to_string("C:\\file_that_doesnt_exist")
        .expect("Something went wrong reading the file");

    /*
    The panic above will generate something like 
    error: process didn't exit successfully: `target\x86_64-pc-windows-msvc\release\std_tests.exe` (exit code: 0xc000001d, STATUS_ILLEGAL_INSTRUCTION)
    Illegal instruction
    */

    println!("File contents :\n{}", contents);
}
