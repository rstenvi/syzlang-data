use anyhow::Result;
use nix::fcntl::Flock;
use std::fs::File;
use std::path::PathBuf;
use std::process::Command;
use std::{env, path::Path};
use syzlang_parser::parser::{Consts, Parsed, Statement};

const LINUX_MIPS32_SYS: &str = r#"
arches = mips32

SYS_syscall = 4000
SYS_exit = 4001
SYS_fork = 4002
SYS_read = 4003
SYS_write = 4004
SYS_open = 4005
SYS_close = 4006
SYS_waitpid = 4007
SYS_creat = 4008
SYS_link = 4009
SYS_unlink = 4010
SYS_execve = 4011
SYS_chdir = 4012
SYS_time = 4013
SYS_mknod = 4014
SYS_chmod = 4015
SYS_lchown = 4016
SYS_break = 4017
SYS_lseek = 4019
SYS_getpid = 4020
SYS_mount = 4021
SYS_umount = 4022
SYS_setuid = 4023
SYS_getuid = 4024
SYS_stime = 4025
SYS_ptrace = 4026
SYS_alarm = 4027
SYS_pause = 4029
SYS_utime = 4030
SYS_stty = 4031
SYS_gtty = 4032
SYS_access = 4033
SYS_nice = 4034
SYS_ftime = 4035
SYS_sync = 4036
SYS_kill = 4037
SYS_rename = 4038
SYS_mkdir = 4039
SYS_rmdir = 4040
SYS_dup = 4041
SYS_pipe = 4042
SYS_times = 4043
SYS_prof = 4044
SYS_brk = 4045
SYS_setgid = 4046
SYS_getgid = 4047
SYS_signal = 4048
SYS_geteuid = 4049
SYS_getegid = 4050
SYS_acct = 4051
SYS_umount2 = 4052
SYS_lock = 4053
SYS_ioctl = 4054
SYS_fcntl = 4055
SYS_mpx = 4056
SYS_setpgid = 4057
SYS_ulimit = 4058
SYS_umask = 4060
SYS_chroot = 4061
SYS_ustat = 4062
SYS_dup2 = 4063
SYS_getppid = 4064
SYS_getpgrp = 4065
SYS_setsid = 4066
SYS_sigaction = 4067
SYS_sgetmask = 4068
SYS_ssetmask = 4069
SYS_setreuid = 4070
SYS_setregid = 4071
SYS_sigsuspend = 4072
SYS_sigpending = 4073
SYS_sethostname = 4074
SYS_setrlimit = 4075
SYS_getrlimit = 4076
SYS_getrusage = 4077
SYS_gettimeofday = 4078
SYS_settimeofday = 4079
SYS_getgroups = 4080
SYS_setgroups = 4081
SYS_symlink = 4083
SYS_readlink = 4085
SYS_uselib = 4086
SYS_swapon = 4087
SYS_reboot = 4088
SYS_readdir = 4089
SYS_mmap = 4090
SYS_munmap = 4091
SYS_truncate = 4092
SYS_ftruncate = 4093
SYS_fchmod = 4094
SYS_fchown = 4095
SYS_getpriority = 4096
SYS_setpriority = 4097
SYS_profil = 4098
SYS_statfs = 4099
SYS_fstatfs = 4100
SYS_ioperm = 4101
SYS_socketcall = 4102
SYS_syslog = 4103
SYS_setitimer = 4104
SYS_getitimer = 4105
SYS_stat = 4106
SYS_lstat = 4107
SYS_fstat = 4108
SYS_iopl = 4110
SYS_vhangup = 4111
SYS_idle = 4112
SYS_vm86 = 4113
SYS_wait4 = 4114
SYS_swapoff = 4115
SYS_sysinfo = 4116
SYS_ipc = 4117
SYS_fsync = 4118
SYS_sigreturn = 4119
SYS_clone = 4120
SYS_setdomainname = 4121
SYS_uname = 4122
SYS_modify_ldt = 4123
SYS_adjtimex = 4124
SYS_mprotect = 4125
SYS_sigprocmask = 4126
SYS_create_module = 4127
SYS_init_module = 4128
SYS_delete_module = 4129
SYS_get_kernel_syms = 4130
SYS_quotactl = 4131
SYS_getpgid = 4132
SYS_fchdir = 4133
SYS_bdflush = 4134
SYS_sysfs = 4135
SYS_personality = 4136
SYS_afs_syscall = 4137
SYS_setfsuid = 4138
SYS_setfsgid = 4139
SYS__llseek = 4140
SYS_getdents = 4141
SYS__newselect = 4142
SYS_flock = 4143
SYS_msync = 4144
SYS_readv = 4145
SYS_writev = 4146
SYS_cacheflush = 4147
SYS_cachectl = 4148
SYS_sysmips = 4149
SYS_getsid = 4151
SYS_fdatasync = 4152
SYS__sysctl = 4153
SYS_mlock = 4154
SYS_munlock = 4155
SYS_mlockall = 4156
SYS_munlockall = 4157
SYS_sched_setparam = 4158
SYS_sched_getparam = 4159
SYS_sched_setscheduler = 4160
SYS_sched_getscheduler = 4161
SYS_sched_yield = 4162
SYS_sched_get_priority_max = 4163
SYS_sched_get_priority_min = 4164
SYS_sched_rr_get_interval = 4165
SYS_nanosleep = 4166
SYS_mremap = 4167
SYS_accept = 4168
SYS_bind = 4169
SYS_connect = 4170
SYS_getpeername = 4171
SYS_getsockname = 4172
SYS_getsockopt = 4173
SYS_listen = 4174
SYS_recv = 4175
SYS_recvfrom = 4176
SYS_recvmsg = 4177
SYS_send = 4178
SYS_sendmsg = 4179
SYS_sendto = 4180
SYS_setsockopt = 4181
SYS_shutdown = 4182
SYS_socket = 4183
SYS_socketpair = 4184
SYS_setresuid = 4185
SYS_getresuid = 4186
SYS_query_module = 4187
SYS_poll = 4188
SYS_nfsservctl = 4189
SYS_setresgid = 4190
SYS_getresgid = 4191
SYS_prctl = 4192
SYS_rt_sigreturn = 4193
SYS_rt_sigaction = 4194
SYS_rt_sigprocmask = 4195
SYS_rt_sigpending = 4196
SYS_rt_sigtimedwait = 4197
SYS_rt_sigqueueinfo = 4198
SYS_rt_sigsuspend = 4199
SYS_pread64 = 4200
SYS_pwrite64 = 4201
SYS_chown = 4202
SYS_getcwd = 4203
SYS_capget = 4204
SYS_capset = 4205
SYS_sigaltstack = 4206
SYS_sendfile = 4207
SYS_getpmsg = 4208
SYS_putpmsg = 4209
SYS_mmap2 = 4210
SYS_truncate64 = 4211
SYS_ftruncate64 = 4212
SYS_stat64 = 4213
SYS_lstat64 = 4214
SYS_fstat64 = 4215
SYS_pivot_root = 4216
SYS_mincore = 4217
SYS_madvise = 4218
SYS_getdents64 = 4219
SYS_fcntl64 = 4220
SYS_gettid = 4222
SYS_readahead = 4223
SYS_setxattr = 4224
SYS_lsetxattr = 4225
SYS_fsetxattr = 4226
SYS_getxattr = 4227
SYS_lgetxattr = 4228
SYS_fgetxattr = 4229
SYS_listxattr = 4230
SYS_llistxattr = 4231
SYS_flistxattr = 4232
SYS_removexattr = 4233
SYS_lremovexattr = 4234
SYS_fremovexattr = 4235
SYS_tkill = 4236
SYS_sendfile64 = 4237
SYS_futex = 4238
SYS_sched_setaffinity = 4239
SYS_sched_getaffinity = 4240
SYS_io_setup = 4241
SYS_io_destroy = 4242
SYS_io_getevents = 4243
SYS_io_submit = 4244
SYS_io_cancel = 4245
SYS_exit_group = 4246
SYS_lookup_dcookie = 4247
SYS_epoll_create = 4248
SYS_epoll_ctl = 4249
SYS_epoll_wait = 4250
SYS_remap_file_pages = 4251
SYS_set_tid_address = 4252
SYS_restart_syscall = 4253
SYS_fadvise64 = 4254
SYS_statfs64 = 4255
SYS_fstatfs64 = 4256
SYS_timer_create = 4257
SYS_timer_settime = 4258
SYS_timer_gettime = 4259
SYS_timer_getoverrun = 4260
SYS_timer_delete = 4261
SYS_clock_settime = 4262
SYS_clock_gettime = 4263
SYS_clock_getres = 4264
SYS_clock_nanosleep = 4265
SYS_tgkill = 4266
SYS_utimes = 4267
SYS_mbind = 4268
SYS_get_mempolicy = 4269
SYS_set_mempolicy = 4270
SYS_mq_open = 4271
SYS_mq_unlink = 4272
SYS_mq_timedsend = 4273
SYS_mq_timedreceive = 4274
SYS_mq_notify = 4275
SYS_mq_getsetattr = 4276
SYS_vserver = 4277
SYS_waitid = 4278
# SYS_sys_setaltroot = 4279
SYS_add_key = 4280
SYS_request_key = 4281
SYS_keyctl = 4282
SYS_set_thread_area = 4283
SYS_inotify_init = 4284
SYS_inotify_add_watch = 4285
SYS_inotify_rm_watch = 4286
SYS_migrate_pages = 4287
SYS_openat = 4288
SYS_mkdirat = 4289
SYS_mknodat = 4290
SYS_fchownat = 4291
SYS_futimesat = 4292
SYS_fstatat64 = 4293
SYS_unlinkat = 4294
SYS_renameat = 4295
SYS_linkat = 4296
SYS_symlinkat = 4297
SYS_readlinkat = 4298
SYS_fchmodat = 4299
SYS_faccessat = 4300
SYS_pselect6 = 4301
SYS_ppoll = 4302
SYS_unshare = 4303
SYS_splice = 4304
SYS_sync_file_range = 4305
SYS_tee = 4306
SYS_vmsplice = 4307
SYS_move_pages = 4308
SYS_set_robust_list = 4309
SYS_get_robust_list = 4310
SYS_kexec_load = 4311
SYS_getcpu = 4312
SYS_epoll_pwait = 4313
SYS_ioprio_set = 4314
SYS_ioprio_get = 4315
SYS_utimensat = 4316
SYS_signalfd = 4317
SYS_timerfd = 4318
SYS_eventfd = 4319
SYS_fallocate = 4320
SYS_timerfd_create = 4321
SYS_timerfd_gettime = 4322
SYS_timerfd_settime = 4323
SYS_signalfd4 = 4324
SYS_eventfd2 = 4325
SYS_epoll_create1 = 4326
SYS_dup3 = 4327
SYS_pipe2 = 4328
SYS_inotify_init1 = 4329
SYS_preadv = 4330
SYS_pwritev = 4331
SYS_rt_tgsigqueueinfo = 4332
SYS_perf_event_open = 4333
SYS_accept4 = 4334
SYS_recvmmsg = 4335
SYS_fanotify_init = 4336
SYS_fanotify_mark = 4337
SYS_prlimit64 = 4338
SYS_name_to_handle_at = 4339
SYS_open_by_handle_at = 4340
SYS_clock_adjtime = 4341
SYS_syncfs = 4342
SYS_sendmmsg = 4343
SYS_setns = 4344
SYS_process_vm_readv = 4345
SYS_process_vm_writev = 4346
SYS_kcmp = 4347
SYS_finit_module = 4348
SYS_sched_setattr = 4349
SYS_sched_getattr = 4350
SYS_renameat2 = 4351
SYS_seccomp = 4352
SYS_getrandom = 4353
SYS_memfd_create = 4354
SYS_bpf = 4355
SYS_execveat = 4356
SYS_userfaultfd = 4357
SYS_membarrier = 4358
SYS_mlock2 = 4359
SYS_copy_file_range = 4360
SYS_preadv2 = 4361
SYS_pwritev2 = 4362
SYS_pkey_mprotect = 4363
SYS_pkey_alloc = 4364
SYS_pkey_free = 4365
SYS_statx = 4366
SYS_rseq = 4367
SYS_pidfd_send_signal = 4424
SYS_io_uring_setup = 4425
SYS_io_uring_enter = 4426
SYS_io_uring_register = 4427
SYS_open_tree = 4428
SYS_move_mount = 4429
SYS_fsopen = 4430
SYS_fsconfig = 4431
SYS_fsmount = 4432
SYS_fspick = 4433
SYS_pidfd_open = 4434
SYS_clone3 = 4435
SYS_close_range = 4436
SYS_openat2 = 4437
SYS_pidfd_getfd = 4438
SYS_faccessat2 = 4439
SYS_process_madvise = 4440
SYS_epoll_pwait2 = 4441
SYS_mount_setattr = 4442
SYS_quotactl_fd = 4443
SYS_landlock_create_ruleset = 4444
SYS_landlock_add_rule = 4445
SYS_landlock_restrict_self = 4446
SYS_memfd_secret = 4447
SYS_process_mrelease = 4448
SYS_futex_waitv = 4449
SYS_set_mempolicy_home_node = 4450
"#;

