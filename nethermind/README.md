# Netherming client syscall usage.

This document lists all syscall used by stateless program execution defined in [this repo](https://github.com/rodiazet/nethermind/blob/feature/stateless_execution2-syscalls/tools/StatelessExecution/Program.cs). It is impossible to build it to any riscv target at the moment so we analyse `strace` of the binary built for `x86-linux-gnu` target.

1. Gargabe collector is **on**. 
2. Configuration `release`.
3. `dotnet v9.0.108`
4. Program arguments [`8B000_witness.json`](./8B000_witness.json) [`8B000_block.json`](./8B000_block.json)
5. Built to `x86-linux-gnu` target.

### Syscalls used in full trace

[The full trace](./trace-full.txt) ilustrates syscall used by the whole program from start to exit. It uses 67 different syscalls.

#### Number of accurances:
|syscall|NoA|
|-------|---|
| Syscall | NoA |
| ------- | --- |
| [clock_gettime](https://man7.org/linux/man-pages/man2/clock_gettime.2.html) | 7609 |
| [mprotect](https://man7.org/linux/man-pages/man2/mprotect.2.html) | 2313 |
| [read](https://man7.org/linux/man-pages/man2/read.2.html) | 1662 |
| [lseek](https://man7.org/linux/man-pages/man2/lseek.2.html) | 1525 |
| [mmap](https://man7.org/linux/man-pages/man2/mmap.2.html) | 1012 |
| [readlink](https://man7.org/linux/man-pages/man2/readlink.2.html) | 650 |
| [munmap](https://man7.org/linux/man-pages/man2/munmap.2.html) | 565 |
| [rt_sigprocmask](https://man7.org/linux/man-pages/man2/rt_sigprocmask.2.html) | 383 |
| [fstat](https://man7.org/linux/man-pages/man2/fstat.2.html) | 194 |
| [futex](https://man7.org/linux/man-pages/man2/futex.2.html) | 193 |
| [pread64](https://man7.org/linux/man-pages/man2/pread64.2.html) | 181 |
| [openat](https://man7.org/linux/man-pages/man2/openat.2.html) | 173 |
| [fcntl](https://man7.org/linux/man-pages/man2/fcntl.2.html) | 130 |
| [brk](https://man7.org/linux/man-pages/man2/brk.2.html) | 125 |
| [close](https://man7.org/linux/man-pages/man2/close.2.html) | 106 |
| [madvise](https://man7.org/linux/man-pages/man2/madvise.2.html) | 98 |
| [newfstatat](https://man7.org/linux/man-pages/man2/newfstatat.2.html) | 78 |
| [sched_yield](https://man7.org/linux/man-pages/man2/sched_yield.2.html) | 38 |
| [rt_sigaction](https://man7.org/linux/man-pages/man2/rt_sigaction.2.html) | 26 |
| [membarrier](https://man7.org/linux/man-pages/man2/membarrier.2.html) | 25 |
| [write](https://man7.org/linux/man-pages/man2/write.2.html) | 22 |
| [access](https://man7.org/linux/man-pages/man2/access.2.html) | 18 |
| [getpid](https://man7.org/linux/man-pages/man2/getpid.2.html) | 15 |
| [prlimit64](https://man7.org/linux/man-pages/man2/prlimit64.2.html) | 14 |
| [clone3](https://man7.org/linux/man-pages/man2/clone3.2.html) | 12 |
| [getuid](https://man7.org/linux/man-pages/man2/getuid.2.html) | 12 |
| [geteuid](https://man7.org/linux/man-pages/man2/geteuid.2.html) | 11 |
| [getegid](https://man7.org/linux/man-pages/man2/getegid.2.html) | 10 |
| [getgid](https://man7.org/linux/man-pages/man2/getgid.2.html) | 10 |
| [pipe2](https://man7.org/linux/man-pages/man2/pipe2.2.html) | 10 |
| [getdents64](https://man7.org/linux/man-pages/man2/getdents64.2.html) | 8 |
| [mincore](https://man7.org/linux/man-pages/man2/mincore.2.html) | 5 |
| [sched_getaffinity](https://man7.org/linux/man-pages/man2/sched_getaffinity.2.html) | 5 |
| [unlink](https://man7.org/linux/man-pages/man2/unlink.2.html) | 5 |
| [flock](https://man7.org/linux/man-pages/man2/flock.2.html) | 4 |
| [getcwd](https://man7.org/linux/man-pages/man2/getcwd.2.html) | 4 |
| [getrandom](https://man7.org/linux/man-pages/man2/getrandom.2.html) | 4 |
| [ioctl](https://man7.org/linux/man-pages/man2/ioctl.2.html) | 4 |
| [gettid](https://man7.org/linux/man-pages/man2/gettid.2.html) | 3 |
| [sched_get_priority_max](https://man7.org/linux/man-pages/man2/sched_get_priority_max.2.html) | 3 |
| [sched_get_priority_min](https://man7.org/linux/man-pages/man2/sched_get_priority_min.2.html) | 3 |
| [sched_getparam](https://man7.org/linux/man-pages/man2/sched_getparam.2.html) | 3 |
| [sched_getscheduler](https://man7.org/linux/man-pages/man2/sched_getscheduler.2.html) | 3 |
| [sched_setscheduler](https://man7.org/linux/man-pages/man2/sched_setscheduler.2.html) | 3 |
| [statfs](https://man7.org/linux/man-pages/man2/statfs.2.html) | 3 |
| [getsid](https://man7.org/linux/man-pages/man2/getsid.2.html) | 2 |
| [mknodat](https://man7.org/linux/man-pages/man2/mknodat.2.html) | 2 |
| [prctl](https://man7.org/linux/man-pages/man2/prctl.2.html) | 2 |
| [sigaltstack](https://man7.org/linux/man-pages/man2/sigaltstack.2.html) | 2 |
| [sysinfo](https://man7.org/linux/man-pages/man2/sysinfo.2.html) | 2 |
| [vfork](https://man7.org/linux/man-pages/man2/vfork.2.html) | 2 |
| [arch_prctl](https://man7.org/linux/man-pages/man2/arch_prctl.2.html) | 1 |
| [bind](https://man7.org/linux/man-pages/man2/bind.2.html) | 1 |
| [epoll_create1](https://man7.org/linux/man-pages/man2/epoll_create1.2.html) | 1 |
| [execve](https://man7.org/linux/man-pages/man2/execve.2.html) | 1 |
| [exit_group](https://man7.org/linux/man-pages/man2/exit_group.2.html) | 1 |
| [fchmod](https://man7.org/linux/man-pages/man2/fchmod.2.html) | 1 |
| [ftruncate](https://man7.org/linux/man-pages/man2/ftruncate.2.html) | 1 |
| [get_mempolicy](https://man7.org/linux/man-pages/man2/get_mempolicy.2.html) | 1 |
| [listen](https://man7.org/linux/man-pages/man2/listen.2.html) | 1 |
| [memfd_create](https://man7.org/linux/man-pages/man2/memfd_create.2.html) | 1 |
| [readlinkat](https://man7.org/linux/man-pages/man2/readlinkat.2.html) | 1 |
| [rseq](https://man7.org/linux/man-pages/man2/rseq.2.html) | 1 |
| [rt_sigreturn](https://man7.org/linux/man-pages/man2/rt_sigreturn.2.html) | 1 |
| [set_robust_list](https://man7.org/linux/man-pages/man2/set_robust_list.2.html) | 1 |
| [set_tid_address](https://man7.org/linux/man-pages/man2/set_tid_address.2.html) | 1 |
| [socket](https://man7.org/linux/man-pages/man2/socket.2.html) | 1 |

### Syscalls used by processsing

List of syscalls used by the program [fragment](https://github.com/rodiazet/nethermind/blob/feature/stateless_execution2-syscalls/tools/StatelessExecution/Program.cs#L66-L77) can be found below. It uses only 32 different syscalls.

#### Num of accurances:
| syscall | NoA |
| --- | --- |
| [clock_gettime](https://man7.org/linux/man-pages/man2/clock_gettime.2.html) | 5623 |
| [mprotect](https://man7.org/linux/man-pages/man2/mprotect.2.html) | 996 |
| [mmap](https://man7.org/linux/man-pages/man2/mmap.2.html) | 518 |
| [munmap](https://man7.org/linux/man-pages/man2/munmap.2.html) | 385 |
| [readlink](https://man7.org/linux/man-pages/man2/readlink.2.html) | 161 |
| [futex](https://man7.org/linux/man-pages/man2/futex.2.html) | 148 |
| [rt_sigprocmask](https://man7.org/linux/man-pages/man2/rt_sigprocmask.2.html) | 106 |
| [fstat](https://man7.org/linux/man-pages/man2/fstat.2.html) | 50 |
| [fcntl](https://man7.org/linux/man-pages/man2/fcntl.2.html) | 46 |
| [pread64](https://man7.org/linux/man-pages/man2/pread64.2.html) | 44 |
| [sched_yield](https://man7.org/linux/man-pages/man2/sched_yield.2.html) | 38 |
| [newfstatat](https://man7.org/linux/man-pages/man2/newfstatat.2.html) | 35 |
| [openat](https://man7.org/linux/man-pages/man2/openat.2.html) | 24 |
| [close](https://man7.org/linux/man-pages/man2/close.2.html) | 12 |
| [write](https://man7.org/linux/man-pages/man2/write.2.html) | 10 |
| [read](https://man7.org/linux/man-pages/man2/read.2.html) | 9 |
| [pipe2](https://man7.org/linux/man-pages/man2/pipe2.2.html) | 7 |
| [brk](https://man7.org/linux/man-pages/man2/brk.2.html) | 6 |
| [mincore](https://man7.org/linux/man-pages/man2/mincore.2.html) | 5 |
| [getpid](https://man7.org/linux/man-pages/man2/getpid.2.html) | 4 |
| [madvise](https://man7.org/linux/man-pages/man2/madvise.2.html) | 4 |
| [clone3](https://man7.org/linux/man-pages/man2/clone3.2.html) | 3 |
| [sched_get_priority_max](https://man7.org/linux/man-pages/man2/sched_get_priority_max.2.html) | 3 |
| [sched_get_priority_min](https://man7.org/linux/man-pages/man2/sched_get_priority_min.2.html) | 3 |
| [sched_getparam](https://man7.org/linux/man-pages/man2/sched_getparam.2.html) | 3 |
| [sched_getscheduler](https://man7.org/linux/man-pages/man2/sched_getscheduler.2.html) | 3 |
| [sched_setscheduler](https://man7.org/linux/man-pages/man2/sched_setscheduler.2.html) | 3 |
| [access](https://man7.org/linux/man-pages/man2/access.2.html) | 2 |
| [vfork](https://man7.org/linux/man-pages/man2/vfork.2.html) | 2 |
| [epoll_create1](https://man7.org/linux/man-pages/man2/epoll_create1.2.html) | 1 |
| [getcwd](https://man7.org/linux/man-pages/man2/getcwd.2.html) | 1 |
| [rt_sigreturn](https://man7.org/linux/man-pages/man2/rt_sigreturn.2.html) | 1 |

### Sumary
Most of the syscall are related to GC operations (`clock_gettime`, `mmpa`, `munmap` etc.), multhreding (`futex`, `rt_sigprocmask` etc.), file system operations(`readlink`, `openet`, `fstat`, `fcntl` etc.) and resources managing(`sched_*`, `madvice`, etc).

Unfortunatelly it's not easy to turn off GC in dotnet. We add no-gc region using [TryStartNoGCRegion](https://learn.microsoft.com/en-us/dotnet/api/system.gc.trystartnogcregion?view=net-9.0) method but it reduced GC related syscalls only by a couple of %. GC should be off for the whole stateless validation function as it increases sugnificantly proving cost.

There is not any official support for compilation C# to riscv. It's implemented parcialy in dotnet SDK but it's not officialy supported. RISC-V target gh issue can be found [here](https://github.com/dotnet/runtime/issues/36748).