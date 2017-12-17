use std::fs::File;
use std::io::prelude::*;

const EXPECTED_PROGRAM_SIZE: usize = 60_100;

mod synacor;

fn main() {
    let mut f = File::open("./challenge.bin").expect("File not found");
    let mut challenge: Vec<u8> = Vec::new();
    match f.read_to_end(&mut challenge) {
        Ok(bytes_read) => {
            if bytes_read != EXPECTED_PROGRAM_SIZE { panic!("Did not read the complete program, only read {}/{} bytes", bytes_read, EXPECTED_PROGRAM_SIZE); }

            let challenge_program: Vec<u16> = challenge.chunks(2).map(|c| {
                ((c[1] as u16) << 8) + c[0] as u16
            })
            .collect();

            let mut pooter = synacor::Vm::new();
            pooter.load_memory(challenge_program);
            pooter.run();
        },
        Err(e) => panic!("Unable to read challenge program: {}", e)
    }
}

#[cfg(test)]
mod test {
    use synacor;

    #[test]
    fn test_basic_program() {
        let test_program: Vec<u16> = vec![9, 32768, 32769, 4, 19, 32768];
        let mut vm = synacor::Vm::new();
        vm.load_memory(test_program);
        { vm.run(); }

        assert_eq!(&synacor::cpu::CpuState::Halted, vm.cpu().state());
        assert_eq!(0u16, vm.cpu().register_get(1));
        assert_eq!(4u16, vm.cpu().register_get(0));
        assert_eq!(6, vm.cpu().pc());
        assert_eq!(0, vm.cpu().sp());
    }
}