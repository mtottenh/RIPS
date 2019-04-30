use std::fs::File;

mod machine_state {
    use std::io::Cursor;
    use std::io::prelude::*;
    use std::fs::File;
    use std::convert::TryInto;
    use super::decoder;
    use super::decoder::Instruction;
    #[derive(Debug, Default)]
    pub struct State {
        program_counter: u16,
        stack_pointer: usize,
        stack_size: u16,
        stack_boundary: usize,
        regfile: Vec<u32>,
        memory: Vec<u8>,

    }

    impl State {
        /// Takes two arguments 'memory_size' and 'num_registers'
        pub fn new(memory_size: usize, num_registers: usize) -> State {
            State { memory: vec![0; memory_size],
                    regfile: vec![0; num_registers],
                    stack_pointer: memory_size - 5,
                    stack_size: 1024,
                    stack_boundary: memory_size - 5 - 1024,
                    program_counter: 0 }
        }
        pub fn get_current_instruction(&self) -> u32  {
            let slice: &Vec<u8> = &self.memory;
            u32::from_le_bytes(slice[self.program_counter as usize
                               .. (self.program_counter + 4) as usize].try_into().unwrap())
        }
        pub fn load_memory_from_file(&mut self, input_file: &mut File) -> usize {
            let read_bytes = input_file.read(&mut  self.memory).unwrap();
            if read_bytes < self.memory.len() {
                eprintln!("Warning! read {} bytes, but memory size is {} bytes", read_bytes, self.memory.len());
            }
            read_bytes
        }
        pub fn set_memory(&mut self, location: usize, value: u32) {
            self.memory[location] = ((value >> 24) & 0xFF) as u8;
            self.memory[location+1] = ((value >> 16) & 0xFF) as u8;
            self.memory[location+2] = ((value >>  8) & 0xFF) as u8;
            self.memory[location+3] = (value & 0xFF) as u8;
        }
        pub fn read_word(&self, location: usize) -> u32 {
            let mut value: u32 = ((self.memory[location] & 0xFF) as u32) << 24;
            value |= ((self.memory[location+1] & 0xFF) as u32) << 16;
            value |= ((self.memory[location+2] & 0xFF) as u32) << 8;
            value |= (self.memory[location+3] & 0xFF) as u32;
            value
        }
        /// Increments the program counter by `steps`
        pub fn increment_pc(&mut self, steps: u16) {
            self.program_counter += steps * 4;
        }
        /// Print the values of PC and the machine registers.
        pub fn halt_instruction(&self) -> () {
            eprint!("PC: 0x{:x}\t",self.program_counter);
            let mut num_cols = 1;
            for i in 0..self.regfile.len() {
                eprint!("R{:02}: 0x{:x}", i, self.regfile[i]);
                num_cols += 1;
                if num_cols >= 4 {
                    eprint!("\n");
                    num_cols = 0;
                } else {
                    eprint!("\t");
                }
            }
            eprint!("\n");
        }

        pub fn add_instruction(&mut self, instruction: &u32) {
            eprintln!("add called");
            let ins = decoder::decode(instruction);
            eprintln!(" * Repr = {:?}", ins); 
            match ins {
                Some(Instruction::RType{Opcode,r1,r2,r3}) => {
                    self.regfile[r1.unwrap() as usize] = self.regfile[r2.unwrap() as usize] 
                                                            + self.regfile[r3.unwrap() as usize];
                                                                          
                },
                _ => {}
            };
            eprintln!("REGFILE: {:?}", self.regfile);
        
        }

