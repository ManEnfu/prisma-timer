use gtk::glib;

/// Penalty of a solve.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, glib::Enum)]
#[enum_type(name = "PtPenalty")]
#[repr(u32)]
pub enum Penalty {
    /// No penalty.
    #[default]
    #[enum_value(name = "Ok", nick = "ok")]
    Ok = 0,
    /// +2. Add two seconds penalty to the solve time.
    #[enum_value(name = "+2", nick = "plus-two")]
    Plus2 = 1,
    /// Did not finish (DNF)
    #[enum_value(name = "DNF", nick = "dnf")]
    Dnf = 2,
}
