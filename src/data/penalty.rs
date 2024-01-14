/// Penalty of a solve.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum Penalty {
    /// No penalty.
    #[default]
    Ok = 0,
    /// +2. Add two seconds penalty to the solve time.
    Plus2 = 1,
    /// Did not finish (DNF)
    Dnf = 2,
}