        pub fn addi_instruction(&mut self, instruction: &u32) {
            eprintln!("addi called");
            let ins = decoder::decode(instruction);
            eprintln!(" * Repr = {:?}", ins); 
            match ins {
                Some(Instruction::IType{Opcode,r1,r2,imm}) => {

                    self.regfile[r1.unwrap() as usize] = self.regfile[r2.unwrap() as usize] 
                                                            + imm.unwrap() as u32;
                },
                _ => {}
            };
            eprintln!("REGFILE: {:?}", self.regfile);
        }
        pub fn sub_instruction(&mut self, instruction: &u32) {
        }
        pub fn subi_instruction(&mut self, instruction: &u32) {
            eprintln!("subi called"); 
            let ins = decoder::decode(instruction);
            eprintln!(" * Repr = {:?}", ins);
            match ins {
                Some(Instruction::IType{Opcode, r1, r2, imm}) => {
                    self.regfile[r1.unwrap() as usize] = (self.regfile[r2.unwrap() as usize] as i32 - (imm.unwrap() as i32)) as u32;
                },
                _ => {}
            };
        }
        pub fn mul_instruction(&mut self, instruction: &u32) {
        }
        pub fn muli_instruction(&mut self, instruction: &u32) {
            eprintln!("muli called"); 
            let ins = decoder::decode(instruction);
            eprintln!(" * Repr = {:?}", ins); 
            match ins {
                Some(Instruction::IType{Opcode, r1 ,r2, imm}) => {
                    //self.regfile[r1] = self.regfile[r2] * imm
                    eprintln!("Attempting: {} * {}", self.regfile[r2.unwrap() as usize], imm.unwrap() as u32);
                    self.regfile[r1.unwrap() as usize] = (self.regfile[r2.unwrap() as usize] as i32 * (imm.unwrap() as i32)) as u32;
                },
                _ => {}
            };
        
        }
        pub fn lw_instruction(&mut self, instruction: &u32) {
            eprintln!("lw called");
            let ins = decoder::decode(instruction);
            eprintln!(" * Repr = {:?}", ins); 
            match ins { 
                Some(Instruction::IType{Opcode,r1,r2,imm}) => {
                    let location: usize = (self.regfile[r2.unwrap() as usize] + imm.unwrap() as u32) as usize;
                    //TODO - check_mem_access?
                    self.regfile[r1.unwrap() as usize] = self.read_word(location);
                },
                _ => {}
            };      
        }
        pub fn sw_instruction(&mut self, instruction: &u32) {
            eprintln!("sw called");
            let ins = decoder::decode(instruction);
            eprintln!(" * Repr = {:?}", ins); 
            match ins {
               Some(Instruction::IType{Opcode,r1,r2,imm}) => {
                   let location: usize = (self.regfile[r2.unwrap() as usize] + imm.unwrap() as u32) as usize;
                   //TODO - check_mem_access?
                   let value = self.regfile[r1.unwrap() as usize];
                   self.set_memory(location,value);
               },
               _ => {}
            };      
        }
        pub fn beq_instruction(&mut self, instruction: &u32) {
        }
        pub fn bne_instruction(&mut self, instruction: &u32) {
        }
        pub fn blt_instruction(&mut self, instruction: &u32) {
        }
        pub fn bgt_instruction(&mut self, instruction: &u32) {
        }
        pub fn ble_instruction(&mut self, instruction: &u32) {
        }
        pub fn bge_instruction(&mut self, instruction: &u32) {
        }
        pub fn jmp_instruction(&mut self, instruction: &u32) {
        }
        pub fn jr_instruction(&mut self, instruction: &u32) {
        }    
        pub fn jal_instruction(&mut self, instruction: &u32) {
        }
        pub fn out_instruction(&mut self, instruction: &u32) {
            eprintln!("out called");
            let ins = decoder::decode(instruction);
            eprintln!(" * Repr = {:?}", ins); 
            match ins {           
                Some(Instruction::RType{Opcode, r1, r2, r3}) => {
                    print!("{}", ((self.regfile[r1.unwrap() as usize] & 0xFF) as u8) as char); 
                },
                _ => {}
            }
        }

    }
}
mod decoder {
    use super::machine_state;
    #[derive(Debug)]
    pub enum InstructionType {
        R,
        I,
        J,
        S,
        HALT,
        FILL,
        SKIP
    }
    /* Instruction formats are bleow,
     *
     * R-Type Insructions
     * OPCODE |  R 1  |  R 2  |   R 3  | Unused |
     *   6    |   5   |   5   |    5   |  11    |
     *
     * I-Type Instructions
     * OPCODE |  R 1  |  R 2  | Immediate Value |
     *   6    |   5   |   5   |     16          |
     *
     * J-Type Instructions
     * OPCODE |  Unused   | Immediate Value |
     *   6    |   10      |     16          |
     *
     * S-Type Instructions
     * OPCODE | FLAGS | Immediate Value |
     *   6    |   2   |      24         |
     *  
     */
    #[derive(Debug)]
    pub enum Instruction {
        RType { 
            Opcode: u8,
            r1: Option<u8>, 
            r2: Option<u8>, 
            r3: Option<u8> },
        IType {
            Opcode: u8,
            r1: Option<u8>,
            r2: Option<u8>,
            imm: Option<i16>
            },
        JType { Opcode: u8, imm: Option<i16> },
        Halt,
    }
    pub fn decode(instruction: &u32) -> Option<Instruction> {
        match get_type(instruction) {
            Some(InstructionType::R) => Some(decode_r(instruction)),
            Some(InstructionType::I) => Some(decode_i(instruction)),
            Some(InstructionType::J) => Some(decode_j(instruction)),
            _ => None,
        }
    }
    fn decode_r(instruction: &u32) -> Instruction {
        let Opcode = opcode(instruction);
        let r1 = register(instruction, 1);
        let r2 = register(instruction, 2);
        let r3 = register(instruction, 3);
        
        Instruction::RType{Opcode,r1,r2,r3 }
    }
    fn decode_i(instruction: &u32) -> Instruction {
        let Opcode = opcode(instruction);
        let r1 = register(instruction, 1);
        let r2 = register(instruction, 2);
        let imm = immediate(instruction);
        Instruction::IType{ Opcode, r1, r2, imm }
    }
    fn decode_j(instruction: &u32) -> Instruction {
        let Opcode = opcode(instruction);
        let imm = immediate(instruction);
        Instruction::JType {Opcode, imm}
    }
    /// Extract the immediate value from an instruction, only defined on I/J types initially.
    fn immediate(instruction: &u32) -> Option<i16> {
        eprintln!("** Extracting immediate operand of {:?}", get_type(instruction));
        match get_type(instruction).unwrap() {
            InstructionType::I | InstructionType::J =>  Some((instruction & 0xFFFF) as i16),
            _ => None
        }
    }
    /// Extract the register field reg_noo from instruciton.
    fn register(instruction: &u32, reg_no: u8) -> Option<u8> {
        // First, some sanity checking.
        if reg_no > 3 { return None };
        match get_type(instruction).unwrap() {
            InstructionType::I => { if reg_no > 2 { return None } },
            InstructionType::J => { if reg_no > 0 { return None } },
            InstructionType::R => {},
            _ => return None
        };
        // Now some bit twiddling
        let REG_WIDTH = 5;
        let OPCODE_WIDTH=6;
        let RTYPE_UNUSED_PADDING = 11;
        let regs =  *instruction;
        let ret = match reg_no {
            1 => (regs >> (32 - (OPCODE_WIDTH + REG_WIDTH))) & 0x1F,
            2 => (regs >> (32 - (OPCODE_WIDTH + REG_WIDTH *2) )) & 0x1F,
            3 => (regs >> (32 - (OPCODE_WIDTH + REG_WIDTH *3)))& 0x1F,
            _ => 0,
        };

        // And sanity check the reuslt.
        if ret > 31 {
            None
        } else {
            Some(ret as u8)
        }
        
    }
    /// Opcodes are the highest 6 bytes of a given instruction.
    pub fn opcode (instruction: &u32) -> u8 {
        let OPCODE_WIDTH = 6;
        ( instruction >> (32 - OPCODE_WIDTH) )as u8
    }
    pub fn get_type(instruction: &u32) -> Option<InstructionType> {
        match opcode(instruction) {
            0 => Some(InstructionType::HALT),
            1|3|5|16|18|26 => Some(InstructionType::R),
            2|4|6|7|8|9|10|11|12|13|14|25 => Some(InstructionType::I),
            15|17 => Some(InstructionType::J),
            19 => Some(InstructionType::FILL),
            20 => Some(InstructionType::SKIP),
            21|22|23|24 => Some(InstructionType::S),
            _ => None 
        }
    }
    pub fn dispatch(machine: &mut machine_state::State, instruction: &u32) -> () {
        // This *should* be fairly efficient, and compile to a jump table?
        match opcode(instruction)  {
            0  => machine.halt_instruction(),
            1  => machine.add_instruction(instruction), 
            2  => machine.addi_instruction(instruction),
            3  => machine.sub_instruction(instruction),
            4  => machine.subi_instruction(instruction),
            5  => machine.mul_instruction(instruction),
            6  => machine.muli_instruction(instruction),
            7  => machine.lw_instruction(instruction),
            8  => machine.sw_instruction(instruction),
            9  => machine.beq_instruction(instruction),
            10 => machine.bne_instruction(instruction),
            11 => machine.blt_instruction(instruction), 
            12 => machine.bgt_instruction(instruction),
            13 => machine.ble_instruction(instruction),
            14 => machine.bge_instruction(instruction),
            15 => machine.jmp_instruction(instruction),
            16 => machine.jr_instruction(instruction),
            17 => machine.jal_instruction(instruction),
            18 => machine.out_instruction(instruction),
/*            21 => machine.push_instruction(instruction),
            22 => machine.pop_instruction(instruction),
            23 => machine.call_instruction(instruction),
            24 => machine.ret_instruction(instruction),
            25 => machine.mov_instruction(instruction),
            26 => machine.inc_instruction(instruction), */
            _ => ()
        }
    }
}
pub fn emulate(input: String) {
                // 
    let mut machine = machine_state::State::new( 65536, 32);
    println!("Emulating '{}'.", input);
    let mut f = File::open(&input).unwrap();
    machine.load_memory_from_file(&mut f);
    {
        let mut opcode = 0;
        loop {
            let instruction = machine.get_current_instruction();
            opcode = decoder::opcode(&instruction);
            println!("Got a {:?} instruction, OPCODE: {}", decoder::get_type(&instruction), opcode);
            decoder::dispatch(&mut machine, &instruction);
            if opcode == 0 {
                break;
            }
            machine.increment_pc(1);
        } 
    }
}
