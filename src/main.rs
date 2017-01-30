#![feature(slice_patterns)] 
extern crate nix;
extern crate libc;
use std::env;
use libc::{pid_t,c_void};
use nix::sys::ptrace;
use nix::sys::ptrace::ptrace::{PTRACE_ATTACH,PTRACE_DETACH};
fn infect(pid: pid_t) -> Result<(),()> {
    ptrace::ptrace(PTRACE_ATTACH,pid,0 as *mut c_void ,0 as *mut c_void);

    // todo stuff

    ptrace::ptrace(PTRACE_DETACH,pid,0 as *mut c_void ,0 as *mut c_void);
    unimplemented!()
}
fn main() {
     let args: Vec<String> = env::args().collect();

     match &args[..] {
         &[ref pid, ref code] => {

             },
         _ => unimplemented!(),
     }
}
