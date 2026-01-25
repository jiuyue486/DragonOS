use system_error::SystemError;

use crate::arch::interrupt::TrapFrame;
use crate::syscall::table::{FormattedSyscallParam, Syscall};
use alloc::vec::Vec;

pub struct Sys2333Handle;

impl Syscall for Sys2333Handle {
    fn num_args(&self) -> usize {
        0
    }

    fn handle(&self, _args: &[usize], _frame: &mut TrapFrame) -> Result<usize, SystemError> {
        log::info!("syscall 2333 called");
        Ok(6666)
    }

    fn entry_format(&self, _args: &[usize]) -> Vec<FormattedSyscallParam> {
        vec![]
    }
}

const SYS_2333: usize = 2333;
syscall_table_macros::declare_syscall!(SYS_2333, Sys2333Handle);
