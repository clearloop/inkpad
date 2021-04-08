//! Host function trait

/// Host function trait
pub trait Host {
    /// Host function module
    fn module() -> &'static str;

    /// Host function name
    fn name() -> &'static str;
}
