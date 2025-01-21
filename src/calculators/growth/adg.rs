use anyhow::{ensure, Result};

/// Calculate Average Daily Gain (ADG) for livestock.
///
/// # Arguments
/// - `initial_weight`: Starting weight of the animal (kg or lbs).
/// - `final_weight`: Ending weight of the animal (kg or lbs).
/// - `days`: The number of days in the measurement period.
///
/// # Returns
/// The average daily gain (ADG) in the same unit as the weight.
///
/// # Example
/// ```
/// use livestock_rs::calculators::growth::adg::calculate_adg;
/// let adg = calculate_adg(100.0, 150.0, 50);
/// assert_eq!(adg, 1.0); // 1 kg or lb per day
/// ```
pub fn calculate_adg(initial_weight: f64, final_weight: f64, days: usize) -> Result<f64> {
    ensure!(days > 0, "Number of days cannot be zero.");
    ensure!(
        final_weight > initial_weight,
        "Final weight must be greater than initial weight."
    );
    ensure!(
        final_weight > 0.0 && initial_weight > 0.0,
        "Weights must be greater than zero."
    );

    Ok((final_weight - initial_weight) / days as f64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_adg() {
        let adg_test_cases = [
            (100.0, 150.0, 50, 1.0),
            (100.0, 200.0, 100, 1.0),
            (100.0, 200.0, 50, 2.0),
            (100.0, 150.0, 100, 0.5),
        ];

        for (initial_weight, final_weight, days, expected) in adg_test_cases.iter() {
            let result = calculate_adg(*initial_weight, *final_weight, *days);
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), *expected);
        }
    }

    #[test]
    fn test_calculate_adg_zero_days() {
        let result = calculate_adg(100.0, 150.0, 0);
        assert!(result.is_err());
    }

    #[test]
    fn test_calculate_adg_negative_weight() {
        let result = calculate_adg(-100.0, 150.0, 50);
        assert!(result.is_err());
    }

    #[test]
    fn test_calculate_adg_negative_final_weight() {
        let result = calculate_adg(100.0, -150.0, 50);
        assert!(result.is_err());
    }

    #[test]
    fn test_calculate_adg_negative_initial_weight() {
        let result = calculate_adg(-100.0, 150.0, 50);
        assert!(result.is_err());
    }
}