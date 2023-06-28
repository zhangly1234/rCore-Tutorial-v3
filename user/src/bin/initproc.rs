#![no_std]
#![no_main]

extern crate user_lib;
extern crate alloc;
use user_lib::{println, getpid, print};
use alloc::vec;
use alloc::vec::Vec;
// use alloc::vec;
use user_lib::{exec, fork, wait, yield_};
use user_lib::{thread_create, get_time, pipe, write, read, close, waitpid, waittid,exit};
use user_lib::{open, OpenFlags};

pub fn thread_a() -> ! {
    for _ in 0..10 {
        print!("a");
    }
    exit(1)
}

pub fn thread_b() -> ! {
    for _ in 0..10 {
        print!("b");
    }
    exit(2)
}

pub fn thread_c() -> ! {
    for _ in 0..10 {
        print!("c");
    }
    exit(3)
}

#[no_mangle]
fn main() -> i32 {

    println!("aaaaaaaaaaaaaa");
    let a = getpid();
    println!("{}",a);
    let fd = open("a\0",  OpenFlags::CREATE | OpenFlags::WRONLY);
    if fd < 0 {
        panic!("Open test file failed!");
    }
    println!("fd={}",fd as usize);
    close(fd as usize);

    let v: Vec<i32> = Vec::new();
    let mut v = vec![1, 2, 3];
    v.push(4);
    let v1:Vec<&str> =Vec::new();
    let mut v1 = vec!["aaaaa","bbbbb","cccccc"];
    //v1.push("ddddddd");
    let v2:Vec<f64> = Vec::new();
    let mut v2=vec![111.111,222222.33333,444444.222222];
    v2.push(2222.7777);
    for i in &v {
        println!("{}",i);
    }

    // let a = thread_create(thread_a as usize, 0);
    // let b = thread_create(thread_b as usize, 0);
    // let c = thread_create(thread_c as usize, 0);

    // // for tid in v.iter() {
    // //     let exit_code = waittid(*tid as usize);
    // //     println!("thread#{} exited with code {}", tid, exit_code);
    // // }
    // let exit_code1=waittid(a as usize);
    // let exit_code2=waittid(b as usize);
    // let exit_code3=waittid(c as usize);
    // println!("main thread exited.");



    if fork() == 0 {
        exec("user_shell\0", &[core::ptr::null::<u8>()]);
    } else {
        loop {
            let mut exit_code: i32 = 0;
            let pid = wait(&mut exit_code);
            if pid == -1 {
                yield_();
                continue;
            }
            /*
            println!(
                "[initproc] Released a zombie process, pid={}, exit_code={}",
                pid,
                exit_code,
            );
            */
        }
    }
    println!("bbbbbbbbbb");
    println!("bbbbbbbbbb");
    println!("bbbbbbbbbb");
    0
}
