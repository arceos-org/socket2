//! x86_64-specific definitions for 64-bit linux-like values

pub type c_schar = i8;
pub type c_uchar = u8;
pub type c_short = i16;
pub type c_ushort = u16;
pub type c_int = i32;
pub type c_uint = u32;
pub type c_float = f32;
pub type c_double = f64;
pub type c_longlong = i64;
pub type c_ulonglong = u64;
pub type intmax_t = i64;
pub type uintmax_t = u64;

pub type size_t = usize;
pub type ptrdiff_t = isize;
pub type intptr_t = isize;
pub type uintptr_t = usize;
pub type ssize_t = isize;

pub type pid_t = i32;
pub type in_addr_t = u32;
pub type in_port_t = u16;
pub type sighandler_t = size_t;
pub type cc_t = c_uchar;

pub type c_char = i8;
pub type wchar_t = i32;
pub type nlink_t = u64;
pub type blksize_t = i64;
pub type greg_t = i64;
pub type suseconds_t = i64;
pub type __u64 = c_ulonglong;
pub type __s64 = c_longlong;

pub type sa_family_t = u16;
pub type socklen_t = u32;

pub const POSIX_FADV_DONTNEED: c_int = 4;
pub const POSIX_FADV_NOREUSE: c_int = 5;

pub const VEOF: usize = 4;
pub const RTLD_DEEPBIND: c_int = 0x8;
pub const RTLD_GLOBAL: c_int = 0x100;
pub const RTLD_NOLOAD: c_int = 0x4;

pub const O_APPEND: c_int = 1024;
pub const O_CREAT: c_int = 64;
pub const O_EXCL: c_int = 128;
pub const O_NOCTTY: c_int = 256;
pub const O_NONBLOCK: c_int = 2048;
pub const O_SYNC: c_int = 1052672;
pub const O_RSYNC: c_int = 1052672;
pub const O_DSYNC: c_int = 4096;
pub const O_FSYNC: c_int = 0x101000;
pub const O_NOATIME: c_int = 0o1000000;
pub const O_PATH: c_int = 0o10000000;
pub const O_TMPFILE: c_int = 0o20000000 | O_DIRECTORY;

pub const MADV_SOFT_OFFLINE: c_int = 101;
pub const MAP_GROWSDOWN: c_int = 0x0100;

pub const EDEADLK: c_int = 35;
pub const ENAMETOOLONG: c_int = 36;
pub const ENOLCK: c_int = 37;
pub const ENOSYS: c_int = 38;
pub const ENOTEMPTY: c_int = 39;
pub const ELOOP: c_int = 40;
pub const ENOMSG: c_int = 42;
pub const EIDRM: c_int = 43;
pub const ECHRNG: c_int = 44;
pub const EL2NSYNC: c_int = 45;
pub const EL3HLT: c_int = 46;
pub const EL3RST: c_int = 47;
pub const ELNRNG: c_int = 48;
pub const EUNATCH: c_int = 49;
pub const ENOCSI: c_int = 50;
pub const EL2HLT: c_int = 51;
pub const EBADE: c_int = 52;
pub const EBADR: c_int = 53;
pub const EXFULL: c_int = 54;
pub const ENOANO: c_int = 55;
pub const EBADRQC: c_int = 56;
pub const EBADSLT: c_int = 57;
pub const EMULTIHOP: c_int = 72;
pub const EOVERFLOW: c_int = 75;
pub const ENOTUNIQ: c_int = 76;
pub const EBADFD: c_int = 77;
pub const EBADMSG: c_int = 74;
pub const EREMCHG: c_int = 78;
pub const ELIBACC: c_int = 79;
pub const ELIBBAD: c_int = 80;
pub const ELIBSCN: c_int = 81;
pub const ELIBMAX: c_int = 82;
pub const ELIBEXEC: c_int = 83;
pub const EILSEQ: c_int = 84;
pub const ERESTART: c_int = 85;
pub const ESTRPIPE: c_int = 86;
pub const EUSERS: c_int = 87;
pub const ENOTSOCK: c_int = 88;
pub const EDESTADDRREQ: c_int = 89;
pub const EMSGSIZE: c_int = 90;
pub const EPROTOTYPE: c_int = 91;
pub const ENOPROTOOPT: c_int = 92;
pub const EPROTONOSUPPORT: c_int = 93;
pub const ESOCKTNOSUPPORT: c_int = 94;
pub const EOPNOTSUPP: c_int = 95;
pub const EPFNOSUPPORT: c_int = 96;
pub const EAFNOSUPPORT: c_int = 97;
pub const EADDRINUSE: c_int = 98;
pub const EADDRNOTAVAIL: c_int = 99;
pub const ENETDOWN: c_int = 100;
pub const ENETUNREACH: c_int = 101;
pub const ENETRESET: c_int = 102;
pub const ECONNABORTED: c_int = 103;
pub const ECONNRESET: c_int = 104;
pub const ENOBUFS: c_int = 105;
pub const EISCONN: c_int = 106;
pub const ENOTCONN: c_int = 107;
pub const ESHUTDOWN: c_int = 108;
pub const ETOOMANYREFS: c_int = 109;
pub const ETIMEDOUT: c_int = 110;
pub const ECONNREFUSED: c_int = 111;
pub const EHOSTDOWN: c_int = 112;
pub const EHOSTUNREACH: c_int = 113;
pub const EALREADY: c_int = 114;
pub const EINPROGRESS: c_int = 115;
pub const ESTALE: c_int = 116;
pub const EDQUOT: c_int = 122;
pub const ENOMEDIUM: c_int = 123;
pub const EMEDIUMTYPE: c_int = 124;
pub const ECANCELED: c_int = 125;
pub const ENOKEY: c_int = 126;
pub const EKEYEXPIRED: c_int = 127;
pub const EKEYREVOKED: c_int = 128;
pub const EKEYREJECTED: c_int = 129;
pub const EOWNERDEAD: c_int = 130;
pub const ENOTRECOVERABLE: c_int = 131;
pub const EHWPOISON: c_int = 133;
pub const ERFKILL: c_int = 132;

