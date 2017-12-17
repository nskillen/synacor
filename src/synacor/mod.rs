pub mod bus;
pub mod cpu;
pub mod memory;
pub mod stack;
pub mod vm;

mod op_res;
use self::op_res::OpRes;

pub use self::vm::Vm;

pub type WORD = u16;