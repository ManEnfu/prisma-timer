use std::{
    cmp::Ordering,
    fmt::Display,
    iter::Sum,
    ops::{Add, Div, Sub},
    time::{Duration, SystemTime},
};

/// Penalty of a solve.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Penalty {
    /// +2
    Plus2,
    /// Did not finish (DNF)
    Dnf,
}

/// Time of a solve.
#[derive(Debug, Clone, Copy, Default)]
pub struct SolveTime {
    /// The measured time.
    pub time: Duration,
    /// The penalty of the solve.
    pub penalty: Option<Penalty>,
}

impl SolveTime {
    pub const DNF: Self = Self {
        time: Duration::ZERO,
        penalty: Some(Penalty::Dnf),
    };

    /// Create a new `SolveTime`.
    pub fn new(time: Duration, penalty: Option<Penalty>) -> Self {
        let millis = time.as_millis() as u64;
        Self {
            time: Duration::from_millis(millis - millis % 10),
            penalty,
        }
    }

    /// Get the measured time.
    pub fn measured_time(&self) -> Duration {
        self.time
    }

    /// Get the recorded time (with penalty).
    pub fn recorded_time(&self) -> Option<Duration> {
        match self.penalty {
            None => Some(self.time),
            Some(Penalty::Plus2) => Some(self.time + Duration::new(2, 0)),
            Some(Penalty::Dnf) => None,
        }
    }

    /// Return `true` if the solve is DNF.
    pub fn is_dnf(&self) -> bool {
        self.penalty == Some(Penalty::Dnf)
    }

    /// Return `true` if the solve is +2.
    pub fn is_plus2(&self) -> bool {
        self.penalty == Some(Penalty::Plus2)
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
        // self.get_time_with_penalty().cmp(&other.get_time_with_penalty())
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
            None => write!(f, "{}", display_time(&rec_time)),
            Some(Penalty::Plus2) => write!(f, "{}+", display_time(&rec_time)),
            Some(Penalty::Dnf) => write!(f, "DNF"),
        }
    }
}

impl Add for SolveTime {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let time =
            self.recorded_time().unwrap_or_default() + rhs.recorded_time().unwrap_or_default();
        let penalty = if self.is_dnf() || rhs.is_dnf() {
            Some(Penalty::Dnf)
        } else {
            None
        };
        Self { time, penalty }
    }
}

impl Sub for SolveTime {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let time =
            self.recorded_time().unwrap_or_default() - rhs.recorded_time().unwrap_or_default();
        let penalty = if self.is_dnf() || rhs.is_dnf() {
            Some(Penalty::Dnf)
        } else {
            None
        };
        Self { time, penalty }
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

/// A trait for a sequence of solves.
pub trait SolvesSeq {
    fn mean_of_n(&self) -> Option<SolveTime>;
    fn average_of_n(&self) -> Option<SolveTime>;
}

/// A solve.
#[derive(Debug, Clone)]
pub struct SolveData {
    pub time: SolveTime,
    pub timestamp: SystemTime,
    pub scramble: String,
}

impl SolveData {
    pub fn new(time: SolveTime, scramble: String) -> Self {
        Self {
            time,
            timestamp: SystemTime::now(),
            scramble,
        }
    }
}

impl SolvesSeq for &[SolveData] {
    fn mean_of_n(&self) -> Option<SolveTime> {
        let len = self.len() as u32;
        if len == 0 {
            return None;
        }

        let sum: SolveTime = self.iter().map(|s| s.time).sum();
        Some(sum / len)
    }

    fn average_of_n(&self) -> Option<SolveTime> {
        let len = self.len() as u32;
        if len < 3 {
            return None;
        }

        let it = self.iter().map(|s| s.time).enumerate();
        let (imax, _max) = it.clone().max_by_key(|&(_, st)| st)?;
        let (imin, _min) = it.clone().min_by_key(|&(_, st)| st)?;
        let sum = it.fold(SolveTime::default(), |acc, (i, st)| {
            if i != imax && i != imin {
                acc + st
            } else {
                acc
            }
        });
        Some(sum / (len - 2))
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
mod test {
    use super::*;

    #[test]
    fn get_recorded_time() {
        assert_eq!(
            SolveTime::new(Duration::from_millis(8_350), None).recorded_time(),
            Some(Duration::from_millis(8_350)),
        );

        assert_eq!(
            SolveTime::new(Duration::from_millis(62_920), None).recorded_time(),
            Some(Duration::from_millis(62_920)),
        );

        assert_eq!(
            SolveTime::new(Duration::from_millis(142_250), None).recorded_time(),
            Some(Duration::from_millis(142_250)),
        );

        assert_eq!(
            SolveTime::new(Duration::from_millis(42_020), Some(Penalty::Plus2)).recorded_time(),
            Some(Duration::from_millis(44_020)),
        );

        assert_eq!(
            SolveTime::new(Duration::from_millis(12_400), Some(Penalty::Dnf)).recorded_time(),
            None,
        );
    }

    #[test]
    fn display_solve_time() {
        assert_eq!(
            SolveTime::new(Duration::from_millis(8_350), None).to_string(),
            "8.35".to_string(),
        );

        assert_eq!(
            SolveTime::new(Duration::from_millis(62_920), None).to_string(),
            "1:02.92".to_string(),
        );

        assert_eq!(
            SolveTime::new(Duration::from_millis(142_250), None).to_string(),
            "2:22.25".to_string(),
        );

        assert_eq!(
            SolveTime::new(Duration::from_millis(42_020), Some(Penalty::Plus2)).to_string(),
            "44.02+".to_string(),
        );

        assert_eq!(
            SolveTime::new(Duration::from_millis(12_400), Some(Penalty::Dnf)).to_string(),
            "DNF".to_string(),
        );
    }
}