pub const SOCK_STREAM: c_int = 1;
pub const SOCK_DGRAM: c_int = 2;

pub const SA_ONSTACK: c_int = 0x08000000;
pub const SA_SIGINFO: c_int = 0x00000004;
pub const SA_NOCLDWAIT: c_int = 0x00000002;

pub const SIGTTIN: c_int = 21;
pub const SIGTTOU: c_int = 22;
pub const SIGXCPU: c_int = 24;
pub const SIGXFSZ: c_int = 25;
pub const SIGVTALRM: c_int = 26;
pub const SIGPROF: c_int = 27;
pub const SIGWINCH: c_int = 28;
pub const SIGCHLD: c_int = 17;
pub const SIGBUS: c_int = 7;
pub const SIGUSR1: c_int = 10;
pub const SIGUSR2: c_int = 12;
pub const SIGCONT: c_int = 18;
pub const SIGSTOP: c_int = 19;
pub const SIGTSTP: c_int = 20;
pub const SIGURG: c_int = 23;
pub const SIGIO: c_int = 29;
pub const SIGSYS: c_int = 31;
pub const SIGSTKFLT: c_int = 16;
#[deprecated(since = "0.2.55", note = "Use SIGSYS instead")]
pub const SIGUNUSED: c_int = 31;
pub const SIGPOLL: c_int = 29;
pub const SIGPWR: c_int = 30;
pub const SIG_SETMASK: c_int = 2;
pub const SIG_BLOCK: c_int = 0x000000;
pub const SIG_UNBLOCK: c_int = 0x01;

pub const POLLWRNORM: c_short = 0x100;
pub const POLLWRBAND: c_short = 0x200;

pub const O_ASYNC: c_int = 0x2000;
pub const O_NDELAY: c_int = 0x800;

pub const PTRACE_DETACH: c_uint = 17;
pub const PTRACE_GET_RSEQ_CONFIGURATION: c_uint = 0x420f;

pub const EFD_NONBLOCK: c_int = 0x800;

pub const F_GETLK: c_int = 5;
pub const F_GETOWN: c_int = 9;
pub const F_SETOWN: c_int = 8;
pub const F_SETLK: c_int = 6;
pub const F_SETLKW: c_int = 7;
pub const F_OFD_GETLK: c_int = 36;
pub const F_OFD_SETLK: c_int = 37;
pub const F_OFD_SETLKW: c_int = 38;

pub const F_RDLCK: c_int = 0;
pub const F_WRLCK: c_int = 1;
pub const F_UNLCK: c_int = 2;

pub const SFD_NONBLOCK: c_int = 0x0800;

pub const TCSANOW: c_int = 0;
pub const TCSADRAIN: c_int = 1;
pub const TCSAFLUSH: c_int = 2;

pub const SFD_CLOEXEC: c_int = 0x080000;

pub const NCCS: usize = 32;

pub const O_TRUNC: c_int = 512;

pub const O_CLOEXEC: c_int = 0x80000;

pub const EBFONT: c_int = 59;
pub const ENOSTR: c_int = 60;
pub const ENODATA: c_int = 61;
pub const ETIME: c_int = 62;
pub const ENOSR: c_int = 63;
pub const ENONET: c_int = 64;
pub const ENOPKG: c_int = 65;
pub const EREMOTE: c_int = 66;
pub const ENOLINK: c_int = 67;
pub const EADV: c_int = 68;
pub const ESRMNT: c_int = 69;
pub const ECOMM: c_int = 70;
pub const EPROTO: c_int = 71;
pub const EDOTDOT: c_int = 73;

pub const SA_NODEFER: c_int = 0x40000000;
//pub const SA_RESETHAND: c_int = 0x80000000;
pub const SA_RESTART: c_int = 0x10000000;
pub const SA_NOCLDSTOP: c_int = 0x00000001;

pub const EPOLL_CLOEXEC: c_int = 0x80000;

pub const EFD_CLOEXEC: c_int = 0x80000;

pub const __SIZEOF_PTHREAD_CONDATTR_T: usize = 4;
pub const __SIZEOF_PTHREAD_MUTEXATTR_T: usize = 4;
pub const __SIZEOF_PTHREAD_BARRIERATTR_T: usize = 4;

