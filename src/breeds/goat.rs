use inflector::Inflector;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

/// An enum representing the different breeds of goats.
///
/// Initial data from: <https://breeds.okstate.edu/goats/>
///
/// # Examples
/// ``` rust
/// use livestock_rs::breeds::goat::GoatBreed;
///
/// let breed = GoatBreed::Alpine;
/// println!("{:?}", breed);
/// ```
#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum GoatBreed {
    Alpine,
    AltaiMountain,
    AngloNubian,
    Angora,
    Appenzell,
    ArapawaIsland,
    Australian,
    Bagot,
    Barbari,
    Beetal,
    BelgianFawn,
    Benadir,
    Bhuj,
    Boer,
    Booted,
    BritishAlpine,
    BrownShorthair,
    CanaryIsland,
    Caninde,
    Cashmere,
    Chapar,
    Corsican,
    DaeraDinPanah,
    Damani,
    DanishLandrace,
    Don,
    DutchLandrace,
    Erzgebirge,
    FinnishLandrace,
    GoldenGuernsey,
    GrisonsStriped,
    Hailun,
    Haimen,
    Hasi,
    Hejazi,
    HexiCashmere,
    Hongtong,
    Huaipi,
    Huaitoutala,
    HungarianImproved,
    Irish,
    JiningGrey,
    Kaghani,
    KalahariRed,
    Kamori,
    Kiko,
    Kinder,
    LaMancha,
    Laoshan,
    Moxoto,
    MurciaGranada,
    Myotonic, // Wooden Leg
    Nachi,
    NigerianDwarf,
    Norwegian,
    Oberhasli,
    Peacock,
    Philippine,
    Poitou,
    Pygmy,
    Pyrenean,
    Qinshan,
    Repartida,
    RussianWhiteAndGorki,
    Saanen,
    Sahelian,
    Savanna,
    SanClemente,
    Somali,
    Spanish,
    SRD,
    SwedishLandrace,
    Thuringian,
    Toggenburg,
    UzbekBlack,
    ValaisBlackneck,
    Verata,
    WestAfricanDwarf,
    WhiteShorthair,
    Xinjiang,
    Xuhai,
    YemenMountain,
    Zhongwei,
}

impl ToString for GoatBreed {
    /// Converts the GoatBreed enum to a human readable string.
    ///
    /// # Examples
    /// ``` rust
    /// use livestock_rs::breeds::goat::GoatBreed;
    ///
    /// let alpine = GoatBreed::Alpine;
    /// println!("{}", alpine.to_string());
    ///
    /// let canindé = GoatBreed::Caninde;
    /// println!("{}", canindé.to_string());
    /// ```
    fn to_string(&self) -> String {
        match self {
            GoatBreed::AngloNubian => "Anglo-Nubian".to_string(),
            GoatBreed::Caninde => "Canindé".to_string(),
            GoatBreed::MurciaGranada => "Murcia-Granada".to_string(),
            GoatBreed::Myotonic => "Myotonic (Wooden Leg)".to_string(),
            _ => format!("{:?}", self).to_title_case(),
        }
    }
}

/// Converts a string to a GoatBreed enum.
///
/// # Examples
/// ``` rust
/// use livestock_rs::breeds::goat::GoatBreed;
/// use std::str::FromStr;
///
/// let breed = GoatBreed::from_str("Alpine").unwrap();
/// println!("{:?}", breed);
///
/// let breed = GoatBreed::from_str("Anglo-Nubian").unwrap();
/// println!("{:?}", breed);
/// ```

