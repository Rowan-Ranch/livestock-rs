use serde::{Deserialize, Serialize};
use std::str::FromStr;

/// An enum representing the different breeds of cattle.
///
/// Initial data from: <https://breeds.okstate.edu/cattle/>
///
/// # Examples
/// ``` rust
/// use livestock_rs::cattle::CattleBreed;
///
/// let breed = CattleBreed::Angus;
/// println!("{:?}", breed);
/// ```
#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum CattleBreed {
    Angus
}

impl ToString for CattleBreed {
    /// Converts the CattleBreed enum to a human readable string.
    ///
    /// # Examples
    /// ``` rust
    /// use livestock_rs::cattle::CattleBreed;
    ///
    /// let angus = CattleBreed::Angus;
    /// println!("{}", angus.to_string());
    /// ```
    fn to_string(&self) -> String {
        match self {
            CattleBreed::Angus => "Angus".to_string(),
        }
    }
}

/// Converts a string to a CattleBreed enum.
///
/// # Examples
/// ``` rust
/// use livestock_rs::cattle::CattleBreed;
/// use std::str::FromStr;
///
/// let breed = CattleBreed::from_str("angus").unwrap();
/// println!("{:?}", breed);
/// ```

impl FromStr for CattleBreed {
    type Err = std::io::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "angus" => Ok(CattleBreed::Angus),
            _ => Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Invalid cattle breed")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_string() {
        let breeds = [
            (CattleBreed::Angus, "Angus"),
        ];

        for (breed, expected) in breeds.iter() {
            assert_eq!(breed.to_string(), *expected);
        }
    }

    #[test]
    fn test_from_str() {
        let breeds = [
            ("angus", CattleBreed::Angus),
            ("Angus", CattleBreed::Angus),
        ];

        for (breed, expected) in breeds.iter() {
            assert_eq!(CattleBreed::from_str(*breed).unwrap(), *expected);
        }
    }
}