pub const O_DIRECT: c_int = 0x4000;
pub const O_DIRECTORY: c_int = 0x10000;
pub const O_NOFOLLOW: c_int = 0x20000;

pub const MAP_HUGETLB: c_int = 0x040000;
pub const MAP_LOCKED: c_int = 0x02000;
pub const MAP_NORESERVE: c_int = 0x04000;
pub const MAP_32BIT: c_int = 0x0040;
pub const MAP_ANON: c_int = 0x0020;
pub const MAP_ANONYMOUS: c_int = 0x0020;
pub const MAP_DENYWRITE: c_int = 0x0800;
pub const MAP_EXECUTABLE: c_int = 0x01000;
pub const MAP_POPULATE: c_int = 0x08000;
pub const MAP_NONBLOCK: c_int = 0x010000;
pub const MAP_STACK: c_int = 0x020000;
pub const MAP_SYNC: c_int = 0x080000;

pub const EDEADLOCK: c_int = 35;
pub const EUCLEAN: c_int = 117;
pub const ENOTNAM: c_int = 118;
pub const ENAVAIL: c_int = 119;
pub const EISNAM: c_int = 120;
pub const EREMOTEIO: c_int = 121;

pub const PTRACE_GETFPREGS: c_uint = 14;
pub const PTRACE_SETFPREGS: c_uint = 15;
pub const PTRACE_GETFPXREGS: c_uint = 18;
pub const PTRACE_SETFPXREGS: c_uint = 19;
pub const PTRACE_GETREGS: c_uint = 12;
pub const PTRACE_SETREGS: c_uint = 13;
pub const PTRACE_PEEKSIGINFO_SHARED: c_uint = 1;
pub const PTRACE_SYSEMU: c_uint = 31;
pub const PTRACE_SYSEMU_SINGLESTEP: c_uint = 32;

pub const PR_GET_SPECULATION_CTRL: c_int = 52;
pub const PR_SET_SPECULATION_CTRL: c_int = 53;
pub const PR_SPEC_NOT_AFFECTED: c_uint = 0;
pub const PR_SPEC_PRCTL: c_uint = 1 << 0;
pub const PR_SPEC_ENABLE: c_uint = 1 << 1;
pub const PR_SPEC_DISABLE: c_uint = 1 << 2;
pub const PR_SPEC_FORCE_DISABLE: c_uint = 1 << 3;
pub const PR_SPEC_DISABLE_NOEXEC: c_uint = 1 << 4;
pub const PR_SPEC_STORE_BYPASS: c_int = 0;
pub const PR_SPEC_INDIRECT_BRANCH: c_int = 1;
// FIXME: perharps for later
//pub const PR_SPEC_L1D_FLUSH: c_int = 2;

pub const MCL_CURRENT: c_int = 0x0001;
pub const MCL_FUTURE: c_int = 0x0002;

// offsets in user_regs_structs, from sys/reg.h
pub const R15: c_int = 0;
pub const R14: c_int = 1;
pub const R13: c_int = 2;
pub const R12: c_int = 3;
pub const RBP: c_int = 4;
pub const RBX: c_int = 5;
pub const R11: c_int = 6;
pub const R10: c_int = 7;
pub const R9: c_int = 8;
pub const R8: c_int = 9;
pub const RAX: c_int = 10;
pub const RCX: c_int = 11;
pub const RDX: c_int = 12;
pub const RSI: c_int = 13;
pub const RDI: c_int = 14;
pub const ORIG_RAX: c_int = 15;
pub const RIP: c_int = 16;
pub const CS: c_int = 17;
pub const EFLAGS: c_int = 18;
pub const RSP: c_int = 19;
pub const SS: c_int = 20;
pub const FS_BASE: c_int = 21;
pub const GS_BASE: c_int = 22;
pub const DS: c_int = 23;
pub const ES: c_int = 24;
pub const FS: c_int = 25;
pub const GS: c_int = 26;

// offsets in mcontext_t.gregs from sys/ucontext.h
pub const REG_R8: c_int = 0;
pub const REG_R9: c_int = 1;
pub const REG_R10: c_int = 2;
pub const REG_R11: c_int = 3;
pub const REG_R12: c_int = 4;
pub const REG_R13: c_int = 5;
pub const REG_R14: c_int = 6;
pub const REG_R15: c_int = 7;
pub const REG_RDI: c_int = 8;
pub const REG_RSI: c_int = 9;
pub const REG_RBP: c_int = 10;
pub const REG_RBX: c_int = 11;
pub const REG_RDX: c_int = 12;
pub const REG_RAX: c_int = 13;
pub const REG_RCX: c_int = 14;
pub const REG_RSP: c_int = 15;
pub const REG_RIP: c_int = 16;
pub const REG_EFL: c_int = 17;
pub const REG_CSGSFS: c_int = 18;
pub const REG_ERR: c_int = 19;
pub const REG_TRAPNO: c_int = 20;
pub const REG_OLDMASK: c_int = 21;
pub const REG_CR2: c_int = 22;

