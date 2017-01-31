#![feature(slice_patterns)]
extern crate nix;
extern crate libc;
use std::{env, ptr, mem, fmt};
use libc::{pid_t, c_void, waitpid};
use nix::sys::ptrace;
use nix::sys::ptrace::ptrace::{PTRACE_ATTACH, PTRACE_DETACH, PTRACE_GETREGS, PTRACE_POKETEXT,
                               PTRACE_SETREGS};

// #[cfg(target_arch = "x86_64")]
#[derive(Debug, Default)]
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


fn infect(pid: pid_t, buffer: &[u8]) -> Result<(), ()> {
    assert_eq!(ptrace::ptrace(PTRACE_ATTACH, pid, 0 as *mut c_void, 0 as *mut c_void).unwrap(),
               0);
    let mut regs: user_regs_struct;
    unsafe {
        waitpid(pid, 0 as *mut i32, 0);
        regs = mem::uninitialized();
    }

    assert_eq!(ptrace::ptrace(PTRACE_GETREGS,
                              pid,
                              &mut regs as *mut user_regs_struct as *mut c_void,
                              &mut regs as *mut user_regs_struct as *mut c_void)
                   .unwrap(),
               0);
    println!("{:?}", regs);


    
    regs.rsp -= 8; // decrement rsp
    println!("New rsp: {:x}", regs.rsp);

    assert_eq!(ptrace::ptrace(PTRACE_POKETEXT,
                              pid,
                              regs.rsp as *mut libc::c_void,
                              regs.rip as *mut libc::c_void)
                   .unwrap(),
               0);// poke rip -> rsp

    let ptr = regs.rsp - 1024; // inject rsp - 1024
    let beginning = ptr;

    println!("injecting into: {:x}", beginning);

    regs.rip = beginning + 2; // set rip as value of rsp - 1024
    println!("rip is at: {:x}", regs.rip);

    assert_eq!(ptrace::ptrace(PTRACE_SETREGS,
                              pid,
                              &mut regs as *mut user_regs_struct as *mut c_void,
                              &mut regs as *mut user_regs_struct as *mut c_void)
                   .unwrap(),
               0);
    for byte in buffer {
        // ptrace::ptrace(PTRACE_POKETEXT,
        //                pid,
        //                regs.rsp as *mut libc::c_void,
        //                regs.rip as *mut libc::c_void);
    }


    assert_eq!(ptrace::ptrace(PTRACE_DETACH, pid, 0 as *mut c_void, 0 as *mut c_void).unwrap(),
               0);
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match &args[1..] {
        &[ref pid, ref code] => {
            infect(pid.parse::<i32>().unwrap(), &[]);
        }
        _ => unimplemented!(),
    }
}
