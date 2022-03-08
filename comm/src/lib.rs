#![allow(unused_imports)]
use std::path::Path;
use anyhow::bail;
pub use anyhow::Result;
pub use log::{debug, info};

pub fn log_setting_with_level(level: &str) {
    std::env::set_var("RUST_LOG", level);
    env_logger::init();
}
pub fn current_path() {
    let cur: &Path = Path::new(".");
    println!("{:?}", cur.canonicalize().unwrap());
}
pub fn log_setting() {
    log_setting_with_level("debug")
}

#[cfg(not(unix))]
pub fn proc_mem() -> Result<u64> {
    bail!("不支持非unix平台");
}

#[cfg(unix)]
pub fn proc_mem() -> Result<u64> {
    let me = procfs::process::Process::myself()?;
    let page_size = procfs::page_size().expect("Unable to determinte page size!") as u64;
    let statm = me.statm()?;
    Ok((statm.resident * page_size - statm.shared * page_size) / 1024)
}

#[cfg(unix)]
pub fn test() -> Result<()> {
    let me = procfs::process::Process::myself()?;
    debug!("PID: {}", me.pid);

    let page_size = procfs::page_size().expect("Unable to determinte page size!") as u64;
    debug!("Memory page size: {}", page_size);

    // Note: when comparing the below values to what "top" will display, note that "top" will use
    // base-2 units (kibibytes), not base-10 units (kilobytes).

    debug!("== Data from /proc/self/stat:");
    debug!("Total virtual memory used: {} bytes, {} kb", me.stat.vsize, me.stat.vsize / 1024);
    debug!(
        "Total resident set(rss): {} pages ({} bytes={} kb={} mb)",
        me.stat.rss,
        me.stat.rss as u64 * page_size, me.stat.rss as u64 * page_size / 1024, me.stat.rss as u64 * page_size / 1024 / 1024
    );
    debug!("");

    if let Ok(statm) = me.statm() {
        debug!("== Data from /proc/self/statm:");
        debug!(
            "Total virtual memory used: {} pages ({} bytes={} kb={} mb)",
            statm.size,
            statm.size * page_size, statm.size * page_size/ 1024, statm.size * page_size/ 1024/ 1024
        );
        debug!(
            "Total resident set: {} pages ({} bytes={} kb={} mb)",
            statm.resident,
            statm.resident * page_size,
            statm.resident * page_size / 1024,
            statm.resident * page_size / 1024 / 1024
        );
        debug!(
            "Total shared memory: {} pages ({} bytes={} kb={} mb)",
            statm.shared,
            statm.shared * page_size,
            statm.shared * page_size / 1024, statm.shared * page_size / 1024 / 1024
        );
        debug!("");
    }

    if let Ok(status) = me.status() {
        debug!("== Data from /proc/self/status:");
        debug!(
            "Total virtual memory used: {} bytes={} kb={} mb",
            status.vmsize.expect("vmsize") * 1024, status.vmsize.expect("vmsize"), status.vmsize.expect("vmsize")/1024
        );
        debug!("Total resident set(vmrss): {} bytes={} kb={} mb", status.vmrss.expect("vmrss") * 1024, status.vmrss.expect("vmrss"), status.vmrss.expect("vmrss")/1024);
        let rsssh = status.rssfile.expect("rssfile") + status.rssshmem.expect("rssshmem");
        debug!(
            "Total shared memory(rss): {} bytes={} kb={} mb",
            rsssh * 1024,
            rsssh, rsssh / 1024
        );
    }

    Ok(())
}
#[cfg(unix)]
#[test]
fn test_test() {
    log_setting();
    test().unwrap();
}