pub const SECCOMP_SET_MODE_STRICT: c_uint = 0;
pub const SECCOMP_SET_MODE_FILTER: c_uint = 1;
pub const SECCOMP_GET_ACTION_AVAIL: c_uint = 2;
pub const SECCOMP_GET_NOTIF_SIZES: c_uint = 3;


pub const AF_UNSPEC: c_int = 0;
pub const AF_UNIX: c_int = 1;
pub const AF_LOCAL: c_int = 1;
pub const AF_INET: c_int = 2;
pub const AF_AX25: c_int = 3;
pub const AF_IPX: c_int = 4;
pub const AF_APPLETALK: c_int = 5;
pub const AF_NETROM: c_int = 6;
pub const AF_BRIDGE: c_int = 7;
pub const AF_ATMPVC: c_int = 8;
pub const AF_X25: c_int = 9;
pub const AF_INET6: c_int = 10;
pub const AF_ROSE: c_int = 11;
pub const AF_DECnet: c_int = 12;
pub const AF_NETBEUI: c_int = 13;
pub const AF_SECURITY: c_int = 14;
pub const AF_KEY: c_int = 15;
pub const AF_NETLINK: c_int = 16;
pub const AF_ROUTE: c_int = AF_NETLINK;
pub const AF_PACKET: c_int = 17;
pub const AF_ASH: c_int = 18;
pub const AF_ECONET: c_int = 19;
pub const AF_ATMSVC: c_int = 20;
pub const AF_RDS: c_int = 21;
pub const AF_SNA: c_int = 22;
pub const AF_IRDA: c_int = 23;
pub const AF_PPPOX: c_int = 24;
pub const AF_WANPIPE: c_int = 25;
pub const AF_LLC: c_int = 26;
pub const AF_CAN: c_int = 29;
pub const AF_TIPC: c_int = 30;
pub const AF_BLUETOOTH: c_int = 31;
pub const AF_IUCV: c_int = 32;
pub const AF_RXRPC: c_int = 33;
pub const AF_ISDN: c_int = 34;
pub const AF_PHONET: c_int = 35;
pub const AF_IEEE802154: c_int = 36;
pub const AF_CAIF: c_int = 37;
pub const AF_ALG: c_int = 38;

pub const IPPROTO_ICMP: c_int = 1;
pub const IPPROTO_ICMPV6: c_int = 58;
pub const IPPROTO_TCP: c_int = 6;
pub const IPPROTO_UDP: c_int = 17;
pub const IPPROTO_IP: c_int = 0;
pub const IPPROTO_IPV6: c_int = 41;

pub const IPV6_ADDRFORM: c_int = 1;
pub const IPV6_2292PKTINFO: c_int = 2;
pub const IPV6_2292HOPOPTS: c_int = 3;
pub const IPV6_2292DSTOPTS: c_int = 4;
pub const IPV6_2292RTHDR: c_int = 5;
pub const IPV6_2292PKTOPTIONS: c_int = 6;
pub const IPV6_CHECKSUM: c_int = 7;
pub const IPV6_2292HOPLIMIT: c_int = 8;
pub const IPV6_NEXTHOP: c_int = 9;
pub const IPV6_AUTHHDR: c_int = 10;
pub const IPV6_UNICAST_HOPS: c_int = 16;
pub const IPV6_MULTICAST_IF: c_int = 17;
pub const IPV6_MULTICAST_HOPS: c_int = 18;
pub const IPV6_MULTICAST_LOOP: c_int = 19;
pub const IPV6_ADD_MEMBERSHIP: c_int = 20;
pub const IPV6_DROP_MEMBERSHIP: c_int = 21;
pub const IPV6_ROUTER_ALERT: c_int = 22;
pub const IPV6_MTU_DISCOVER: c_int = 23;
pub const IPV6_MTU: c_int = 24;
pub const IPV6_RECVERR: c_int = 25;
pub const IPV6_V6ONLY: c_int = 26;
pub const IPV6_JOIN_ANYCAST: c_int = 27;
pub const IPV6_LEAVE_ANYCAST: c_int = 28;
pub const IPV6_IPSEC_POLICY: c_int = 34;
pub const IPV6_XFRM_POLICY: c_int = 35;
pub const IPV6_HDRINCL: c_int = 36;
pub const IPV6_RECVPKTINFO: c_int = 49;
pub const IPV6_PKTINFO: c_int = 50;
pub const IPV6_RECVHOPLIMIT: c_int = 51;
pub const IPV6_HOPLIMIT: c_int = 52;
pub const IPV6_RECVHOPOPTS: c_int = 53;
pub const IPV6_HOPOPTS: c_int = 54;
pub const IPV6_RTHDRDSTOPTS: c_int = 55;
pub const IPV6_RECVRTHDR: c_int = 56;
pub const IPV6_RTHDR: c_int = 57;
pub const IPV6_RECVDSTOPTS: c_int = 58;
pub const IPV6_DSTOPTS: c_int = 59;
pub const IPV6_RECVPATHMTU: c_int = 60;
pub const IPV6_PATHMTU: c_int = 61;
pub const IPV6_DONTFRAG: c_int = 62;
pub const IPV6_RECVTCLASS: c_int = 66;
pub const IPV6_TCLASS: c_int = 67;
pub const IPV6_AUTOFLOWLABEL: c_int = 70;
pub const IPV6_ADDR_PREFERENCES: c_int = 72;
pub const IPV6_MINHOPCOUNT: c_int = 73;
pub const IPV6_ORIGDSTADDR: c_int = 74;
pub const IPV6_RECVORIGDSTADDR: c_int = IPV6_ORIGDSTADDR;
pub const IPV6_TRANSPARENT: c_int = 75;
pub const IPV6_UNICAST_IF: c_int = 76;
pub const IPV6_PREFER_SRC_TMP: c_int = 0x0001;
pub const IPV6_PREFER_SRC_PUBLIC: c_int = 0x0002;
pub const IPV6_PREFER_SRC_PUBTMP_DEFAULT: c_int = 0x0100;
pub const IPV6_PREFER_SRC_COA: c_int = 0x0004;
pub const IPV6_PREFER_SRC_HOME: c_int = 0x0400;
pub const IPV6_PREFER_SRC_CGA: c_int = 0x0008;
pub const IPV6_PREFER_SRC_NONCGA: c_int = 0x0800;

