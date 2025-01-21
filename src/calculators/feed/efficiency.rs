use anyhow::{ensure, Result};
use crate::types::LivestockType;

#[derive(Debug, Eq, PartialEq)]
pub enum FeedEfficiencyRating {
    Poor,
    Average,
    Good,
}

#[derive(Debug, PartialEq)]
pub struct FeedEfficiency {
    pub rating: FeedEfficiencyRating,
    pub value: f64,
    pub avg_min_fcr: f64,
    pub avg_max_fcr: f64
}

/// Calculate Feed Efficiency for livestock.
/// 
/// # Arguments
/// - `fcr`: Feed Conversion Ratio (FCR) of the animal.
/// - `livestock_type`: The type of livestock.
/// 
/// # Returns
/// The feed efficiency of the animal.
/// 
/// # Example
/// ```
/// use livestock_rs::calculators::feed::efficiency::{calculate_feed_efficiency, FeedEfficiency, FeedEfficiencyRating};
/// use livestock_rs::types::LivestockType;
/// 
/// let fcr = 10.0;
/// let livestock_type = LivestockType::Cattle;
/// 
/// let feed_efficiency = calculate_feed_efficiency(fcr, livestock_type).unwrap();
/// 
/// // The feed efficiency is average for a FCR of 10.0 for cattle.
/// assert_eq!(feed_efficiency.rating, FeedEfficiencyRating::Average);
/// 
/// ```
/// 
pub fn calculate_feed_efficiency(fcr: f64, livestock_type: LivestockType) -> Result<FeedEfficiency> {
    ensure!(fcr > 0.0, "Feed Conversion Ratio (FCR) must be greater than 0.");

    // Feed efficiency values for different animals.
    // 
    // Source: <https://www.farmbrite.com/post/feed-conversion-ratio-calculator>
    let (min_fcr, max_fcr) = match livestock_type {
        LivestockType::Cattle => (8.0, 12.0),
        LivestockType::Goat | LivestockType::Sheep => (4.5, 5.5),
        LivestockType::Swine => (3.0, 3.9),
        LivestockType::Chicken => (1.5, 2.0),
        LivestockType::Rabbit => (3.5, 5.0),
    };

    let feed_efficiency = if fcr > max_fcr {
        FeedEfficiency {
            rating: FeedEfficiencyRating::Poor,
            value: fcr,
            avg_min_fcr: min_fcr,
            avg_max_fcr: max_fcr
        }
    } else if fcr < min_fcr {
        FeedEfficiency {
            rating: FeedEfficiencyRating::Good,
            value: fcr,
            avg_min_fcr: min_fcr,
            avg_max_fcr: max_fcr
        }
    } else {
        FeedEfficiency {
            rating: FeedEfficiencyRating::Average,
            value: fcr,
            avg_min_fcr: min_fcr,
            avg_max_fcr: max_fcr
        }
    };

    Ok(feed_efficiency)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::LivestockType;

    #[test]
    fn test_calculate_feed_efficiency() {
        let feed_efficiency_test_cases = [
            (10.0, LivestockType::Cattle, FeedEfficiencyRating::Average),
            (4.0, LivestockType::Cattle, FeedEfficiencyRating::Good),
            (2.0, LivestockType::Cattle, FeedEfficiencyRating::Good),
            (15.0, LivestockType::Cattle, FeedEfficiencyRating::Poor),
            (5.0, LivestockType::Goat, FeedEfficiencyRating::Average),
            (4.5, LivestockType::Goat, FeedEfficiencyRating::Average),
            (5.5, LivestockType::Goat, FeedEfficiencyRating::Average),
            (3.0, LivestockType::Swine, FeedEfficiencyRating::Average),
            (3.9, LivestockType::Swine, FeedEfficiencyRating::Average),
            (1.5, LivestockType::Chicken, FeedEfficiencyRating::Average),
            (2.0, LivestockType::Chicken, FeedEfficiencyRating::Average),
            (3.5, LivestockType::Rabbit, FeedEfficiencyRating::Average),
            (5.0, LivestockType::Rabbit, FeedEfficiencyRating::Average),
        ];

        for (fcr, livestock_type, expected) in feed_efficiency_test_cases.iter() {
            let result = calculate_feed_efficiency(*fcr, livestock_type.clone());
            assert!(result.is_ok());
            assert_eq!(result.unwrap().rating, *expected);
        }
    }

    #[test]
    fn test_calculate_feed_efficiency_zero_fcr() {
        let result = calculate_feed_efficiency(0.0, LivestockType::Cattle);
        assert!(result.is_err());
    }
}