extern crate serde;
use serde::Deserialize;
extern crate docopt;
use docopt::Docopt;
extern crate rips;
use rips::assembler;
use rips::emulator;

static USAGE: &'static str ="
Usage: rips asm [-v] <source> <dest>
       rips emulate [-v] <file>
Options:
    -v, --verbose  Print debugging output. 
";
#[derive(Debug, Deserialize)]
struct Args {
    cmd_asm: bool,
    cmd_emulate: bool,
    flag_verbose: bool,
    arg_source: String,
    arg_dest: String,
    arg_file: String,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
                        .and_then(|d| d.deserialize()) 
                        .unwrap_or_else(|e| e.exit());
    if args.cmd_asm {
        assembler::assemble(args.arg_source, args.arg_dest);
    } else {
        emulator::emulate(args.arg_file);
    }
}
