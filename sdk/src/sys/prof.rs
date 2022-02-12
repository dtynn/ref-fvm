super::fvm_syscalls! {
    module = "prof";

    pub fn capture_coverage(data_off: *const u8, data_len: u32) -> Result<()>;
}
