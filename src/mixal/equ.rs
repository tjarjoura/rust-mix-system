use anyhow::Result;

use super::wval::WVal;

/// Pseudo-operation in MIXAL that sets the value of a symbol equal to
/// the given W-Value
pub struct Equ {
    pub wval: WVal,
}
