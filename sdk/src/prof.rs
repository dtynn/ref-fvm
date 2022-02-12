use crate::{sys, SyscallResult};

pub fn reset_coverage() {
    minicov::reset_coverage();
}

pub fn capture_coverage() -> SyscallResult<()> {
    let data = minicov::capture_coverage();

    unsafe {
        sys::prof::capture_coverage(data.as_ptr(), data.len() as u32)?;
    }

    Ok(())
}
