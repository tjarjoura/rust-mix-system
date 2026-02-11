use super::wval::WVal;

/// Pseudo-operation in MIXAL that sets the starting address for assembly
pub struct Orig {
    pub wval: WVal,
}