fn chosen_oss() -> Vec<String> {
	let oss: Vec<&str> = vec![
		#[cfg(feature = "akaros")]
		"akaros",
		#[cfg(feature = "darwin")]
		"darwin",
		#[cfg(feature = "freebsd")]
		"freebsd",
		#[cfg(feature = "fuchsia")]
		"fuchsia",
		#[cfg(feature = "linux")]
		"linux",
		#[cfg(feature = "netbsd")]
		"netbsd",
		#[cfg(feature = "openbsd")]
		"openbsd",
		#[cfg(feature = "trusty")]
		"trusty",
		#[cfg(feature = "windows")]
		"windows",
	];
	oss.into_iter()
		.map(|x| x.to_string())
		.collect::<Vec<String>>()
}

fn generate(skdir: &mut PathBuf, outdir: &Path) -> Result<()> {
	let mut code = String::from("");

	skdir.push("sys");
	println!("skdir {skdir:?}");

	for os in chosen_oss().into_iter() {
		println!("os {os:?}");
		let mut gdir = outdir.to_path_buf();
		gdir.push(&os);

		println!("gdir {gdir:?}");
		if !gdir.is_dir() {
			std::fs::create_dir(&gdir)?;
		}

		let mut consts = Consts::default();
		let mut stmts = Vec::new();

		skdir.push(&os);
		println!("skdir {skdir:?}");

		if os == "linux" {
			let inp = Consts::create_from_str(LINUX_MIPS32_SYS, None)?;
			consts.add_vec(inp);
		}
		// std::thread::sleep(std::time::Duration::from_millis(1000));
		let paths = std::fs::read_dir(&skdir).unwrap();
		for p in paths {
			let p = p?;
			println!("reading {p:?}");
			let ft = p.file_type()?;
			if ft.is_file() {
				let path = p.path();
				println!("is file {path:?}");
				let ext = path.extension();
				if ext == Some(std::ffi::OsStr::new("const")) {
					println!("parsing const");
					consts.create_from_file(&path)?;
				} else if ext == Some(std::ffi::OsStr::new("txt")) {
					println!("parsing stmts");
					let mut stmt = Statement::from_file(&path)?;
					stmts.append(&mut stmt);
				}
			}
		}

		let inscode = format!(
			r#"
/// Data files for {os} operating system
pub mod {os} {{
	lazy_static::lazy_static! {{
		#[derive(Default)]
		pub static ref PARSED: std::sync::RwLock<syzlang_parser::parser::Parsed> = {{
			let parsed = include_bytes!(concat!(env!("OUT_DIR"), "/{os}/parsed.json"));
			let parsed = std::str::from_utf8(parsed).unwrap();
			std::sync::RwLock::new(serde_json::from_str(parsed).unwrap())
		}};
	}}
}}
		"#
		);
		code.push_str(inscode.as_str());

		let parsed = Parsed::new(consts, stmts)?;
		let out = serde_json::to_string(&parsed)?;
		gdir.push("parsed.json");
		std::fs::write(&gdir, out)?;
		gdir.pop();

		skdir.pop(); // os
	}
	skdir.pop(); // sys

	let mut gdir = outdir.to_path_buf();
	gdir.push("data.rs");
	println!("writing to {gdir:?}");
	std::fs::write(gdir, code)?;

	Ok(())
}

