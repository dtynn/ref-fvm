use super::Context;
use crate::kernel::{Kernel, Result};

pub fn capture_coverage(
    context: Context<'_, impl Kernel>,
    data_off: u32,
    data_len: u32,
) -> Result<()> {
    let data = context.memory.try_slice(data_off, data_len)?;

    println!("get coverage data, size={} B", data.len());
    std::fs::write("cov.profraw", data).expect("write cov prof raw data");

    Ok(())
}
