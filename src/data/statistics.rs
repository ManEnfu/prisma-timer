use crate::data::SolveTime;

/// A trait to retrieve statistics for a sequence of solves.
pub trait SolveStatistic {
    /// Calculates the mean time of the solves.
    fn mean_of_n(&self) -> Option<SolveTime>;
    /// Calculates the average time of the solves. This is similar to
    /// `mean_of_n`, but the fastest and slowest solves are excluded
    /// from the calculation.
    fn average_of_n(&self) -> Option<SolveTime>;
    /// Gets the index of the best solve.
    fn best_solve_index(&self) -> Option<usize>;
    /// Gets the index of the worst solve.
    fn worst_solve_index(&self) -> Option<usize>;
}

impl<T> SolveStatistic for [T]
where
    for<'a> &'a T: Into<SolveTime>,
{
    fn mean_of_n(&self) -> Option<SolveTime> {
        let len = self.len() as u32;
        if len == 0 {
            return None;
        }

        let sum: SolveTime = self.iter().map(Into::<SolveTime>::into).sum();
        Some(sum / len)
    }

    fn average_of_n(&self) -> Option<SolveTime> {
        let len = self.len() as u32;
        if len < 3 {
            return None;
        }

        let it = self.iter().map(Into::<SolveTime>::into).enumerate();
        let (imax, _max) = it.clone().max_by_key(|&(_, time)| time)?;
        let (imin, _min) = it.clone().min_by_key(|&(_, time)| time)?;
        let sum = it.fold(SolveTime::default(), |acc, (i, time)| {
            if i != imax && i != imin {
                acc + time
            } else {
                acc
            }
        });
        Some(sum / (len - 2))
    }

    fn best_solve_index(&self) -> Option<usize> {
        self.iter()
            .map(Into::<SolveTime>::into)
            .enumerate()
            .min_by_key(|&(_, time)| time)
            .map(|(i, _)| i)
    }

    fn worst_solve_index(&self) -> Option<usize> {
        self.iter()
            .map(Into::<SolveTime>::into)
            .enumerate()
            .max_by_key(|&(_, time)| time)
            .map(|(i, _)| i)
    }
}

#[cfg(test)]
mod tests {
    use crate::data::Penalty;

    use super::*;
    use std::time::Duration;

    fn test_mean(solves: &[SolveTime], expected: SolveTime) {
        let mean = solves.mean_of_n().unwrap();
        assert!(mean.eq_aprrox(&expected, 10))
    }

    #[test]
    fn calculate_mean() {
        test_mean(
            &[
                SolveTime::new(Duration::from_millis(15_900), Penalty::Ok),
                SolveTime::new(Duration::from_millis(12_530), Penalty::Ok),
                SolveTime::new(Duration::from_millis(13_080), Penalty::Ok),
            ],
            SolveTime::new(Duration::from_millis(13_840), Penalty::Ok),
        );
    }

    #[test]
    fn calculate_mean_with_plus2() {
        test_mean(
            &[
                SolveTime::new(Duration::from_millis(15_900), Penalty::Ok),
                SolveTime::new(Duration::from_millis(12_530), Penalty::Plus2),
                SolveTime::new(Duration::from_millis(13_080), Penalty::Ok),
            ],
            SolveTime::new(Duration::from_millis(14_500), Penalty::Ok),
        );
    }

    #[test]
    fn calculate_mean_with_dnf() {
        test_mean(
            &[
                SolveTime::new(Duration::from_millis(15_900), Penalty::Ok),
                SolveTime::new(Duration::from_millis(12_530), Penalty::Dnf),
                SolveTime::new(Duration::from_millis(13_080), Penalty::Ok),
            ],
            SolveTime::DNF,
        );
    }

    fn test_average(solves: &[SolveTime], expected: SolveTime) {
        let average = solves.average_of_n().unwrap();
        assert!(average.eq_aprrox(&expected, 10))
    }

    #[test]
    fn calculate_average() {
        test_average(
            &[
                SolveTime::new(Duration::from_millis(13_440), Penalty::Ok),
                SolveTime::new(Duration::from_millis(14_320), Penalty::Ok),
                SolveTime::new(Duration::from_millis(15_900), Penalty::Ok),
                SolveTime::new(Duration::from_millis(12_530), Penalty::Ok),
                SolveTime::new(Duration::from_millis(13_080), Penalty::Ok),
            ],
            SolveTime::new(Duration::from_millis(13_610), Penalty::Ok),
        );
    }

    #[test]
    fn calculate_average_with_plus2() {
        test_average(
            &[
                SolveTime::new(Duration::from_millis(13_440), Penalty::Ok),
                SolveTime::new(Duration::from_millis(14_320), Penalty::Ok),
                SolveTime::new(Duration::from_millis(15_900), Penalty::Ok),
                SolveTime::new(Duration::from_millis(12_530), Penalty::Ok),
                SolveTime::new(Duration::from_millis(13_080), Penalty::Plus2),
            ],
            SolveTime::new(Duration::from_millis(14_280), Penalty::Ok),
        );
    }

    #[test]
    fn calculate_average_with_plus2_2() {
        test_average(
            &[
                SolveTime::new(Duration::from_millis(13_440), Penalty::Ok),
                SolveTime::new(Duration::from_millis(14_320), Penalty::Plus2),
                SolveTime::new(Duration::from_millis(15_900), Penalty::Ok),
                SolveTime::new(Duration::from_millis(12_530), Penalty::Ok),
                SolveTime::new(Duration::from_millis(13_080), Penalty::Ok),
            ],
            SolveTime::new(Duration::from_millis(14_140), Penalty::Ok),
        );
    }

    #[test]
    fn calculate_average_with_dnf() {
        test_average(
            &[
                SolveTime::new(Duration::from_millis(13_440), Penalty::Ok),
                SolveTime::new(Duration::from_millis(14_320), Penalty::Dnf),
                SolveTime::new(Duration::from_millis(15_900), Penalty::Ok),
                SolveTime::new(Duration::from_millis(12_530), Penalty::Ok),
                SolveTime::new(Duration::from_millis(13_080), Penalty::Ok),
            ],
            SolveTime::new(Duration::from_millis(14_140), Penalty::Ok),
        );
    }

    #[test]
    fn calculate_average_with_two_dnf() {
        test_average(
            &[
                SolveTime::new(Duration::from_millis(13_440), Penalty::Ok),
                SolveTime::new(Duration::from_millis(14_320), Penalty::Dnf),
                SolveTime::new(Duration::from_millis(15_900), Penalty::Ok),
                SolveTime::new(Duration::from_millis(12_530), Penalty::Dnf),
                SolveTime::new(Duration::from_millis(13_080), Penalty::Ok),
            ],
            SolveTime::DNF,
        );
    }
}
