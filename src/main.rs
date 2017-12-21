#![feature(match_default_bindings)]
#![feature(try_from)]
#![feature(try_trait)]

extern crate getopts;
#[macro_use] extern crate lazy_static;
extern crate regex;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use getopts::Options;

//const EXPECTED_PROGRAM_SIZE: usize = 60_100;

mod assembler;
mod debugger;
mod synacor;

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts: Options = Options::new();
    opts.optopt("r", "run", "run the selected binary file", "BINARY");
    opts.optopt("a", "assemble", "assemble the selected source file into a binary file", "SOURCE");
    opts.optopt("d", "disassemble", "disassemble the selected binary file into a source file", "BINARY");
    opts.optflag("g", "debugger", "attach debugger to program run");
    opts.optflag("h", "help", "prints this help menu");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(e) => panic!(e.to_string()),
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    } else if matches.opt_present("r") {
        match matches.opt_str("r") {
            Some(filename) => {
                let mut f = File::open(filename).expect("File not found");
                let mut challenge: Vec<u8> = Vec::new();
                match f.read_to_end(&mut challenge) {
                    Ok(_) => {
                        //if bytes_read != EXPECTED_PROGRAM_SIZE { panic!("Did not read the complete program, only read {}/{} bytes", bytes_read, EXPECTED_PROGRAM_SIZE); }

                        let program: Vec<synacor::WORD> = challenge.chunks(2).map(|c| ((c[1] as synacor::WORD) << 8) + c[0] as synacor::WORD).collect();
                        let mut pooter = synacor::Vm::new();

                        if matches.opt_present("g") {
                            // TODO figure out how to attach debugger, and intercept keypresses
                            println!("Debugger not yet supported");
                            pooter.set_debug(true);
                        }

                        pooter.load_memory(program);
                        pooter.run();
                    },
                    Err(e) => panic!("Unable to read challenge program: {}", e)
                }
            },
            None => println!("You mut supply a filename to run"),
        }
    } else if matches.opt_present("a") {
        match matches.opt_str("a") {
            Some(filename) => {
                // TODO run assembler on file and produce a binary output
                let p = Path::new(&filename);
                let mut f = File::open(p).expect("file not found");
                let mut contents: String = String::new();
                f.read_to_string(&mut contents);
                assembler::assemble(contents, p.with_extension("bin").to_str().unwrap());
            },
            None => println!("You must supply a filename to assemble into a binary"),
        }
    } else if matches.opt_present("d") {
        match matches.opt_str("d") {
            Some(filename) => {
                // TODO run disassembler on file and produce an assembly output
                unimplemented!();
            },
            None => println!("You must supply a filename to disassemble into an assembly file"),
        }
    } else {
        print_usage(&program, opts);
    }
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

#[cfg(test)]
mod test {
    use synacor;

    #[test]
    fn test_basic_program() {
        let test_program: Vec<synacor::WORD> = vec![9, 32768, 32769, 4, 19, 32768];
        let mut vm = synacor::Vm::new();
        vm.load_memory(test_program);
        { vm.run(); }

        assert_eq!(&synacor::cpu::CpuState::Halted, vm.cpu().state());
        assert_eq!(0u16, vm.cpu().register_get(1));
        assert_eq!(4u16, vm.cpu().register_get(0));
        assert_eq!(6, vm.cpu().pc());
    }
}