pub const IPV6_PMTUDISC_DONT: c_int = 0;
pub const IPV6_PMTUDISC_WANT: c_int = 1;
pub const IPV6_PMTUDISC_DO: c_int = 2;
pub const IPV6_PMTUDISC_PROBE: c_int = 3;
pub const IPV6_PMTUDISC_INTERFACE: c_int = 4;
pub const IPV6_PMTUDISC_OMIT: c_int = 5;

pub const TCP_NODELAY: c_int = 1;
pub const TCP_MAXSEG: c_int = 2;
pub const TCP_CORK: c_int = 3;
pub const TCP_KEEPIDLE: c_int = 4;
pub const TCP_KEEPINTVL: c_int = 5;
pub const TCP_KEEPCNT: c_int = 6;
pub const TCP_SYNCNT: c_int = 7;
pub const TCP_LINGER2: c_int = 8;
pub const TCP_DEFER_ACCEPT: c_int = 9;
pub const TCP_WINDOW_CLAMP: c_int = 10;
pub const TCP_INFO: c_int = 11;
pub const TCP_QUICKACK: c_int = 12;
pub const TCP_CONGESTION: c_int = 13;

pub const MSG_OOB: c_int = 1;
pub const MSG_PEEK: c_int = 2;
pub const MSG_DONTROUTE: c_int = 4;
pub const MSG_CTRUNC: c_int = 8;
pub const MSG_TRUNC: c_int = 0x20;
pub const MSG_DONTWAIT: c_int = 0x40;
pub const MSG_EOR: c_int = 0x80;
pub const MSG_WAITALL: c_int = 0x100;
pub const MSG_FIN: c_int = 0x200;
pub const MSG_SYN: c_int = 0x400;
pub const MSG_CONFIRM: c_int = 0x800;
pub const MSG_RST: c_int = 0x1000;
pub const MSG_ERRQUEUE: c_int = 0x2000;
pub const MSG_NOSIGNAL: c_int = 0x4000;
pub const MSG_MORE: c_int = 0x8000;
pub const MSG_WAITFORONE: c_int = 0x10000;
pub const MSG_FASTOPEN: c_int = 0x20000000;
pub const MSG_CMSG_CLOEXEC: c_int = 0x40000000;

// include/uapi/asm-generic/socket.h
// arch/alpha/include/uapi/asm/socket.h
// tools/include/uapi/asm-generic/socket.h
// arch/mips/include/uapi/asm/socket.h
pub const SOL_SOCKET: c_int = 1;

