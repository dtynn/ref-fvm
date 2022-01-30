super::fvm_syscalls! {
    module = "message";

    /// Returns the originator's actor ID.
    pub fn originator() -> Result<u64>;

    /// Returns the caller's actor ID.
    pub fn caller() -> Result<u64>;

    /// Returns the receiver's actor ID (i.e. ourselves).
    pub fn receiver() -> Result<u64>;

    /// Returns the method number from the message.
    pub fn method_number() -> Result<u64>;

    /// Returns the value that was received, as little-Endian
    /// tuple of u64 values to be concatenated in a u128.
    pub fn value_received() -> Result<fvm_shared::sys::TokenAmount>;
}