fn download_syzkaller(skdir: &PathBuf, max: usize) {
	if !skdir.exists() {
		println!("Directory does not exist, downloading");
		let c = Command::new("git")
			.arg("clone")
			.arg("--quiet")
			.arg("--branch")
			.arg("master")
			.arg("https://github.com/google/syzkaller.git")
			.arg(skdir)
			.output()
			.expect("Unable to download syzkaller from git");
		println!("c1 {c:?}");
		assert!(c.stderr == b"");
	} else {
		let c = Command::new("git")
			.arg("-C")
			.arg(skdir)
			.arg("checkout")
			.arg("master")
			.output()
			.expect("unable to checkout master");
		println!("c1.5 {c:?}");
		if !c.status.success() {
			if max > 0 {
				std::fs::remove_dir_all(skdir).expect("Unable to remove syzkaller directory");
				download_syzkaller(skdir, max - 1)
			} else {
				panic!("unable to download Syzkaller");
			}
		}
	}
}

fn acquire_lock(scratch: &Path) -> anyhow::Result<Flock<File>> {
	let mut lock = PathBuf::from(scratch);
	lock.push("build.lock");
	let lock = std::fs::OpenOptions::new()
		.create(true)
		.truncate(true)
		.write(true)
		.open(lock)
		.unwrap();
	let lock = nix::fcntl::Flock::lock(lock, nix::fcntl::FlockArg::LockExclusive)
		.expect("Unable to acquire lock");
	Ok(lock)
}

