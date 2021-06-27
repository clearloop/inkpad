use parity_scale_codec::{Decode, Encode};

bitflags! {
    /// Flags used by a contract to customize exit behaviour.
    #[derive(Encode, Decode)]
    pub struct ReturnFlags: u32 {
        /// If this bit is set all changes made by the contract execution are rolled back.
        const REVERT = 0x0000_0001;
    }
}
