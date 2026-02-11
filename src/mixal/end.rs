use super::wval::WVal;

/// Pseudo-operation in MIXAL that marks the end of the program
pub struct End {
    pub wval: WVal,
}
