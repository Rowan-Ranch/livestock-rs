use serde::{Deserialize, Serialize};
use std::str::FromStr;

/// An enum representing the different breeds of reindeer.
///
/// Initial data from: <https://breeds.okstate.edu/other-breeds-of-livestock/reindeer/>
///
/// # Examples
/// ``` rust
/// use livestock_rs::breeds::reindeer::ReindeerBreed;
///
/// let breed = ReindeerBreed::Chukotka;
/// println!("{:?}", breed);
/// ```
#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum ReindeerBreed {
    Chukotka,
    Even,
    Evenk,
    Nentsi,
}

impl ToString for ReindeerBreed {
    /// Converts the ReindeerBreed enum to a human readable string.
    ///
    /// # Examples
    /// ``` rust
    /// use livestock_rs::breeds::reindeer::ReindeerBreed;
    ///
    /// let evenk = ReindeerBreed::Evenk;
    /// println!("{}", evenk.to_string());
    /// ```
    fn to_string(&self) -> String {
        match self {
            ReindeerBreed::Chukotka => "Chukotka".to_string(),
            ReindeerBreed::Even => "Even".to_string(),
            ReindeerBreed::Evenk => "Evenk".to_string(),
            ReindeerBreed::Nentsi => "Nentsi".to_string(),
        }
    }
}

/// Converts a string to a ReindeerBreed enum.
///
/// # Examples
/// ``` rust
/// use livestock_rs::breeds::reindeer::ReindeerBreed;
/// use std::str::FromStr;
///
/// let breed = ReindeerBreed::from_str("evenk").unwrap();
/// println!("{:?}", breed);
/// ```

impl FromStr for ReindeerBreed {
    type Err = std::io::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "chukotka" => Ok(ReindeerBreed::Chukotka),
            "even" => Ok(ReindeerBreed::Even),
            "evenk" => Ok(ReindeerBreed::Evenk),
            "nentsi" => Ok(ReindeerBreed::Nentsi),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Invalid reindeer breed",
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_string() {
        let breeds = [
            (ReindeerBreed::Chukotka, "Chukotka"),
            (ReindeerBreed::Even, "Even"),
            (ReindeerBreed::Evenk, "Evenk"),
            (ReindeerBreed::Nentsi, "Nentsi"),
        ];

        for (breed, expected) in breeds.iter() {
            assert_eq!(breed.to_string(), *expected);
        }
    }

    #[test]
    fn test_from_str() {
        let breeds = [
            ("chukotka", ReindeerBreed::Chukotka),
            ("even", ReindeerBreed::Even),
            ("evenk", ReindeerBreed::Evenk),
            ("nentsi", ReindeerBreed::Nentsi),
        ];

        for (breed, expected) in breeds.iter() {
            assert_eq!(ReindeerBreed::from_str(*breed).unwrap(), *expected);
        }
    }
}
