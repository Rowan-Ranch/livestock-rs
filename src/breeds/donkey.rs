use inflector::Inflector;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

/// An enum representing the different breeds of donkeys.
///
/// Initial data from: <https://breeds.okstate.edu/other-breeds-of-livestock/donkeys/>
///
/// # Examples
/// ``` rust
/// use livestock_rs::breeds::donkey::DonkeyBreed;
///
/// let breed = DonkeyBreed::Abyssinian;
/// println!("{:?}", breed);
/// ```
#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum DonkeyBreed {
    Abyssinian,
    Anatolia,
    LargeStandard,
    MammothJackStock,
    Mary,
    Miniature,
    Poitou,
    Standard
}

impl ToString for DonkeyBreed {
    /// Converts the DonkeyBreed enum to a human readable string.
    ///
    /// # Examples
    /// ``` rust
    /// use livestock_rs::breeds::donkey::DonkeyBreed;
    ///
    /// let evenk = DonkeyBreed::Standard;
    /// println!("{}", evenk.to_string());
    /// ```
    fn to_string(&self) -> String {
        format!("{:?}", self).to_title_case()
    }
}

/// Converts a string to a DonkeyBreed enum.
///
/// # Examples
/// ``` rust
/// use livestock_rs::breeds::donkey::DonkeyBreed;
/// use std::str::FromStr;
///
/// let breed = DonkeyBreed::from_str("standard").unwrap();
/// println!("{:?}", breed);
/// ```

impl FromStr for DonkeyBreed {
    type Err = std::io::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "abyssinian" => Ok(DonkeyBreed::Abyssinian),
            "anatolia" => Ok(DonkeyBreed::Anatolia),
            "large standard" => Ok(DonkeyBreed::LargeStandard),
            "mammoth jack stock" => Ok(DonkeyBreed::MammothJackStock),
            "mary" => Ok(DonkeyBreed::Mary),
            "miniature" | "mini" => Ok(DonkeyBreed::Miniature),
            "poitou" => Ok(DonkeyBreed::Poitou),
            "standard" => Ok(DonkeyBreed::Standard),
            _ => Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Invalid reindeer breed")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_string() {
        let breeds = [
            (DonkeyBreed::Abyssinian, "Abyssinian"),
            (DonkeyBreed::Anatolia, "Anatolia"),
            (DonkeyBreed::LargeStandard, "Large Standard"),
            (DonkeyBreed::MammothJackStock, "Mammoth Jack Stock"),
            (DonkeyBreed::Mary, "Mary"),
            (DonkeyBreed::Miniature, "Miniature"),
            (DonkeyBreed::Poitou, "Poitou"),
            (DonkeyBreed::Standard, "Standard"),
        ];

        for (breed, expected) in breeds.iter() {
            assert_eq!(breed.to_string(), *expected);
        }
    }

    #[test]
    fn test_from_str() {
        let breeds = [
            ("abyssinian", DonkeyBreed::Abyssinian),
            ("anatolia", DonkeyBreed::Anatolia),
            ("large standard", DonkeyBreed::LargeStandard),
            ("mammoth jack stock", DonkeyBreed::MammothJackStock),
            ("mary", DonkeyBreed::Mary),
            ("miniature", DonkeyBreed::Miniature),
            ("mini", DonkeyBreed::Miniature),
            ("poitou", DonkeyBreed::Poitou),
            ("standard", DonkeyBreed::Standard)
        ];

        for (breed, expected) in breeds.iter() {
            assert_eq!(DonkeyBreed::from_str(*breed).unwrap(), *expected);
        }
    }
}