fn main() -> Result<()> {
	println!("Build started");
	let out_dir = env::var_os("OUT_DIR").unwrap();
	println!("out_dir = {out_dir:?}");
	let out_dir = PathBuf::from(out_dir);
	let mut skdir = out_dir.clone();
	skdir.push("sk");
	if !skdir.exists() {
		std::fs::create_dir(&skdir)?;
	}


	// Get a scratch directory to download code
	// let mut skdir = scratch::path("sk");
	println!("using path {skdir:?}");
	let mut skversion = out_dir.clone();
	skversion.push("skversion.txt");

	let oss = chosen_oss();
	if oss.is_empty() {
		std::fs::write(skversion, b"").unwrap();
		let mut gdir = out_dir.to_path_buf();
		gdir.push("data.rs");
		std::fs::write(gdir, b"")?;
	} else {
		// Get a lock so that multiple build.rs processes don't interfere with
		// eachother
		let lock = skdir.clone();
		let lock = acquire_lock(&lock)?;

		skdir.push("syzkaller");
		println!("output will be in {skdir:?}");
		download_syzkaller(&skdir, 1);

		println!("pulling newest version of Syzkaller");
		let c = Command::new("git")
			.arg("-C")
			.arg(&skdir)
			.arg("pull")
			.output()
			.expect("unable to pull newest version from syzkaller");

		println!("c2 {c:?}");
		assert!(c.status.success());

		let checked = "1834ff143d083ae2c374f2a18d887575887321a9";
		let c = Command::new("git")
			.arg("-C")
			.arg(&skdir)
			.arg("checkout")
			.arg(checked)
			.output()
			.expect("unable to get specific version {checked}");

		println!("c3 {c:?}");
		assert!(c.status.success());

		// Get last hash from git
		let hash = Command::new("git")
			.arg("-C")
			.arg(&skdir)
			.arg("rev-parse")
			.arg("HEAD")
			.output()
			.expect("unable to get git hash");

		println!("c4 {hash:?}");
		assert!(hash.status.success());
		let hash = std::str::from_utf8(&hash.stdout).unwrap();
		let hash = hash.trim_end();
		println!("git has = {hash}");
		assert!(hash == checked);

		let is_match = if skversion.is_file() {
			let data = std::fs::read(&skversion).unwrap();
			let data = std::str::from_utf8(&data).unwrap();
			data == hash
		} else {
			println!("skversion.txt does not exit, generating fresh");
			false
		};
		println!("skversion.txt returned {is_match}");

		if !is_match {
			println!("Generating fresh");
			generate(&mut skdir, &out_dir).unwrap();
			std::fs::write(skversion, hash).unwrap();
		} else {
			println!("Hashes matches, aborting fresh build");
		}

		lock.unlock().expect("unable to unlock lock");
		println!("Lock unlocked");
	}

	println!("cargo:rerun-if-changed=build.rs");
	println!("cargo:rerun-if-env-changed=CARGO_CFG_FEATURE");
	Ok(())
}
