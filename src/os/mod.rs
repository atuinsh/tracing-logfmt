//! Operating system utilities.

#[cfg(target_vendor = "apple")]
mod apple;
#[cfg(any(target_os = "linux", target_os = "android"))]
mod linux;
#[cfg(target_os = "windows")]
mod windows;

/// Gets the current thread id as an integer.
///
/// [`Option::None`] if the active thread could not be determined or the
/// platform doesn't support it.
///
/// Note that a Rust `ThreadId` is actually just a process specific atomic,
/// completely unrelated to the thread id assigned by the operating system, but
/// we use the operating system one, at least until they stabilize
/// <https://doc.rust-lang.org/std/thread/struct.ThreadId.html#method.as_u64>
#[inline]
pub(crate) fn gettid() -> Option<u64> {
    #[cfg(target_vendor = "apple")]
    {
        Some(apple::gettid())
    }
    #[cfg(any(target_os = "linux", target_os = "android"))]
    {
        Some(linux::gettid())
    }
    #[cfg(target_os = "windows")]
    {
        Some(windows::gettid())
    }

    #[cfg(not(any(
        target_vendor = "apple",
        target_os = "linux",
        target_os = "android",
        target_os = "windows"
    )))]
    None
}

#[cfg(all(test, any(target_os = "linux", target_os = "android")))]
mod tests {
    use super::*;

    /// Calls `f` on a new thread, returning its result and the thread's id per procfs.
    ///
    /// Not the test's own thread, since under `--test-threads=1` that is the main
    /// thread, whose tid is also the pid.
    fn on_new_thread(f: fn() -> Option<u64>) -> (Option<u64>, u64) {
        std::thread::spawn(move || {
            let link = std::fs::read_link("/proc/thread-self").expect("procfs is mounted");
            let tid = link
                .file_name()
                .and_then(|name| name.to_str())
                .and_then(|name| name.parse().ok())
                .expect("/proc/thread-self ends in the tid");

            (f(), tid)
        })
        .join()
        .expect("the probe thread does not panic")
    }

    #[test]
    fn gettid_returns_the_kernel_tid() {
        let (got, tid) = on_new_thread(gettid);
        assert_eq!(got, Some(tid));
    }
}
