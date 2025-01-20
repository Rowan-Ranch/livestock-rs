use inflector::Inflector;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

/// An enum representing the different breeds of swine.
///
/// Initial data from: <https://breeds.okstate.edu/swine/>
///
/// # Examples
/// ``` rust
/// use livestock_rs::swine::SwineBreed;
///
/// let breed = SwineBreed::Kunekune;
/// println!("{:?}", breed);
/// ```
#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum SwineBreed {
    AmericanLandrace,
    AmericanYorkshire,
    AngelnSaddleback,
    ArapawaIsland,
    BaXuyen,
    Bantu,
    BeijingBlack,
    BelarusBlackPied,
    BelgianLandrace,
    Berkshire,
    BritishLandrace,
    BritishLop,
    BulgarianWhite,
    Cantonese,
    ChesterWhite,
    ChoctawHog,
    CzechImprovedWhite,
    DanishLandrace,
    Duroc,
    DutchLandrace,
    Fengjing,
    FinnishLandrace,
    FrenchLandrace,
    GermanLandrace,
    GloucestershireOldSpot,
    GuineaHog,
    Hampshire,
    Hereford,
    Hezuo,
    Iberian,
    ItalianLandrace,
    Jinhua,
    Kele,
    Krskopolje,
    Kunekune,
    Lacombe,
    LargeBlack,
    LargeBlackWhite,
    LargeWhite,
    Lithuanian,
    Mangalitza,
    Meishan,
    MiddleWhite,
    MongCai,
    Minzhu,
    MoraRomagnola,
    Mukota,
    Mulefoot,
    Neijiang,
    Ningxiang,
    NorwegianLandrace,
    OssabawIsland,
    OxfordSandyAndBlack,
    PhilippineNative,
    Pietrain,
    PolandChina,
    RedWattle,
    Saddleback,
    Spotted,
    SwedishLandrace,
    Tamworth,
    ThuocNhieu,
    Tibetan,
    Thuropolje,
    VietnamesePotbelly,
    Welsh,
    Wuzhishan,
    Yorkshire
}


impl ToString for SwineBreed {
    /// Converts the SwineBreed enum to a human readable string.
    ///
    /// # Examples
    /// ``` rust
    /// use livestock_rs::swine::SwineBreed;
    ///
    /// let kune = SwineBreed::Kunekune;
    /// println!("{}", kune.to_string());
    /// ```
    fn to_string(&self) -> String {
        match self {
            SwineBreed::LargeBlackWhite => "Large Black-White".to_string(),
            _ => format!("{:?}", self).to_title_case(),
        }
    }
}

/// Converts a string to a SwineBreed enum.
///
/// # Examples
/// ``` rust
/// use livestock_rs::swine::SwineBreed;
/// use std::str::FromStr;
///
/// let breed = SwineBreed::from_str("kunekune").unwrap();
/// println!("{:?}", breed);
/// ```