// Defined in unix/linux_like/mod.rs
// pub const SO_DEBUG: c_int = 1;
pub const SO_REUSEADDR: c_int = 2;
pub const SO_TYPE: c_int = 3;
pub const SO_ERROR: c_int = 4;
pub const SO_DONTROUTE: c_int = 5;
pub const SO_BROADCAST: c_int = 6;
pub const SO_SNDBUF: c_int = 7;
pub const SO_RCVBUF: c_int = 8;
pub const SO_KEEPALIVE: c_int = 9;
pub const SO_OOBINLINE: c_int = 10;
pub const SO_NO_CHECK: c_int = 11;
pub const SO_PRIORITY: c_int = 12;
pub const SO_LINGER: c_int = 13;
pub const SO_BSDCOMPAT: c_int = 14;
pub const SO_REUSEPORT: c_int = 15;
pub const SO_PASSCRED: c_int = 16;
pub const SO_PEERCRED: c_int = 17;
pub const SO_RCVLOWAT: c_int = 18;
pub const SO_SNDLOWAT: c_int = 19;
pub const SO_RCVTIMEO: c_int = 20;
pub const SO_SNDTIMEO: c_int = 21;
// pub const SO_RCVTIMEO_OLD: c_int = 20;
// pub const SO_SNDTIMEO_OLD: c_int = 21;
pub const SO_SECURITY_AUTHENTICATION: c_int = 22;
pub const SO_SECURITY_ENCRYPTION_TRANSPORT: c_int = 23;
pub const SO_SECURITY_ENCRYPTION_NETWORK: c_int = 24;
pub const SO_BINDTODEVICE: c_int = 25;
pub const SO_ATTACH_FILTER: c_int = 26;
pub const SO_DETACH_FILTER: c_int = 27;
pub const SO_GET_FILTER: c_int = SO_ATTACH_FILTER;
pub const SO_PEERNAME: c_int = 28;
pub const SO_TIMESTAMP: c_int = 29;
// pub const SO_TIMESTAMP_OLD: c_int = 29;
pub const SO_ACCEPTCONN: c_int = 30;
pub const SO_PEERSEC: c_int = 31;
pub const SO_SNDBUFFORCE: c_int = 32;
pub const SO_RCVBUFFORCE: c_int = 33;
pub const SO_PASSSEC: c_int = 34;
pub const SO_TIMESTAMPNS: c_int = 35;
// pub const SO_TIMESTAMPNS_OLD: c_int = 35;
pub const SO_MARK: c_int = 36;
pub const SO_TIMESTAMPING: c_int = 37;
// pub const SO_TIMESTAMPING_OLD: c_int = 37;
pub const SO_PROTOCOL: c_int = 38;
pub const SO_DOMAIN: c_int = 39;
pub const SO_RXQ_OVFL: c_int = 40;
pub const SO_WIFI_STATUS: c_int = 41;
pub const SCM_WIFI_STATUS: c_int = SO_WIFI_STATUS;
pub const SO_PEEK_OFF: c_int = 42;
pub const SO_NOFCS: c_int = 43;
pub const SO_LOCK_FILTER: c_int = 44;
pub const SO_SELECT_ERR_QUEUE: c_int = 45;
pub const SO_BUSY_POLL: c_int = 46;
pub const SO_MAX_PACING_RATE: c_int = 47;
pub const SO_BPF_EXTENSIONS: c_int = 48;
pub const SO_INCOMING_CPU: c_int = 49;
pub const SO_ATTACH_BPF: c_int = 50;
pub const SO_DETACH_BPF: c_int = SO_DETACH_FILTER;
pub const SO_ATTACH_REUSEPORT_CBPF: c_int = 51;
pub const SO_ATTACH_REUSEPORT_EBPF: c_int = 52;
pub const SO_CNX_ADVICE: c_int = 53;
pub const SCM_TIMESTAMPING_OPT_STATS: c_int = 54;
pub const SO_MEMINFO: c_int = 55;
pub const SO_INCOMING_NAPI_ID: c_int = 56;
pub const SO_COOKIE: c_int = 57;
pub const SCM_TIMESTAMPING_PKTINFO: c_int = 58;
pub const SO_PEERGROUPS: c_int = 59;
pub const SO_ZEROCOPY: c_int = 60;
pub const SO_TXTIME: c_int = 61;
pub const SCM_TXTIME: c_int = SO_TXTIME;
pub const SO_BINDTOIFINDEX: c_int = 62;

pub const SCM_TIMESTAMP: c_int = SO_TIMESTAMP;

pub const SOCK_RAW: c_int = 3;
pub const SOCK_RDM: c_int = 4;
pub const IP_TOS: c_int = 1;
pub const IP_TTL: c_int = 2;
pub const IP_HDRINCL: c_int = 3;
pub const IP_OPTIONS: c_int = 4;
pub const IP_ROUTER_ALERT: c_int = 5;
pub const IP_RECVOPTS: c_int = 6;
pub const IP_RETOPTS: c_int = 7;
pub const IP_PKTINFO: c_int = 8;
pub const IP_PKTOPTIONS: c_int = 9;
pub const IP_MTU_DISCOVER: c_int = 10;
pub const IP_RECVERR: c_int = 11;
pub const IP_RECVTTL: c_int = 12;
pub const IP_RECVTOS: c_int = 13;
pub const IP_MTU: c_int = 14;
pub const IP_FREEBIND: c_int = 15;
pub const IP_IPSEC_POLICY: c_int = 16;
pub const IP_XFRM_POLICY: c_int = 17;
pub const IP_PASSSEC: c_int = 18;
pub const IP_TRANSPARENT: c_int = 19;
pub const IP_ORIGDSTADDR: c_int = 20;
pub const IP_RECVORIGDSTADDR: c_int = IP_ORIGDSTADDR;
pub const IP_MINTTL: c_int = 21;
pub const IP_NODEFRAG: c_int = 22;
pub const IP_CHECKSUM: c_int = 23;
pub const IP_BIND_ADDRESS_NO_PORT: c_int = 24;
pub const IP_MULTICAST_IF: c_int = 32;
pub const IP_MULTICAST_TTL: c_int = 33;
pub const IP_MULTICAST_LOOP: c_int = 34;
pub const IP_ADD_MEMBERSHIP: c_int = 35;
pub const IP_DROP_MEMBERSHIP: c_int = 36;
pub const IP_UNBLOCK_SOURCE: c_int = 37;
pub const IP_BLOCK_SOURCE: c_int = 38;
pub const IP_ADD_SOURCE_MEMBERSHIP: c_int = 39;
pub const IP_DROP_SOURCE_MEMBERSHIP: c_int = 40;
pub const IP_MSFILTER: c_int = 41;
pub const IP_MULTICAST_ALL: c_int = 49;
pub const IP_UNICAST_IF: c_int = 50;

