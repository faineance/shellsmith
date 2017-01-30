#![feature(slice_patterns)] 
extern crate nix;
extern crate libc;
use std::env;
use libc::{pid_t,c_void};
use nix::sys::ptrace;
use nix::sys::ptrace::ptrace::{PTRACE_ATTACH,PTRACE_DETACH,PTRACE_GETREGS};

#[cfg(target_arch = "x86_64")]
#[derive(Debug,Default)]
#[repr(C)]
struct user_regs_struct {
    r15: u64,
    r14: u64,
    r13: u64,
    r12: u64,
    rbp: u64,
    rbx: u64,
    r11: u64,
    r10: u64,
    r9: u64,
    r8: u64,
    rax: u64,
    rcx: u64,
    rdx: u64,
    rsi: u64,
    rdi: u64,
    orig_rax: u64,
    rip: u64,
    cs: u64,
    eflags: u64,
    rsp: u64,
    ss: u64,
    fs_base: u64,
    gs_base: u64,
    ds: u64,
    es: u64,
    fs: u64,
    gs: u64,
}


fn infect(pid: pid_t) -> Result<(),()> {
    ptrace::ptrace(PTRACE_ATTACH,pid,0 as *mut c_void ,0 as *mut c_void);
    let mut regs: user_regs_struct = user_regs_struct::default();
    ptrace::ptrace(PTRACE_GETREGS,pid,&mut regs as *mut user_regs_struct as *mut c_void,&mut regs as *mut user_regs_struct as *mut c_void);
    println!("{:?}",regs);
    ptrace::ptrace(PTRACE_DETACH,pid,0 as *mut c_void ,0 as *mut c_void);
    unimplemented!()
}
fn main() {
     let args: Vec<String> = env::args().collect();

     match &args[1..] {
         &[ref pid, ref code] => {
            infect(2138);
             },
         _ => unimplemented!(),
     }
}
