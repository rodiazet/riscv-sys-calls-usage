Tested only on ubuntu 24.04

### Prerequisites

Install all required packages from rust [riscv64-unknown-linux-gnu](https://doc.rust-lang.org/nightly/rustc/platform-support/riscv64gc-unknown-linux-gnu.html) spec including `qemu-system-riscv64` package.

### Run
```cargo --config config.toml run --target riscv64gc-unknown-linux-gnu --release```

### Output
```
692646 brk(NULL) = 0x0000555555883000
692646 mmap(NULL,8192,PROT_READ|PROT_WRITE,MAP_PRIVATE|MAP_ANONYMOUS,-1,0) = 0x00002aaaab2d3000
692646 faccessat(AT_FDCWD,"/etc/ld.so.preload",R_OK,AT_SYMLINK_NOFOLLOW|0x558818a0) = -1 errno=2 (No such file or directory)
692646 openat(AT_FDCWD,"/home/ethereum/riscv-sys-calls-usage/target/riscv64gc-unknown-linux-gnu/release/deps/libc.so.6",O_RDONLY|O_CLOEXEC) = -1 errno=2 (No such file or directory)
692646 newfstatat(AT_FDCWD,"/home/ethereum/riscv-sys-calls-usage/target/riscv64gc-unknown-linux-gnu/release/deps/",0x00002aaaab2a9dd0,0) = 0
692646 openat(AT_FDCWD,"/home/ethereum/riscv-sys-calls-usage/target/riscv64gc-unknown-linux-gnu/release/libc.so.6",O_RDONLY|O_CLOEXEC) = -1 errno=2 (No such file or directory)
692646 newfstatat(AT_FDCWD,"/home/ethereum/riscv-sys-calls-usage/target/riscv64gc-unknown-linux-gnu/release/",0x00002aaaab2a9dd0,0) = 0
692646 openat(AT_FDCWD,"/home/ethereum/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/riscv64gc-unknown-linux-gnu/lib/libc.so.6",O_RDONLY|O_CLOEXEC) = -1 errno=2 (No such file or directory)
692646 newfstatat(AT_FDCWD,"/home/ethereum/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/riscv64gc-unknown-linux-gnu/lib/",0x00002aaaab2a9dd0,0) = 0
692646 openat(AT_FDCWD,"/home/ethereum/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/libc.so.6",O_RDONLY|O_CLOEXEC) = -1 errno=2 (No such file or directory)
692646 newfstatat(AT_FDCWD,"/home/ethereum/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/",0x00002aaaab2a9dd0,0) = 0
692646 openat(AT_FDCWD,"/etc/ld.so.cache",O_RDONLY|O_CLOEXEC) = 3
692646 fstat(3,0x00002aaaab2a9dd0) = 0
692646 mmap(NULL,36351,PROT_READ,MAP_PRIVATE,3,0) = 0x00002aaaab2d5000
692646 close(3) = 0
692646 openat(AT_FDCWD,"/lib/riscv64-linux-gnu/libc.so.6",O_RDONLY|O_CLOEXEC) = -1 errno=2 (No such file or directory)
692646 newfstatat(AT_FDCWD,"/lib/riscv64-linux-gnu/",0x00002aaaab2a9dd0,0) = 0
692646 openat(AT_FDCWD,"/usr/lib/riscv64-linux-gnu/libc.so.6",O_RDONLY|O_CLOEXEC) = -1 errno=2 (No such file or directory)
692646 newfstatat(AT_FDCWD,"/usr/lib/riscv64-linux-gnu/",0x00002aaaab2a9dd0,0) = 0
692646 openat(AT_FDCWD,"/lib/libc.so.6",O_RDONLY|O_CLOEXEC) = 3
692646 read(3,0xab2a9f50,832) = 832
692646 fstat(3,0x00002aaaab2a9dd0) = 0
692646 mmap(NULL,1568176,PROT_EXEC|PROT_READ,MAP_PRIVATE|MAP_DENYWRITE,3,0) = 0x00002aaaab2de000
692646 mmap(0x00002aaaab44c000,20480,PROT_READ|PROT_WRITE,MAP_PRIVATE|MAP_DENYWRITE|MAP_FIXED,3,0x16e000) = 0x00002aaaab44c000
692646 mmap(0x00002aaaab451000,48560,PROT_READ|PROT_WRITE,MAP_PRIVATE|MAP_ANONYMOUS|MAP_FIXED,-1,0) = 0x00002aaaab451000
692646 close(3) = 0
692646 openat(AT_FDCWD,"/home/ethereum/riscv-sys-calls-usage/target/riscv64gc-unknown-linux-gnu/release/deps/libm.so.6",O_RDONLY|O_CLOEXEC) = -1 errno=2 (No such file or directory)
692646 openat(AT_FDCWD,"/home/ethereum/riscv-sys-calls-usage/target/riscv64gc-unknown-linux-gnu/release/libm.so.6",O_RDONLY|O_CLOEXEC) = -1 errno=2 (No such file or directory)
692646 openat(AT_FDCWD,"/home/ethereum/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/riscv64gc-unknown-linux-gnu/lib/libm.so.6",O_RDONLY|O_CLOEXEC) = -1 errno=2 (No such file or directory)
692646 openat(AT_FDCWD,"/home/ethereum/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/libm.so.6",O_RDONLY|O_CLOEXEC) = -1 errno=2 (No such file or directory)
692646 openat(AT_FDCWD,"/lib/riscv64-linux-gnu/libm.so.6",O_RDONLY|O_CLOEXEC) = -1 errno=2 (No such file or directory)
692646 openat(AT_FDCWD,"/usr/lib/riscv64-linux-gnu/libm.so.6",O_RDONLY|O_CLOEXEC) = -1 errno=2 (No such file or directory)
692646 openat(AT_FDCWD,"/lib/libm.so.6",O_RDONLY|O_CLOEXEC) = 3
692646 read(3,0xab2a9f10,832) = 832
692646 fstat(3,0x00002aaaab2a9d90) = 0
692646 mmap(NULL,512016,PROT_EXEC|PROT_READ,MAP_PRIVATE|MAP_DENYWRITE,3,0) = 0x00002aaaab45d000
692646 mmap(0x00002aaaab4d9000,8192,PROT_READ|PROT_WRITE,MAP_PRIVATE|MAP_DENYWRITE|MAP_FIXED,3,0x7c000) = 0x00002aaaab4d9000
692646 close(3) = 0
692646 openat(AT_FDCWD,"/home/ethereum/riscv-sys-calls-usage/target/riscv64gc-unknown-linux-gnu/release/deps/libgcc_s.so.1",O_RDONLY|O_CLOEXEC) = -1 errno=2 (No such file or directory)
692646 openat(AT_FDCWD,"/home/ethereum/riscv-sys-calls-usage/target/riscv64gc-unknown-linux-gnu/release/libgcc_s.so.1",O_RDONLY|O_CLOEXEC) = -1 errno=2 (No such file or directory)
692646 openat(AT_FDCWD,"/home/ethereum/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/riscv64gc-unknown-linux-gnu/lib/libgcc_s.so.1",O_RDONLY|O_CLOEXEC) = -1 errno=2 (No such file or directory)
692646 openat(AT_FDCWD,"/home/ethereum/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/libgcc_s.so.1",O_RDONLY|O_CLOEXEC) = -1 errno=2 (No such file or directory)
692646 openat(AT_FDCWD,"/lib/riscv64-linux-gnu/libgcc_s.so.1",O_RDONLY|O_CLOEXEC) = -1 errno=2 (No such file or directory)
692646 openat(AT_FDCWD,"/usr/lib/riscv64-linux-gnu/libgcc_s.so.1",O_RDONLY|O_CLOEXEC) = -1 errno=2 (No such file or directory)
692646 openat(AT_FDCWD,"/lib/libgcc_s.so.1",O_RDONLY|O_CLOEXEC) = 3
692646 read(3,0xab2a9ef0,832) = 832
692646 fstat(3,0x00002aaaab2a9d70) = 0
692646 mmap(NULL,119728,PROT_EXEC|PROT_READ,MAP_PRIVATE|MAP_DENYWRITE,3,0) = 0x00002aaaab4db000
692646 mmap(0x00002aaaab4f7000,8192,PROT_READ|PROT_WRITE,MAP_PRIVATE|MAP_DENYWRITE|MAP_FIXED,3,0x1b000) = 0x00002aaaab4f7000
692646 close(3) = 0
692646 mmap(NULL,8192,PROT_READ|PROT_WRITE,MAP_PRIVATE|MAP_ANONYMOUS,-1,0) = 0x00002aaaab4f9000
692646 set_tid_address(0x2aaaab4f90f0) = 692646
692646 set_robust_list(0x2aaaab4f9100,24) = -1 errno=38 (Function not implemented)
692646 mprotect(0x00002aaaab44c000,12288,PROT_READ) = 0
692646 mprotect(0x00002aaaab4f7000,4096,PROT_READ) = 0
692646 mprotect(0x00002aaaab4d9000,4096,PROT_READ) = 0
692646 mprotect(0x0000555555871000,69632,PROT_READ) = 0
692646 mprotect(0x00002aaaab2ce000,8192,PROT_READ) = 0
692646 prlimit64(0,RLIMIT_STACK,NULL,0x00002aaaab2aa9f8) = 0 ({rlim_cur=8388608,rlim_max=-1})
692646 munmap(0x00002aaaab2d5000,36351) = 0
692646 ppoll(0x2aaaab2aab70,3,0x2aaaab2aab38,(nil)) = 0
692646 rt_sigaction(SIGPIPE,0x00002aaaab2aa8b8,0x00002aaaab2aa948) = 0
692646 getrandom(0x2aaaab4555b8,8,1) = 8
692646 brk(NULL) = 0x0000555555883000
692646 brk(0x00005555558a4000) = 0x00005555558a4000
692646 openat(AT_FDCWD,"/proc/self/maps",O_RDONLY|O_CLOEXEC) = 3
692646 prlimit64(0,RLIMIT_STACK,NULL,0x00002aaaab2aaae8) = 0 ({rlim_cur=8388608,rlim_max=-1})
692646 fstat(3,0x00002aaaab2aa8f8) = 0
692646 read(3,0x55883500,4096) = 2448
692646 close(3) = 0
692646 sched_getaffinity(692646,32,0x555555883500) = 8
692646 rt_sigaction(SIGSEGV,NULL,0x00002aaaab2aaaa8) = 0
692646 sigaltstack((nil),0x2aaaab2aaaf8) = 0
692646 mmap(NULL,12288,PROT_READ|PROT_WRITE,MAP_PRIVATE|MAP_ANONYMOUS|MAP_STACK,-1,0) = 0x00002aaaab4fb000
692646 mprotect(0x00002aaaab4fb000,4096,PROT_NONE) = 0
692646 sigaltstack(0x2aaaab2aaaf8,(nil)) = 0
692646 rt_sigaction(SIGSEGV,0x00002aaaab2aaa18,NULL) = 0
692646 rt_sigaction(SIGBUS,NULL,0x00002aaaab2aaaa8) = 0
692646 rt_sigaction(SIGBUS,0x00002aaaab2aaa18,NULL) = 0
692646 brk(0x00005555558c5000) = 0x00005555558c5000
692646 brk(0x00005555558e6000) = 0x00005555558e6000
692646 brk(0x0000555555907000) = 0x0000555555907000
692646 brk(0x0000555555928000) = 0x0000555555928000
692646 brk(0x0000555555949000) = 0x0000555555949000
692646 brk(0x000055555596a000) = 0x000055555596a000
692646 brk(0x000055555598b000) = 0x000055555598b000
692646 brk(0x00005555559ac000) = 0x00005555559ac000
692646 brk(0x00005555559cd000) = 0x00005555559cd000
692646 brk(0x00005555559ee000) = 0x00005555559ee000
692646 brk(0x0000555555a0f000) = 0x0000555555a0f000
692646 brk(0x0000555555a30000) = 0x0000555555a30000
692646 brk(0x0000555555a51000) = 0x0000555555a51000
692646 brk(0x0000555555a72000) = 0x0000555555a72000
692646 brk(0x0000555555a93000) = 0x0000555555a93000
692646 brk(0x0000555555ab4000) = 0x0000555555ab4000
692646 brk(0x0000555555ad5000) = 0x0000555555ad5000
692646 brk(0x0000555555af6000) = 0x0000555555af6000
692646 brk(0x0000555555b17000) = 0x0000555555b17000
692646 brk(0x0000555555b38000) = 0x0000555555b38000
692646 brk(0x0000555555b59000) = 0x0000555555b59000
692646 mmap(NULL,1638400,PROT_READ|PROT_WRITE,MAP_PRIVATE|MAP_ANONYMOUS,-1,0) = 0x00002aaaab4fe000
692646 munmap(0x00002aaaab4fe000,1638400) = 0
692646 brk(0x0000555555b7a000) = 0x0000555555b7a000
692646 brk(0x0000555555b9b000) = 0x0000555555b9b000
692646 brk(0x0000555555bbc000) = 0x0000555555bbc000
692646 brk(0x0000555555bdd000) = 0x0000555555bdd000
692646 brk(0x0000555555bfe000) = 0x0000555555bfe000
692646 brk(0x0000555555c1f000) = 0x0000555555c1f000
692646 brk(0x0000555555c40000) = 0x0000555555c40000
692646 brk(0x0000555555c61000) = 0x0000555555c61000
692646 brk(0x0000555555c82000) = 0x0000555555c82000
692646 brk(0x0000555555ca3000) = 0x0000555555ca3000
692646 brk(0x0000555555cc4000) = 0x0000555555cc4000
692646 brk(0x0000555555ce5000) = 0x0000555555ce5000
692646 brk(0x0000555555d06000) = 0x0000555555d06000
692646 brk(0x0000555555d27000) = 0x0000555555d27000
692646 brk(0x0000555555d48000) = 0x0000555555d48000
692646 brk(0x0000555555d69000) = 0x0000555555d69000
692646 brk(0x0000555555d8a000) = 0x0000555555d8a000
692646 brk(0x0000555555dab000) = 0x0000555555dab000
692646 brk(0x0000555555dcc000) = 0x0000555555dcc000
692646 brk(0x0000555555ded000) = 0x0000555555ded000
692646 brk(0x0000555555e0e000) = 0x0000555555e0e000
692646 brk(0x0000555555faf000) = 0x0000555555faf000
692646 clock_gettime(CLOCK_REALTIME,0x00002aaaab2a7900) = 0 ({tv_sec = 1752676907,tv_nsec = 990843991})
692646 write(1,0x55e131f0,67)0x3516177fcb08e02f76a350ef56cb45a6034cb82e6c5936e21b1e7e6c309e56ca
 = 67
692646 sigaltstack(0x2aaaab2aaa98,(nil)) = 0
692646 munmap(0x00002aaaab4fb000,12288) = 0
692646 exit_group(0)
```

### Linux sys calls used:
```
brk
clock_gettime
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

