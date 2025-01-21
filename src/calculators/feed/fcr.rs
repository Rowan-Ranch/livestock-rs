use anyhow::{ensure, Result};

/// Calculate Feed Conversion Ratio (FCR)
/// 
/// # Arguments
/// - `feed_intake`: Amount of feed consumed by the animal (in lb or kg).
/// - `weight_gain`: Weight gain of the animal (in lb or kg).
/// 
/// # Returns
/// The feed conversion ratio (FCR) as a ratio of feed intake to weight gain.
/// 
/// # Example
/// ```
/// use livestock_rs::calculators::feed::fcr::calculate_fcr;
/// let fcr = calculate_fcr(100.0, 300.0).unwrap();
/// assert_eq!(fcr,  0.3333333333333333); // 0.33 <unit> of feed per kg of weight gain where <unit> is lb or kg.
/// ```
/// 
/// # Notes
/// - A lower FCR indicates better feed efficiency.
/// - FCR is calculated as `feed_intake / weight_gain`.
pub fn calculate_fcr(feed_intake: f64, weight_gain: f64) -> Result<f64> {
    ensure!(feed_intake > 0.0, "Feed intake must be greater than 0.");
    ensure!(weight_gain > 0.0, "Weight gain must be greater than 0.");

    Ok(feed_intake / weight_gain)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_fcr() {
        let fcr_test_cases = [
            (100.0, 300.0, 0.3333333333333333),
            (100.0, 200.0, 0.5),
            (200.0, 100.0, 2.0),
            (150.0, 100.0, 1.5),
        ];

        for (feed_intake, weight_gain, expected) in fcr_test_cases.iter() {
            let result = calculate_fcr(*feed_intake, *weight_gain);
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), *expected);
        }
    }

    #[test]
    fn test_calculate_fcr_zero_feed_intake() {
        let result = calculate_fcr(0.0, 150.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_calculate_fcr_zero_weight_gain() {
        let result = calculate_fcr(100.0, 0.0);
        assert!(result.is_err());
    }
}