impl FromStr for GoatBreed {
    type Err = std::io::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "alpine" => Ok(GoatBreed::Alpine),
            "altai mountain" => Ok(GoatBreed::AltaiMountain),
            "anglo-nubian" | "anglo nubian" => Ok(GoatBreed::AngloNubian),
            "angora" => Ok(GoatBreed::Angora),
            "appenzell" => Ok(GoatBreed::Appenzell),
            "arapawa island" => Ok(GoatBreed::ArapawaIsland),
            "australian" => Ok(GoatBreed::Australian),
            "bagot" => Ok(GoatBreed::Bagot),
            "barbari" => Ok(GoatBreed::Barbari),
            "beetal" => Ok(GoatBreed::Beetal),
            "belgian fawn" => Ok(GoatBreed::BelgianFawn),
            "benadir" => Ok(GoatBreed::Benadir),
            "bhuj" => Ok(GoatBreed::Bhuj),
            "boer" => Ok(GoatBreed::Boer),
            "booted" => Ok(GoatBreed::Booted),
            "british alpine" => Ok(GoatBreed::BritishAlpine),
            "brown shorthair" => Ok(GoatBreed::BrownShorthair),
            "canary island" => Ok(GoatBreed::CanaryIsland),
            "canindé" | "caninde" => Ok(GoatBreed::Caninde),
            "cashmere" => Ok(GoatBreed::Cashmere),
            "chapar" => Ok(GoatBreed::Chapar),
            "corsican" => Ok(GoatBreed::Corsican),
            "daera din panah" => Ok(GoatBreed::DaeraDinPanah),
            "damani" => Ok(GoatBreed::Damani),
            "danish landrace" => Ok(GoatBreed::DanishLandrace),
            "don" => Ok(GoatBreed::Don),
            "dutch landrace" => Ok(GoatBreed::DutchLandrace),
            "erzgebirge" => Ok(GoatBreed::Erzgebirge),
            "finnish landrace" => Ok(GoatBreed::FinnishLandrace),
            "golden guernsey" => Ok(GoatBreed::GoldenGuernsey),
            "grisons striped" => Ok(GoatBreed::GrisonsStriped),
            "hailun" => Ok(GoatBreed::Hailun),
            "haimen" => Ok(GoatBreed::Haimen),
            "hasi" => Ok(GoatBreed::Hasi),
            "hejazi" => Ok(GoatBreed::Hejazi),
            "hexi cashmere" => Ok(GoatBreed::HexiCashmere),
            "hongtong" => Ok(GoatBreed::Hongtong),
            "huaipi" => Ok(GoatBreed::Huaipi),
            "huaitoutala" => Ok(GoatBreed::Huaitoutala),
            "hungarian improved" => Ok(GoatBreed::HungarianImproved),
            "irish" => Ok(GoatBreed::Irish),
            "jining grey" => Ok(GoatBreed::JiningGrey),
            "kaghani" => Ok(GoatBreed::Kaghani),
            "kalahari red" => Ok(GoatBreed::KalahariRed),
            "kamori" => Ok(GoatBreed::Kamori),
            "kiko" => Ok(GoatBreed::Kiko),
            "kinder" => Ok(GoatBreed::Kinder),
            "la mancha" => Ok(GoatBreed::LaMancha),
            "laoshan" => Ok(GoatBreed::Laoshan),
            "moxoto" => Ok(GoatBreed::Moxoto),
            "murcia granada" | "murcia-granada" => Ok(GoatBreed::MurciaGranada),
            "myotonic" | "wooden leg" | "myotonic wooden leg" | "myotonic (wooden leg)" => {
                Ok(GoatBreed::Myotonic)
            }
            "nachi" => Ok(GoatBreed::Nachi),
            "nigerian dwarf" => Ok(GoatBreed::NigerianDwarf),
            "norwegian" => Ok(GoatBreed::Norwegian),
            "oberhasli" => Ok(GoatBreed::Oberhasli),
            "peacock" => Ok(GoatBreed::Peacock),
            "philippine" => Ok(GoatBreed::Philippine),
            "poitou" => Ok(GoatBreed::Poitou),
            "pygmy" => Ok(GoatBreed::Pygmy),
            "pyrenean" => Ok(GoatBreed::Pyrenean),
            "qinshan" => Ok(GoatBreed::Qinshan),
            "repartida" => Ok(GoatBreed::Repartida),
            "russian white and gorki" | "russian white & gorki" => {
                Ok(GoatBreed::RussianWhiteAndGorki)
            }
            "saanen" => Ok(GoatBreed::Saanen),
            "sahelian" => Ok(GoatBreed::Sahelian),
            "savanna" => Ok(GoatBreed::Savanna),
            "san clemente" => Ok(GoatBreed::SanClemente),
            "somali" => Ok(GoatBreed::Somali),
            "spanish" => Ok(GoatBreed::Spanish),
            "srd" => Ok(GoatBreed::SRD),
            "swedish landrace" => Ok(GoatBreed::SwedishLandrace),
            "thuringian" => Ok(GoatBreed::Thuringian),
            "toggenburg" => Ok(GoatBreed::Toggenburg),
            "uzbek black" => Ok(GoatBreed::UzbekBlack),
            "valais blackneck" => Ok(GoatBreed::ValaisBlackneck),
            "verata" => Ok(GoatBreed::Verata),
            "west african dwarf" => Ok(GoatBreed::WestAfricanDwarf),
            "white shorthair" => Ok(GoatBreed::WhiteShorthair),
            "xinjiang" => Ok(GoatBreed::Xinjiang),
            "xuhai" => Ok(GoatBreed::Xuhai),
            "yemen mountain" => Ok(GoatBreed::YemenMountain),
            "zhongwei" => Ok(GoatBreed::Zhongwei),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Unknown breed",
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_string_special_cases() {
        let special_case_breeds = [
            (GoatBreed::Caninde, "Canindé"),
            (GoatBreed::AngloNubian, "Anglo-Nubian"),
            (GoatBreed::MurciaGranada, "Murcia-Granada"),
            (GoatBreed::Myotonic, "Myotonic (Wooden Leg)"),
        ];

        for (breed, expected) in special_case_breeds.iter() {
            assert_eq!(breed.to_string(), *expected);
        }
    }

    #[test]
    fn test_from_string_special_cases() {
        let special_case_breeds = [
            ("Canindé", GoatBreed::Caninde),
            ("Caninde", GoatBreed::Caninde),
            ("Anglo Nubian", GoatBreed::AngloNubian),
            ("Anglo-Nubian", GoatBreed::AngloNubian),
            ("Murcia Granada", GoatBreed::MurciaGranada),
            ("Murcia-Granada", GoatBreed::MurciaGranada),
            ("Myotonic", GoatBreed::Myotonic),
            ("Wooden Leg", GoatBreed::Myotonic),
            ("Myotonic Wooden Leg", GoatBreed::Myotonic),
            ("Myotonic (Wooden Leg)", GoatBreed::Myotonic),
        ];

        for (breed, expected) in special_case_breeds.iter() {
            assert_eq!(GoatBreed::from_str(breed).unwrap(), *expected);
        }
    }
}