pub const IP_DEFAULT_MULTICAST_TTL: c_int = 1;
pub const IP_DEFAULT_MULTICAST_LOOP: c_int = 1;

pub const IP_PMTUDISC_DONT: c_int = 0;
pub const IP_PMTUDISC_WANT: c_int = 1;
pub const IP_PMTUDISC_DO: c_int = 2;
pub const IP_PMTUDISC_PROBE: c_int = 3;
pub const IP_PMTUDISC_INTERFACE: c_int = 4;
pub const IP_PMTUDISC_OMIT: c_int = 5;

pub const SOCK_SEQPACKET: c_int = 5;

pub const POLLIN: c_short = 0x1;
pub const POLLPRI: c_short = 0x2;
pub const POLLOUT: c_short = 0x4;
pub const POLLERR: c_short = 0x8;
pub const POLLHUP: c_short = 0x10;
pub const POLLNVAL: c_short = 0x20;
pub const POLLRDNORM: c_short = 0x040;
pub const POLLRDBAND: c_short = 0x080;
#[cfg(not(any(target_arch = "sparc", target_arch = "sparc64")))]
pub const POLLRDHUP: c_short = 0x2000;
#[cfg(any(target_arch = "sparc", target_arch = "sparc64"))]
pub const POLLRDHUP: c_short = 0x800;

pub const F_DUPFD: c_int = 0;
pub const F_GETFD: c_int = 1;
pub const F_SETFD: c_int = 2;
pub const F_GETFL: c_int = 3;
pub const F_SETFL: c_int = 4;
pub const FD_CLOEXEC: c_int = 0x1;

// Linux-specific fcntls
pub const F_SETLEASE: c_int = 1024;
pub const F_GETLEASE: c_int = 1025;
pub const F_NOTIFY: c_int = 1026;
pub const F_CANCELLK: c_int = 1029;
pub const F_DUPFD_CLOEXEC: c_int = 1030;
pub const F_SETPIPE_SZ: c_int = 1031;
pub const F_GETPIPE_SZ: c_int = 1032;
pub const F_ADD_SEALS: c_int = 1033;
pub const F_GET_SEALS: c_int = 1034;

pub const F_SEAL_SEAL: c_int = 0x0001;
pub const F_SEAL_SHRINK: c_int = 0x0002;
pub const F_SEAL_GROW: c_int = 0x0004;
pub const F_SEAL_WRITE: c_int = 0x0008;

pub const SHUT_RD: c_int = 0;
pub const SHUT_WR: c_int = 1;
pub const SHUT_RDWR: c_int = 2;

#[derive(Debug, Clone, Copy)]
pub struct in_addr {
    pub s_addr: in_addr_t,
}

#[derive(Debug, Clone, Copy)]
#[repr(align(4))]
pub struct in6_addr {
    pub s6_addr: [u8; 16],
}

#[derive(Debug)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [c_char; 14],
}

#[derive(Debug)]
pub struct sockaddr_in {
    pub sin_family: sa_family_t,
    pub sin_port: in_port_t,
    pub sin_addr: in_addr,
    pub sin_zero: [u8; 8],
}

#[derive(Debug)]
pub struct sockaddr_in6 {
    pub sin6_family: sa_family_t,
    pub sin6_port: in_port_t,
    pub sin6_flowinfo: u32,
    pub sin6_addr: in6_addr,
    pub sin6_scope_id: u32,
}

#[derive(Debug, Clone)]
pub struct sockaddr_storage {
    pub ss_family: sa_family_t,
    #[cfg(target_pointer_width = "32")]
    __ss_pad2: [u8; 128 - 2 - 4],
    #[cfg(target_pointer_width = "64")]
    __ss_pad2: [u8; 128 - 2 - 8],
    __ss_align: size_t,
}

#[derive(Debug)]
pub struct ipv6_mreq {
    pub ipv6mr_multiaddr: in6_addr,
    #[cfg(target_os = "android")]
    pub ipv6mr_interface: c_int,
    #[cfg(not(target_os = "android"))]
    pub ipv6mr_interface: c_uint,
}

#[derive(Debug)]
pub struct ip_mreq {
    pub imr_multiaddr: in_addr,
    pub imr_interface: in_addr,
}

#[derive(Debug)]
pub struct linger {
    pub l_onoff: c_int,
    pub l_linger: c_int,
}

#[derive(Debug)]
pub struct ip_mreq_source {
    pub imr_multiaddr: in_addr,
    pub imr_interface: in_addr,
    pub imr_sourceaddr: in_addr,
}

#[derive(Debug)]
pub struct iovec {
    pub iov_base: *mut std::ffi::c_void,
    pub iov_len: size_t,
}