impl FromStr for SwineBreed {
    type Err = std::io::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "american landrace" => Ok(SwineBreed::AmericanLandrace),
            "american yorkshire" => Ok(SwineBreed::AmericanYorkshire),
            "angeln saddleback" => Ok(SwineBreed::AngelnSaddleback),
            "arapawa island" | "arapawa" => Ok(SwineBreed::ArapawaIsland),
            "ba xuyen" => Ok(SwineBreed::BaXuyen),
            "bantu" => Ok(SwineBreed::Bantu),
            "beijing black" => Ok(SwineBreed::BeijingBlack),
            "belarus black pied" => Ok(SwineBreed::BelarusBlackPied),
            "belgian landrace" => Ok(SwineBreed::BelgianLandrace),
            "berkshire" => Ok(SwineBreed::Berkshire),
            "british landrace" => Ok(SwineBreed::BritishLandrace),
            "british lop" => Ok(SwineBreed::BritishLop),
            "bulgarian white" => Ok(SwineBreed::BulgarianWhite),
            "cantonese" => Ok(SwineBreed::Cantonese),
            "chester white" => Ok(SwineBreed::ChesterWhite),
            "choctaw hog" => Ok(SwineBreed::ChoctawHog),
            "czech improved white" => Ok(SwineBreed::CzechImprovedWhite),
            "danish landrace" => Ok(SwineBreed::DanishLandrace),
            "duroc" => Ok(SwineBreed::Duroc),
            "dutch landrace" => Ok(SwineBreed::DutchLandrace),
            "fengjing" => Ok(SwineBreed::Fengjing),
            "finnish landrace" => Ok(SwineBreed::FinnishLandrace),
            "french landrace" => Ok(SwineBreed::FrenchLandrace),
            "german landrace" => Ok(SwineBreed::GermanLandrace),
            "gloucestershire old spot" => Ok(SwineBreed::GloucestershireOldSpot),
            "guinea hog" => Ok(SwineBreed::GuineaHog),
            "hampshire" => Ok(SwineBreed::Hampshire),
            "hereford" => Ok(SwineBreed::Hereford),
            "hezuo" => Ok(SwineBreed::Hezuo),
            "iberian" => Ok(SwineBreed::Iberian),
            "italian landrace" => Ok(SwineBreed::ItalianLandrace),
            "jinhua" => Ok(SwineBreed::Jinhua),
            "kele" => Ok(SwineBreed::Kele),
            "krskopolje" => Ok(SwineBreed::Krskopolje),
            "kunekune" => Ok(SwineBreed::Kunekune),
            "lacombe" => Ok(SwineBreed::Lacombe),
            "large black" => Ok(SwineBreed::LargeBlack),
            "large black-white" | "large black white"=> Ok(SwineBreed::LargeBlackWhite),
            "large white" => Ok(SwineBreed::LargeWhite),
            "lithuanian" => Ok(SwineBreed::Lithuanian),
            "mangalitza" => Ok(SwineBreed::Mangalitza),
            "meishan" => Ok(SwineBreed::Meishan),
            "middle white" => Ok(SwineBreed::MiddleWhite),
            "mong cai" => Ok(SwineBreed::MongCai),
            "minzhu" => Ok(SwineBreed::Minzhu),
            "mora romagnola" => Ok(SwineBreed::MoraRomagnola),
            "mukota" => Ok(SwineBreed::Mukota),
            "mulefoot" => Ok(SwineBreed::Mulefoot),
            "neijiang" => Ok(SwineBreed::Neijiang),
            "ningxiang" => Ok(SwineBreed::Ningxiang),
            "norwegian landrace" => Ok(SwineBreed::NorwegianLandrace),
            "ossabaw island" => Ok(SwineBreed::OssabawIsland),
            "oxford sandy and black" | "oxford sandy & black"=> Ok(SwineBreed::OxfordSandyAndBlack),
            "philippine native" => Ok(SwineBreed::PhilippineNative),
            "pietrain" => Ok(SwineBreed::Pietrain),
            "poland china" => Ok(SwineBreed::PolandChina),
            "red wattle" => Ok(SwineBreed::RedWattle),
            "saddleback" => Ok(SwineBreed::Saddleback),
            "spotted" => Ok(SwineBreed::Spotted),
            "swedish landrace" => Ok(SwineBreed::SwedishLandrace),
            "tamworth" => Ok(SwineBreed::Tamworth),
            "thuoc nhieu" => Ok(SwineBreed::ThuocNhieu),
            "tibetan" => Ok(SwineBreed::Tibetan),
            "thuropolje" => Ok(SwineBreed::Thuropolje),
            "vietnamese potbelly" | "potbelly" | "potbellied" | "vietnamese potbellied" => Ok(SwineBreed::VietnamesePotbelly),
            "welsh" => Ok(SwineBreed::Welsh),
            "wuzhishan" => Ok(SwineBreed::Wuzhishan),
            "yorkshire" => Ok(SwineBreed::Yorkshire),
            _ => Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Invalid reindeer breed")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_string() {
        // We don't have many special cases, just test a few.
        let breeds = [
            (SwineBreed::LargeBlackWhite, "Large Black-White"),
            (SwineBreed::AmericanLandrace, "American Landrace"),
            (SwineBreed::AmericanYorkshire, "American Yorkshire"),
            (SwineBreed::AngelnSaddleback, "Angeln Saddleback"),
            (SwineBreed::ArapawaIsland, "Arapawa Island"),
            (SwineBreed::BaXuyen, "Ba Xuyen"),
            (SwineBreed::Bantu, "Bantu"),
            (SwineBreed::BeijingBlack, "Beijing Black"),
            (SwineBreed::BelarusBlackPied, "Belarus Black Pied"),
            (SwineBreed::BelgianLandrace, "Belgian Landrace"),
            (SwineBreed::Berkshire, "Berkshire"),
            (SwineBreed::BritishLandrace, "British Landrace"),
            (SwineBreed::BritishLop, "British Lop"),
            (SwineBreed::BulgarianWhite, "Bulgarian White"),
            (SwineBreed::Cantonese, "Cantonese"),
            (SwineBreed::ChesterWhite, "Chester White"),
            (SwineBreed::ChoctawHog, "Choctaw Hog"),
            (SwineBreed::CzechImprovedWhite, "Czech Improved White"),
            (SwineBreed::DanishLandrace, "Danish Landrace"),
            (SwineBreed::Duroc, "Duroc"),
            (SwineBreed::DutchLandrace, "Dutch Landrace"),
            (SwineBreed::Fengjing, "Fengjing"),
            (SwineBreed::FinnishLandrace, "Finnish Landrace"),
            (SwineBreed::FrenchLandrace, "French Landrace"),
            (SwineBreed::GermanLandrace, "German Landrace"),
            (SwineBreed::GloucestershireOldSpot, "Gloucestershire Old Spot"),
            (SwineBreed::GuineaHog, "Guinea Hog"),
            (SwineBreed::Hampshire, "Hampshire"),
            (SwineBreed::Hereford, "Hereford")
        ];

        for (breed, expected) in breeds.iter() {
            assert_eq!(breed.to_string(), *expected);
        }
    }

    #[test]
    fn test_from_str() {
       // We don't have many special cases, just test a few.
       let breeds = [
            ("american landrace", SwineBreed::AmericanLandrace),
            ("american yorkshire", SwineBreed::AmericanYorkshire),
            ("angeln saddleback", SwineBreed::AngelnSaddleback),
            ("arapawa island", SwineBreed::ArapawaIsland),
            ("ba xuyen", SwineBreed::BaXuyen),
            ("potbelly", SwineBreed::VietnamesePotbelly),
            ("potbellied", SwineBreed::VietnamesePotbelly),
            ("vietnamese potbelly", SwineBreed::VietnamesePotbelly),
            ("vietnamese potbellied", SwineBreed::VietnamesePotbelly),
            ("oxford sandy and black", SwineBreed::OxfordSandyAndBlack),
            ("oxford sandy & black", SwineBreed::OxfordSandyAndBlack),
            ("large black-white", SwineBreed::LargeBlackWhite),
            ("large black white", SwineBreed::LargeBlackWhite)
       ];

         for (breed, expected) in breeds.iter() {
              assert_eq!(SwineBreed::from_str(*breed).unwrap(), *expected);
         }
    }
}
