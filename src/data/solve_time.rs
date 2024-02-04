use std::{
    cmp::Ordering,
    fmt::Display,
    iter::Sum,
    ops::{Add, Div, Sub},
    time::Duration,
};

use crate::data::Penalty;

/// Time of a solve.
#[derive(Debug, Clone, Copy, Default)]
pub struct SolveTime {
    /// The measured time.
    pub time: Duration,
    /// The penalty of the solve.
    pub penalty: Penalty,
}

impl SolveTime {
    pub const DNF: Self = Self {
        time: Duration::ZERO,
        penalty: Penalty::Dnf,
    };

    /// Creates a new `SolveTime`.
    pub fn new(time: Duration, penalty: Penalty) -> Self {
        let millis = time.as_millis() as u64;
        Self {
            time: Duration::from_millis(millis - millis % 10),
            penalty,
        }
    }

    /// Gets the measured time.
    pub fn measured_time(&self) -> Duration {
        self.time
    }

    /// Gets the recorded time (with penalty).
    /// Returns `None` if the solve is DNF.
    pub fn recorded_time(&self) -> Option<Duration> {
        match self.penalty {
            Penalty::Ok => Some(self.time),
            Penalty::Plus2 => Some(self.time + Duration::new(2, 0)),
            Penalty::Dnf => None,
        }
    }

    /// Returns `true` if the solve is DNF.
    pub fn is_dnf(&self) -> bool {
        self.penalty == Penalty::Dnf
    }

    /// Returns `true` if the solve is +2.
    pub fn is_plus2(&self) -> bool {
        self.penalty == Penalty::Plus2
    }

    pub fn eq_aprrox(&self, other: &Self, eps_millis: u128) -> bool {
        if other.is_dnf() {
            return self.is_dnf();
        }
        let eps = if self > other {
            self - other
        } else {
            other - self
        };
        eps.recorded_time()
            .map(|x| x.as_millis() <= eps_millis)
            .unwrap_or_default()
    }
}

impl PartialEq for SolveTime {
    fn eq(&self, other: &Self) -> bool {
        self.recorded_time() == other.recorded_time()
    }
}

impl Eq for SolveTime {}

impl PartialOrd for SolveTime {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SolveTime {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self.recorded_time(), other.recorded_time()) {
            (Some(a), Some(b)) => a.cmp(&b),
            (Some(_), None) => Ordering::Less,
            (None, Some(_)) => Ordering::Greater,
            (None, None) => Ordering::Equal,
        }
    }
}

impl Display for SolveTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let rec_time = self.recorded_time().unwrap_or_default();
        match self.penalty {
            Penalty::Ok => write!(f, "{}", display_time(&rec_time)),
            Penalty::Plus2 => write!(f, "{}+", display_time(&rec_time)),
            Penalty::Dnf => write!(f, "DNF"),
        }
    }
}

impl Add for &SolveTime {
    type Output = SolveTime;

    fn add(self, rhs: Self) -> Self::Output {
        if self.is_dnf() || rhs.is_dnf() {
            SolveTime {
                time: Duration::ZERO,
                penalty: Penalty::Dnf,
            }
        } else {
            SolveTime {
                time: self.recorded_time().unwrap_or_default()
                    + rhs.recorded_time().unwrap_or_default(),
                penalty: Penalty::Ok,
            }
        }
    }
}

impl Add for SolveTime {
    type Output = SolveTime;

    fn add(self, rhs: Self) -> Self::Output {
        (&self).add(&rhs)
    }
}

impl Sub for &SolveTime {
    type Output = SolveTime;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.is_dnf() || rhs.is_dnf() {
            SolveTime {
                time: Duration::ZERO,
                penalty: Penalty::Dnf,
            }
        } else {
            SolveTime {
                time: self.recorded_time().unwrap_or_default()
                    - rhs.recorded_time().unwrap_or_default(),
                penalty: Penalty::Ok,
            }
        }
    }
}

impl Sub for SolveTime {
    type Output = SolveTime;

    fn sub(self, rhs: Self) -> Self::Output {
        (&self).sub(&rhs)
    }
}

impl Div<u32> for SolveTime {
    type Output = Self;

    fn div(self, rhs: u32) -> Self::Output {
        Self {
            time: self.recorded_time().map(|t| t / rhs).unwrap_or_default(),
            penalty: self.penalty,
        }
    }
}

impl Sum<SolveTime> for SolveTime {
    fn sum<I: Iterator<Item = SolveTime>>(iter: I) -> Self {
        iter.fold(Self::default(), |acc, x| acc + x)
    }
}

impl From<&SolveTime> for SolveTime {
    fn from(value: &SolveTime) -> Self {
        *value
    }
}

fn display_time(time: &Duration) -> String {
    let hundreths = ((time.as_millis() / 10) % 100) as u32;
    let seconds = (time.as_secs() % 60) as u32;
    let minutes = (time.as_secs() / 60) as u32;

    if minutes >= 1 {
        format!("{}:{:02}.{:02}", minutes, seconds, hundreths)
    } else {
        format!("{}.{:02}", seconds, hundreths)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_recorded_time() {
        assert_eq!(
            SolveTime::new(Duration::from_millis(8_350), Penalty::Ok).recorded_time(),
            Some(Duration::from_millis(8_350)),
        );
    }

    #[test]
    fn get_recorded_time_plus2() {
        assert_eq!(
            SolveTime::new(Duration::from_millis(42_020), Penalty::Plus2).recorded_time(),
            Some(Duration::from_millis(44_020)),
        );
    }

    #[test]
    fn get_recorded_time_dnf() {
        assert_eq!(
            SolveTime::new(Duration::from_millis(12_400), Penalty::Dnf).recorded_time(),
            None,
        );
    }

    #[test]
    fn display_solve_time() {
        assert_eq!(
            SolveTime::new(Duration::from_millis(8_350), Penalty::Ok).to_string(),
            "8.35".to_string(),
        );
    }

    #[test]
    fn display_solve_time_plus2() {
        assert_eq!(
            SolveTime::new(Duration::from_millis(42_020), Penalty::Plus2).to_string(),
            "44.02+".to_string(),
        );
    }

    #[test]
    fn display_solve_time_dnf() {
        assert_eq!(
            SolveTime::new(Duration::from_millis(12_400), Penalty::Dnf).to_string(),
            "DNF".to_string(),
        );
    }
}
