use std::env;
use std::error::Error;
use std::io;
use libc::fork;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <prog> <args>", args[0]);
        return Err(
            io::Error::new(
                io::ErrorKind::InvalidInput,
                "Not enough arguments"
            ).into());
    }

    unsafe {
        let fork_result: i32 = fork();
        if fork_result == 0 {
            return do_child(&args[1..]);
        } else {
            return do_trace(fork_result);
        }
    }
}

fn do_child(args: &[String]) -> Result<(), Box<dyn Error>> {
    let _prog = &args[0];
    let _prog_args: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
    
    unsafe {
        libc::ptrace(
            libc::PTRACE_TRACEME, 
            0,
            std::ptr::null_mut(),
            std::ptr::null_mut()
        );
        libc::kill(libc::getpid(), libc::SIGSTOP);
    }
    
    return Ok(())
}

fn do_trace(child_pid: i32) -> Result<(), Box<dyn Error>> {
    println!("Tracing child process with PID: {}", child_pid);
    return Ok(());
}
