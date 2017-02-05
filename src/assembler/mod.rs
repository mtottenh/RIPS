use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;
use std::collections::BTreeMap;
use std::collections::HashMap;
mod tokeniser;
type InstrAssembleFn = fn(&tokeniser::TokenisedLine) -> u32;

const i_type_instr : [&'static str; 12] = [ "addi", "subi", "muli", "lw", "sw", "beq", "bne",
"blt", "bgt", "ble", "bge", "mov" ];
const r_type_instr : [&'static str; 5] = [ "add", "sub", "mul", "jr", "out" ];
const j_type_instr : [&'static str; 1] = [ "jmp" ];


fn assemble_i_type (line: &tokeniser::TokenisedLine) -> u32 {
    println!("Assembling I type instruction");
    return 1;
}

fn assemble_j_type (line: &tokeniser::TokenisedLine) -> u32 {
    println!("Assembling J type instruction");
    return 1;
}

fn assemble_r_type (line: &tokeniser::TokenisedLine) -> u32 {
    println!("Assembling R type instruction");
    return 1;
}

fn assemble_fill (line: &tokeniser::TokenisedLine) -> u32 {

    return 1;
}
fn assemble_skip (line: &tokeniser::TokenisedLine) -> u32 {

    return 1;
}

fn assemble_halt (line: &tokeniser::TokenisedLine) -> u32 {

    return 1;
}

struct InstrFactory {
    table: HashMap<String, InstrAssembleFn>
}

impl InstrFactory {
    fn new () -> InstrFactory {
        let mut i = InstrFactory { table: HashMap::new() };


        for ins in i_type_instr.iter() {
            i.table.insert(ins.to_string(),assemble_i_type);
        }
        for ins in r_type_instr.iter() {
            i.table.insert(ins.to_string(),assemble_r_type);
        }
        for ins in j_type_instr.iter() {
            i.table.insert(ins.to_string(),assemble_j_type);
        }

        // Assembler directives
        i.table.insert(".fill".to_string(), assemble_fill);
        i.table.insert(".skip".to_string(), assemble_skip);
        i.table.insert("halt".to_string(), assemble_halt);
        return i
    }
    fn create (&self, ins: &String, line: &tokeniser::TokenisedLine) -> u32 {
        println!("Assembling op: {}", ins);
        if self.table.contains_key(ins) {
            self.table[ins](line);
        } else {
            panic!("Invalid instruction {}", ins);
        }
        return 1;
    }
}



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

    let f = File::open(&input).unwrap();
    let reader = BufReader::new(f);
    let lines = reader.lines();
    let fac = InstrFactory::new();
    /* Pass 2 - assemble instructions */
    for l in lines {
         let t = tokeniser::tokenise_line(l.unwrap());
         match &t {
            &None => (),
            &Some(ref tokens) => {
                match &tokens.opcode {
                    &None => (),
                    &Some(ref op) => {
                        fac.create(op,tokens);
                    }
                }
            }
         }
    }
}
