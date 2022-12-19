use std::env;
use std::fs;

fn main() {
    /* Creating an iterator which will the arguments supplied. Our 
     collector in this case is of type - Vector */
    let args: Vec<String> = env::args().collect();
    // dbg!(args);
    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {}",query);
    println!("In file {}", file_path);

    /* fs::read_to_string takes the file_path, opens that file, 
    and returns a std::io::Result<String> of the file’s contents */
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}");    
}
