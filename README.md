Tested only on ubuntu 24.04

### Prerequisites

Install all required packages from rust [riscv64-unknown-linux-gnu](https://doc.rust-lang.org/nightly/rustc/platform-support/riscv64gc-unknown-linux-gnu.html) spec including `qemu-system-riscv64` package.

### Run
```cargo --config config.toml run --target riscv64gc-unknown-linux-gnu --release```

### Output
```
579006 brk(NULL) = 0x00005555555a6000
579006 mmap(NULL,8192,PROT_READ|PROT_WRITE,MAP_PRIVATE|MAP_ANONYMOUS,-1,0) = 0x00002aaaab2d3000
579006 faccessat(AT_FDCWD,"/etc/ld.so.preload",R_OK,AT_EACCESS|AT_SYMLINK_NOFOLLOW|0x555a4830) = -1 errno=2 (No such file or directory)
579006 openat(AT_FDCWD,"/home/ethereum/riscv-sys-calls-usage/target/riscv64gc-unknown-linux-gnu/release/deps/libgcc_s.so.1",O_RDONLY|O_CLOEXEC) = -1 errno=2 (No such file or directory)
579006 newfstatat(AT_FDCWD,"/home/ethereum/riscv-sys-calls-usage/target/riscv64gc-unknown-linux-gnu/release/deps/",0x00002aaaab2a9dd0,0) = 0
579006 openat(AT_FDCWD,"/home/ethereum/riscv-sys-calls-usage/target/riscv64gc-unknown-linux-gnu/release/libgcc_s.so.1",O_RDONLY|O_CLOEXEC) = -1 errno=2 (No such file or directory)
579006 newfstatat(AT_FDCWD,"/home/ethereum/riscv-sys-calls-usage/target/riscv64gc-unknown-linux-gnu/release/",0x00002aaaab2a9dd0,0) = 0
579006 openat(AT_FDCWD,"/home/ethereum/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/riscv64gc-unknown-linux-gnu/lib/libgcc_s.so.1",O_RDONLY|O_CLOEXEC) = -1 errno=2 (No such file or directory)
579006 newfstatat(AT_FDCWD,"/home/ethereum/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/riscv64gc-unknown-linux-gnu/lib/",0x00002aaaab2a9dd0,0) = 0
579006 openat(AT_FDCWD,"/home/ethereum/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/libgcc_s.so.1",O_RDONLY|O_CLOEXEC) = -1 errno=2 (No such file or directory)
579006 newfstatat(AT_FDCWD,"/home/ethereum/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/",0x00002aaaab2a9dd0,0) = 0
579006 openat(AT_FDCWD,"/etc/ld.so.cache",O_RDONLY|O_CLOEXEC) = 3
579006 fstat(3,0x00002aaaab2a9dd0) = 0
579006 mmap(NULL,36351,PROT_READ,MAP_PRIVATE,3,0) = 0x00002aaaab2d5000
579006 close(3) = 0
579006 openat(AT_FDCWD,"/lib/riscv64-linux-gnu/libgcc_s.so.1",O_RDONLY|O_CLOEXEC) = -1 errno=2 (No such file or directory)
579006 newfstatat(AT_FDCWD,"/lib/riscv64-linux-gnu/",0x00002aaaab2a9dd0,0) = 0
579006 openat(AT_FDCWD,"/usr/lib/riscv64-linux-gnu/libgcc_s.so.1",O_RDONLY|O_CLOEXEC) = -1 errno=2 (No such file or directory)
579006 newfstatat(AT_FDCWD,"/usr/lib/riscv64-linux-gnu/",0x00002aaaab2a9dd0,0) = 0
579006 openat(AT_FDCWD,"/lib/libgcc_s.so.1",O_RDONLY|O_CLOEXEC) = 3
579006 read(3,0xab2a9f50,832) = 832
579006 fstat(3,0x00002aaaab2a9dd0) = 0
579006 mmap(NULL,119728,PROT_EXEC|PROT_READ,MAP_PRIVATE|MAP_DENYWRITE,3,0) = 0x00002aaaab2de000
579006 mmap(0x00002aaaab2fa000,8192,PROT_READ|PROT_WRITE,MAP_PRIVATE|MAP_DENYWRITE|MAP_FIXED,3,0x1b000) = 0x00002aaaab2fa000
579006 close(3) = 0
579006 openat(AT_FDCWD,"/home/ethereum/riscv-sys-calls-usage/target/riscv64gc-unknown-linux-gnu/release/deps/libc.so.6",O_RDONLY|O_CLOEXEC) = -1 errno=2 (No such file or directory)
579006 openat(AT_FDCWD,"/home/ethereum/riscv-sys-calls-usage/target/riscv64gc-unknown-linux-gnu/release/libc.so.6",O_RDONLY|O_CLOEXEC) = -1 errno=2 (No such file or directory)
579006 openat(AT_FDCWD,"/home/ethereum/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/riscv64gc-unknown-linux-gnu/lib/libc.so.6",O_RDONLY|O_CLOEXEC) = -1 errno=2 (No such file or directory)
579006 openat(AT_FDCWD,"/home/ethereum/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/libc.so.6",O_RDONLY|O_CLOEXEC) = -1 errno=2 (No such file or directory)
579006 openat(AT_FDCWD,"/lib/riscv64-linux-gnu/libc.so.6",O_RDONLY|O_CLOEXEC) = -1 errno=2 (No such file or directory)
579006 openat(AT_FDCWD,"/usr/lib/riscv64-linux-gnu/libc.so.6",O_RDONLY|O_CLOEXEC) = -1 errno=2 (No such file or directory)
579006 openat(AT_FDCWD,"/lib/libc.so.6",O_RDONLY|O_CLOEXEC) = 3
579006 read(3,0xab2a9f30,832) = 832
579006 fstat(3,0x00002aaaab2a9db0) = 0
579006 mmap(NULL,1568176,PROT_EXEC|PROT_READ,MAP_PRIVATE|MAP_DENYWRITE,3,0) = 0x00002aaaab2fc000
579006 mmap(0x00002aaaab46a000,20480,PROT_READ|PROT_WRITE,MAP_PRIVATE|MAP_DENYWRITE|MAP_FIXED,3,0x16e000) = 0x00002aaaab46a000
579006 mmap(0x00002aaaab46f000,48560,PROT_READ|PROT_WRITE,MAP_PRIVATE|MAP_ANONYMOUS|MAP_FIXED,-1,0) = 0x00002aaaab46f000
579006 close(3) = 0
579006 mmap(NULL,8192,PROT_READ|PROT_WRITE,MAP_PRIVATE|MAP_ANONYMOUS,-1,0) = 0x00002aaaab47b000
579006 set_tid_address(0x2aaaab47b0f0) = 579006
579006 set_robust_list(0x2aaaab47b100,24) = -1 errno=38 (Function not implemented)
579006 mprotect(0x00002aaaab46a000,12288,PROT_READ) = 0
579006 mprotect(0x00002aaaab2fa000,4096,PROT_READ) = 0
579006 mprotect(0x00005555555a2000,12288,PROT_READ) = 0
579006 mprotect(0x00002aaaab2ce000,8192,PROT_READ) = 0
579006 prlimit64(0,RLIMIT_STACK,NULL,0x00002aaaab2aa9f8) = 0 ({rlim_cur=8388608,rlim_max=-1})
579006 munmap(0x00002aaaab2d5000,36351) = 0
579006 ppoll(0x2aaaab2aab70,3,0x2aaaab2aab38,(nil)) = 0
579006 rt_sigaction(SIGPIPE,0x00002aaaab2aa8b8,0x00002aaaab2aa948) = 0
579006 getrandom(0x2aaaab4735b8,8,1) = 8
579006 brk(NULL) = 0x00005555555a6000
579006 brk(0x00005555555c7000) = 0x00005555555c7000
579006 openat(AT_FDCWD,"/proc/self/maps",O_RDONLY|O_CLOEXEC) = 3
579006 prlimit64(0,RLIMIT_STACK,NULL,0x00002aaaab2aaae8) = 0 ({rlim_cur=8388608,rlim_max=-1})
579006 fstat(3,0x00002aaaab2aa8f8) = 0
579006 read(3,0x555a6500,4096) = 2044
579006 close(3) = 0
579006 sched_getaffinity(579006,32,0x5555555a6500) = 8
579006 rt_sigaction(SIGSEGV,NULL,0x00002aaaab2aaaa8) = 0
579006 sigaltstack((nil),0x2aaaab2aaaf8) = 0
579006 mmap(NULL,12288,PROT_READ|PROT_WRITE,MAP_PRIVATE|MAP_ANONYMOUS|MAP_STACK,-1,0) = 0x00002aaaab47d000
579006 mprotect(0x00002aaaab47d000,4096,PROT_NONE) = 0
579006 sigaltstack(0x2aaaab2aaaf8,(nil)) = 0
579006 rt_sigaction(SIGSEGV,0x00002aaaab2aaa18,NULL) = 0
579006 rt_sigaction(SIGBUS,NULL,0x00002aaaab2aaaa8) = 0
579006 rt_sigaction(SIGBUS,0x00002aaaab2aaa18,NULL) = 0
579006 write(1,0x555a6600,11)5 + 7 = 12
 = 11
579006 sigaltstack(0x2aaaab2aaa98,(nil)) = 0
579006 munmap(0x00002aaaab47d000,12288) = 0
579006 exit_group(0)
```

### Linux sys calls used:
```
brk
close
exit_group
faccessat
fstat
getrandom
mmap
mprotect
munmap
newfstatat
openat
ppoll
prlimit64
read
rt_sigaction
sched_getaffinity
set_robust_list
set_tid_address
sigaltstack
write
```

