use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;
use std::collections::BTreeMap;

mod tokeniser;



pub fn assemble (input: String, output: String) {
    println!("Assembling '{}' => '{}'", input, output);
    /* Open file */
    let f = File::open(&input).unwrap();
    let reader = BufReader::new(f);
    let lines = reader.lines();
    /* Pass 1 - Store addresses of labels in a symbol table */
    let mut sym_tab = BTreeMap::new();
    let mut address : u32 = 0;
    for l in lines {
        let t = tokeniser::tokenise_line(l.unwrap());

        match t {
            None => (),
            Some(tokens) => { 
                    match tokens.label {
                        // TODO: Add to some symbol table
                        Some(l) => {
                            println!("Found label '{}' at address: {}", l, address);
                            sym_tab.insert(address,l);
                        }
                        _ => (),
                    }
                    match tokens.opcode {
                        Some(ref op) if op == ".skip" => address +=
                            tokens.operand1.unwrap().parse::<u32>().unwrap() * 4, 
                        _ => address += 4,
                    }
            }
        }

    }

    println!("Symbol Table: {:?}", sym_tab);
    let f = File::open(input).unwrap();
    let reader = BufReader::new(f);
    let lines = reader.lines();
    for l in lines {
        
    }
}
