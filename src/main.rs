#![feature(plugin)]
#![plugin(docopt_macros)]
extern crate rustc_serialize;
extern crate docopt;
extern crate rips;
use rips::assembler;
use rips::emulator;

docopt!(Args, "
Usage: rips asm [-v] <source> <dest>
       rips emulate [-v] <file>
Options:
    -v, --verbose  Print debugging output. 
");


fn main() {
    let args: Args = Args::docopt().decode()
                        .unwrap_or_else(|e| e.exit());
    if args.cmd_asm {
        assembler::assemble(args.arg_source, args.arg_dest);
    } else {
        emulator::emulate(args.arg_file);
    }
}
