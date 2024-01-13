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

    /// Creates a new `SolveTime`.
    pub fn new(time: Duration, penalty: Option<Penalty>) -> Self {
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
            None => Some(self.time),
            Some(Penalty::Plus2) => Some(self.time + Duration::new(2, 0)),
            Some(Penalty::Dnf) => None,
        }
    }

    /// Returns `true` if the solve is DNF.
    pub fn is_dnf(&self) -> bool {
        self.penalty == Some(Penalty::Dnf)
    }

    /// Returns `true` if the solve is +2.
    pub fn is_plus2(&self) -> bool {
        self.penalty == Some(Penalty::Plus2)
    }

    pub fn eq_aprrox(&self, other: &Self, eps_millis: u128) -> bool {
        if other.is_dnf() {
            self.is_dnf()
        } else {
            let eps = if self > other {
                self - other
            } else {
                other - self
            };
            eps.recorded_time().unwrap().as_millis() <= eps_millis
        }
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
            None => write!(f, "{}", display_time(&rec_time)),
            Some(Penalty::Plus2) => write!(f, "{}+", display_time(&rec_time)),
            Some(Penalty::Dnf) => write!(f, "DNF"),
        }
    }
}

impl Add for &SolveTime {
    type Output = SolveTime;

    fn add(self, rhs: Self) -> Self::Output {
        if self.is_dnf() || rhs.is_dnf() {
            SolveTime {
                time: Duration::ZERO,
                penalty: Some(Penalty::Dnf),
            }
        } else {
            SolveTime {
                time: self.recorded_time().unwrap_or_default()
                    + rhs.recorded_time().unwrap_or_default(),
                penalty: None,
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
                penalty: Some(Penalty::Dnf),
            }
        } else {
            SolveTime {
                time: self.recorded_time().unwrap_or_default()
                    - rhs.recorded_time().unwrap_or_default(),
                penalty: None,
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

/// A trait for a sequence of solves.
pub trait SolvesSeq {
    /// Calculates the mean time of the solves.
    fn mean_of_n(&self) -> Option<SolveTime>;
    /// Calculates the average time of the solves. This is similar to
    /// `mean_of_n`, but the fastest and slowest solves are excluded
    /// from the calculation.
    fn average_of_n(&self) -> Option<SolveTime>;
}

impl SolvesSeq for &[SolveTime] {
    fn mean_of_n(&self) -> Option<SolveTime> {
        let len = self.len() as u32;
        if len == 0 {
            return None;
        }

        let sum: SolveTime = self.iter().copied().sum();
        Some(sum / len)
    }

    fn average_of_n(&self) -> Option<SolveTime> {
        let len = self.len() as u32;
        if len < 3 {
            return None;
        }

        let it = self.iter().enumerate();
        let (imax, _max) = it.clone().max_by_key(|&(_, st)| st)?;
        let (imin, _min) = it.clone().min_by_key(|&(_, st)| st)?;
        let sum = it.fold(SolveTime::default(), |acc, (i, st)| {
            if i != imax && i != imin {
                acc + *st
            } else {
                acc
            }
        });
        Some(sum / (len - 2))
    }
}

/// A solve.
#[derive(Debug, Clone)]
pub struct SolveData {
    /// The recorded time of the solve.
    pub time: SolveTime,
    /// The timestamp of when the solve was being recorded.
    pub timestamp: SystemTime,
    /// The scramble used in the solve.
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

    fn test_mean(solves: &[SolveTime], expected: SolveTime) {
        let mean = solves.mean_of_n().unwrap();
        assert!(mean.eq_aprrox(&expected, 10))
    }

    #[test]
    fn calculate_mean() {
        test_mean(
            &[
                SolveTime::new(Duration::from_millis(15_900), None),
                SolveTime::new(Duration::from_millis(12_530), None),
                SolveTime::new(Duration::from_millis(13_080), None),
            ],
            SolveTime::new(Duration::from_millis(13_840), None),
        );
        test_mean(
            &[
                SolveTime::new(Duration::from_millis(15_900), None),
                SolveTime::new(Duration::from_millis(12_530), Some(Penalty::Plus2)),
                SolveTime::new(Duration::from_millis(13_080), None),
            ],
            SolveTime::new(Duration::from_millis(14_500), None),
        );
        test_mean(
            &[
                SolveTime::new(Duration::from_millis(15_900), None),
                SolveTime::new(Duration::from_millis(12_530), Some(Penalty::Dnf)),
                SolveTime::new(Duration::from_millis(13_080), None),
            ],
            SolveTime::DNF,
        );
    }

    fn test_average(solves: &[SolveTime], expected: SolveTime) {
        let average = solves.average_of_n().unwrap();
        assert!(average.eq_aprrox(&expected, 10))
    }

    #[test]
    fn calculate_averages() {
        test_average(
            &[
                SolveTime::new(Duration::from_millis(13_440), None),
                SolveTime::new(Duration::from_millis(14_320), None),
                SolveTime::new(Duration::from_millis(15_900), None),
                SolveTime::new(Duration::from_millis(12_530), None),
                SolveTime::new(Duration::from_millis(13_080), None),
            ],
            SolveTime::new(Duration::from_millis(13_610), None),
        );
        test_average(
            &[
                SolveTime::new(Duration::from_millis(13_440), None),
                SolveTime::new(Duration::from_millis(14_320), None),
                SolveTime::new(Duration::from_millis(15_900), None),
                SolveTime::new(Duration::from_millis(12_530), None),
                SolveTime::new(Duration::from_millis(13_080), Some(Penalty::Plus2)),
            ],
            SolveTime::new(Duration::from_millis(14_280), None),
        );
        test_average(
            &[
                SolveTime::new(Duration::from_millis(13_440), None),
                SolveTime::new(Duration::from_millis(14_320), Some(Penalty::Plus2)),
                SolveTime::new(Duration::from_millis(15_900), None),
                SolveTime::new(Duration::from_millis(12_530), None),
                SolveTime::new(Duration::from_millis(13_080), None),
            ],
            SolveTime::new(Duration::from_millis(14_140), None),
        );
        test_average(
            &[
                SolveTime::new(Duration::from_millis(13_440), None),
                SolveTime::new(Duration::from_millis(14_320), Some(Penalty::Dnf)),
                SolveTime::new(Duration::from_millis(15_900), None),
                SolveTime::new(Duration::from_millis(12_530), Some(Penalty::Dnf)),
                SolveTime::new(Duration::from_millis(13_080), None),
            ],
            SolveTime::DNF,
        );
    }
}
