# Strc - A Minimal Strace

Strace is a neat program. It can tell you everything that your program is doing by clocking all the system calls that the program makes. This is a minimal implementation of `strace`, written in rust. `strc` will run the program, but it will write all the system calls to an output file and leave `stdin`, `stdout`, and `stderr` unchanged. 