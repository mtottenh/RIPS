use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;
mod tokeniser;

pub fn assemble (input: String, output: String) {
    println!("Assembling '{}' => '{}'", input, output);
    /* Open file */
    let f = File::open(input).unwrap();
    let reader = BufReader::new(f);
    let lines = reader.lines(); 
    for l in lines {
        let t = tokeniser::tokenise_line(l.unwrap());
        println!("Tokenised Line: {:?}", t);
    }
        
}
