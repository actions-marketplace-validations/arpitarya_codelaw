//! Stopping a child process and everything it started (SR-ARCHITECTURE
//! decisions 18 and 19). The one copy: the adapter runner and the external
//! fixer both spawn their child as the leader of its own process group and
//! call [`kill_group`] at a timeout or after it exits.

use std::process::Child;

/// Stop `child` and everything it started. On Unix the child leads its own
/// process group (spawned with `CommandExt::process_group(0)`), so the whole
/// group is killed, not just the immediate child; elsewhere only the child is.
pub fn kill_group(child: &mut Child) {
    #[cfg(unix)]
    if let Ok(pid) = libc::pid_t::try_from(child.id()) {
        // SAFETY: `kill` takes plain integers and has no memory effects; a
        // group that is already gone is ESRCH, which is ignored.
        unsafe {
            libc::kill(-pid, libc::SIGKILL);
        }
    }
    let _ = child.kill();
}

/// Hand the allocator's free pages back to the system. Loading packs and
/// compiling rules frees a great deal of scratch (rule files read and parsed,
/// regexes built) between allocations that stay for the life of the process;
/// with glibc, where each thread allocates from its own arena, the holes in
/// the loading thread's arena are not reused by the worker threads, and a
/// check's peak resident memory (L8's `peak_memory_10k`) carries them. One
/// call after the load returns them (W-172). A no-op where the allocator has
/// no such call (macOS's measured no difference: one zone serves every thread).
pub fn release_free_memory() {
    #[cfg(all(target_os = "linux", target_env = "gnu"))]
    // SAFETY: `malloc_trim` takes a padding size and touches no live allocation.
    unsafe {
        libc::malloc_trim(0);
    }
}