#[derive(Debug)]
pub struct pollfd {
    pub fd: c_int,
    pub events: c_short,
    pub revents: c_short,
}

#[derive(Debug)]
pub struct timeval {
    pub tv_sec: time_t,
    pub tv_usec: suseconds_t,
}

cfg_if! {
    if #[cfg(all(target_arch = "aarch64", target_pointer_width = "32"))] {
        pub type clock_t = i32;
        pub type time_t = i32;
        pub type __fsword_t = i32;
    } else {
        pub type __fsword_t = i64;
        pub type clock_t = i64;
        pub type time_t = i64;
    }
}

/// Raw file descriptors.
#[cfg(not(target_os = "hermit"))]
pub type RawFd = c_int;
#[rustc_allowed_through_unstable_modules]
#[stable(feature = "rust1", since = "1.0.0")]
#[cfg(target_os = "hermit")]
pub type RawFd = i32;

/// A trait to extract the raw file descriptor from an underlying object.
///
/// This is only available on unix and WASI platforms and must be imported in
/// order to call the method. Windows platforms have a corresponding
/// `AsRawHandle` and `AsRawSocket` set of traits.
pub trait AsRawFd {
    /// Extracts the raw file descriptor.
    ///
    /// This function is typically used to **borrow** an owned file descriptor.
    /// When used in this way, this method does **not** pass ownership of the
    /// raw file descriptor to the caller, and the file descriptor is only
    /// guaranteed to be valid while the original object has not yet been
    /// destroyed.
    ///
    /// However, borrowing is not strictly required. See [`AsFd::as_fd`]
    /// for an API which strictly borrows a file descriptor.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use std::fs::File;
    /// # use std::io;
    /// #[cfg(any(unix, target_os = "wasi"))]
    /// use std::os::fd::{AsRawFd, RawFd};
    ///
    /// let mut f = File::open("foo.txt")?;
    /// // Note that `raw_fd` is only valid as long as `f` exists.
    /// #[cfg(any(unix, target_os = "wasi"))]
    /// let raw_fd: RawFd = f.as_raw_fd();
    /// # Ok::<(), io::Error>(())
    /// ```
    fn as_raw_fd(&self) -> RawFd;
}

/// A trait to express the ability to construct an object from a raw file
/// descriptor.
pub trait FromRawFd {
    /// Constructs a new instance of `Self` from the given raw file
    /// descriptor.
    ///
    /// This function is typically used to **consume ownership** of the
    /// specified file descriptor. When used in this way, the returned object
    /// will take responsibility for closing it when the object goes out of
    /// scope.
    ///
    /// However, consuming ownership is not strictly required. Use a
    /// [`From<OwnedFd>::from`] implementation for an API which strictly
    /// consumes ownership.
    ///
    /// # Safety
    ///
    /// The `fd` passed in must be a valid and open file descriptor.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use std::fs::File;
    /// # use std::io;
    /// #[cfg(any(unix, target_os = "wasi"))]
    /// use std::os::fd::{FromRawFd, IntoRawFd, RawFd};
    ///
    /// let f = File::open("foo.txt")?;
    /// # #[cfg(any(unix, target_os = "wasi"))]
    /// let raw_fd: RawFd = f.into_raw_fd();
    /// // SAFETY: no other functions should call `from_raw_fd`, so there
    /// // is only one owner for the file descriptor.
    /// # #[cfg(any(unix, target_os = "wasi"))]
    /// let f = unsafe { File::from_raw_fd(raw_fd) };
    /// # Ok::<(), io::Error>(())
    /// ```
    unsafe fn from_raw_fd(fd: RawFd) -> Self;
}

/// A trait to express the ability to consume an object and acquire ownership of
/// its raw file descriptor.
pub trait IntoRawFd {
    /// Consumes this object, returning the raw underlying file descriptor.
    ///
    /// This function is typically used to **transfer ownership** of the underlying
    /// file descriptor to the caller. When used in this way, callers are then the unique
    /// owners of the file descriptor and must close it once it's no longer needed.
    ///
    /// However, transferring ownership is not strictly required. Use a
    /// [`Into<OwnedFd>::into`] implementation for an API which strictly
    /// transfers ownership.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use std::fs::File;
    /// # use std::io;
    /// #[cfg(any(unix, target_os = "wasi"))]
    /// use std::os::fd::{IntoRawFd, RawFd};
    ///
    /// let f = File::open("foo.txt")?;
    /// #[cfg(any(unix, target_os = "wasi"))]
    /// let raw_fd: RawFd = f.into_raw_fd();
    /// # Ok::<(), io::Error>(())
    /// ```
    fn into_raw_fd(self) -> RawFd;
}


impl AsRawFd for RawFd {
    #[inline]
    fn as_raw_fd(&self) -> RawFd {
        *self
    }
}

impl IntoRawFd for RawFd {
    #[inline]
    fn into_raw_fd(self) -> RawFd {
        self
    }
}

impl FromRawFd for RawFd {
    #[inline]
    unsafe fn from_raw_fd(fd: RawFd) -> RawFd {
        fd
    }
}