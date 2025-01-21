use inflector::Inflector;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

/// An enum representing the different breeds of camel.
///
/// Initial data from: <https://breeds.okstate.edu/other-breeds-of-livestock/camels/>
///
/// # Examples
/// ``` rust
/// use livestock_rs::breeds::camel::CamelBreed;
///
/// let breed = CamelBreed::AfarDromedary;
/// println!("{:?}", breed);
/// ```
#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum CamelBreed {
    AfarDromedary,
    AlxaBactrian,
    ArvanaDromedary,
    KalmykBactrian,
    SomaliDromedary
}

impl ToString for CamelBreed {
    /// Converts the CamelBreed enum to a human readable string.
    ///
    /// # Examples
    /// ``` rust
    /// use livestock_rs::breeds::camel::CamelBreed;
    ///
    /// let alxa = CamelBreed::AlxaBactrian;
    /// println!("{}", alxa.to_string());
    /// ```
    fn to_string(&self) -> String {
        format!("{:?}", self).to_title_case()
    }
}

/// Converts a string to a CamelBreed enum.
///
/// # Examples
/// ``` rust
/// use livestock_rs::breeds::camel::CamelBreed;
/// use std::str::FromStr;
///
/// let breed = CamelBreed::from_str("Somali Dromedary").unwrap();
/// println!("{:?}", breed);
/// ```

impl FromStr for CamelBreed {
    type Err = std::io::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "afar dromedary" | "afar" => Ok(CamelBreed::AfarDromedary),
            "alxa bactrian" | "alxa" => Ok(CamelBreed::AlxaBactrian),
            "arvana dromedary" | "arvana" => Ok(CamelBreed::ArvanaDromedary),
            "kalmyk bactrian" | "kalmyk" => Ok(CamelBreed::KalmykBactrian),
            "somali dromedary" | "somali" => Ok(CamelBreed::SomaliDromedary),
            _ => Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Invalid camel breed")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_string() {
        let breeds = [
            (CamelBreed::AfarDromedary, "Afar Dromedary"),
            (CamelBreed::AlxaBactrian, "Alxa Bactrian"),
            (CamelBreed::ArvanaDromedary, "Arvana Dromedary"),
            (CamelBreed::KalmykBactrian, "Kalmyk Bactrian"),
            (CamelBreed::SomaliDromedary, "Somali Dromedary"),
        ];

        for (breed, expected) in breeds.iter() {
            assert_eq!(breed.to_string(), *expected);
        }
    }

    #[test]
    fn test_from_str() {
        let breeds = [
            ("afar dromedary", CamelBreed::AfarDromedary),
            ("alxa bactrian", CamelBreed::AlxaBactrian),
            ("arvana dromedary", CamelBreed::ArvanaDromedary),
            ("kalmyk bactrian", CamelBreed::KalmykBactrian),
            ("somali dromedary", CamelBreed::SomaliDromedary),
        ];

        for (breed, expected) in breeds.iter() {
            assert_eq!(CamelBreed::from_str(*breed).unwrap(), *expected);
        }
    }
}
