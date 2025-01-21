use serde::{Deserialize, Serialize};
use std::str::FromStr;

/// An enum representing the different breeds of chickens.
///
/// Initial data from: <https://breeds.okstate.edu/poultry/chickens/>
///
/// # Examples
/// ``` rust
/// use livestock_rs::chicken::ChickenBreed;
///
/// let breed = ChickenBreed::Orpington;
/// println!("{:?}", breed);
/// ```
#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum ChickenBreed {
    AC,
    Ameraucana,
    Ancona,
    Andalusian,
    AppenzellBeardedHen,
    AppenzellPointedHoodHen,
    Araucana,
    Aseel,
    Australorp,
    Baheij,
    Bandara,
    Barnevelders,
    Brahma,
    Buckeye,
    Buttercup,
    Campine,
    Catalana,
    Chantecler,
    Cochin,
    Cornish,
    Crevecoeur,
    Cubalaya,
    Delaware,
    Dominiques,
    Dorking,
    DutchBantam,
    Faverolles,
    Friesian,
    Frizzle,
    Gimmizah,
    GoldenMontazah,
    Hamburg,
    Holland,
    Houdan,
    Java,
    JerseyGiant,
    LaFleche,
    Lakenvelder,
    Lamona,
    Langshan,
    Legbar,
    Leghorn,
    Marans,
    Malay,
    Matrouh,
    Minorca,
    ModernGame,
    NakedNeck, // (Turken)
    Nankin,
    NewHampshireRed,
    OldEnglishGame,
    Orpington,
    PlymouthRock,
    Polish,
    RedCap,
    RhodeIslandRed,
    RussianOrloff,
    Sasso,
    Sebright,
    Shamo,
    SilkieBantam,
    SilverMontazah,
    Styrian,
    Sultan,
    Sumatra,
    Sussex,
    SwissHen, // (Schweizerhuhn)
    Welsummer,
    WhiteFacedBlackSpanish,
    Wyandotte,
    Yokohama
}

impl ToString for ChickenBreed {
    /// Converts the ChickenBreed enum to a human readable string.
    ///
    /// # Examples
    /// ``` rust
    /// use livestock_rs::chicken::ChickenBreed;
    ///
    /// let orp = ChickenBreed::Orpington;
    /// println!("{}", orp.to_string());
    /// ```
    fn to_string(&self) -> String {
        match self {
            ChickenBreed::AC => "AC".to_string(),
            ChickenBreed::Ameraucana => "Ameraucana".to_string(),
            ChickenBreed::Ancona => "Ancona".to_string(),
            ChickenBreed::Andalusian => "Andalusian".to_string(),
            ChickenBreed::AppenzellBeardedHen => "Appenzell Bearded Hen".to_string(),
            ChickenBreed::AppenzellPointedHoodHen => "Appenzell Pointed Hood Hen".to_string(),
            ChickenBreed::Araucana => "Araucana".to_string(),
            ChickenBreed::Aseel => "Aseel".to_string(),
            ChickenBreed::Australorp => "Australorp".to_string(),
            ChickenBreed::Baheij => "Baheij".to_string(),
            ChickenBreed::Bandara => "Bandara".to_string(),
            ChickenBreed::Barnevelders => "Barnevelders".to_string(),
            ChickenBreed::Brahma => "Brahma".to_string(),
            ChickenBreed::Buckeye => "Buckeye".to_string(),
            ChickenBreed::Buttercup => "Buttercup".to_string(),
            ChickenBreed::Campine => "Campine".to_string(),
            ChickenBreed::Catalana => "Catalana".to_string(),
            ChickenBreed::Chantecler => "Chantecler".to_string(),
            ChickenBreed::Cochin => "Cochin".to_string(),
            ChickenBreed::Cornish => "Cornish".to_string(),
            ChickenBreed::Crevecoeur => "Crevecoeur".to_string(),
            ChickenBreed::Cubalaya => "Cubalaya".to_string(),
            ChickenBreed::Delaware => "Delaware".to_string(),
            ChickenBreed::Dominiques => "Dominiques".to_string(),
            ChickenBreed::Dorking => "Dorking".to_string(),
            ChickenBreed::DutchBantam => "Dutch Bantam".to_string(),
            ChickenBreed::Faverolles => "Faverolles".to_string(),
            ChickenBreed::Friesian => "Friesian".to_string(),
            ChickenBreed::Frizzle => "Frizzle".to_string(),
            ChickenBreed::Gimmizah => "Gimmizah".to_string(),
            ChickenBreed::GoldenMontazah => "Golden Montazah".to_string(),
            ChickenBreed::Hamburg => "Hamburg".to_string(),
            ChickenBreed::Holland => "Holland".to_string(),
            ChickenBreed::Houdan => "Houdan".to_string(),
            ChickenBreed::Java => "Java".to_string(),
            ChickenBreed::JerseyGiant => "Jersey Giant".to_string(),
            ChickenBreed::LaFleche => "La Fleche".to_string(),
            ChickenBreed::Lakenvelder => "Lakenvelder".to_string(),
            ChickenBreed::Lamona => "Lamona".to_string(),
            ChickenBreed::Langshan => "Langshan".to_string(),
            ChickenBreed::Legbar => "Legbar".to_string(),
            ChickenBreed::Leghorn => "Leghorn".to_string(),
            ChickenBreed::Marans => "Marans".to_string(),
            ChickenBreed::Malay => "Malay".to_string(),
            ChickenBreed::Matrouh => "Matrouh".to_string(),
            ChickenBreed::Minorca => "Minorca".to_string(),
            ChickenBreed::ModernGame => "Modern Game".to_string(),
            ChickenBreed::NakedNeck => "Naked Neck".to_string(),
            ChickenBreed::Nankin => "Nankin".to_string(),
            ChickenBreed::NewHampshireRed => "New Hampshire Red".to_string(),
            ChickenBreed::OldEnglishGame => "Old English Game".to_string(),
            ChickenBreed::Orpington => "Orpington".to_string(),
            ChickenBreed::PlymouthRock => "Plymouth Rock".to_string(),
            ChickenBreed::Polish => "Polish".to_string(),
            ChickenBreed::RedCap => "Red Cap".to_string(),
            ChickenBreed::RhodeIslandRed => "Rhode Island Red".to_string(),
            ChickenBreed::RussianOrloff => "Russian Orloff".to_string(),
            ChickenBreed::Sasso => "Sasso".to_string(),
            ChickenBreed::Sebright => "Sebright".to_string(),
            ChickenBreed::Shamo => "Shamo".to_string(),
            ChickenBreed::SilkieBantam => "Silkie Bantam".to_string(),
            ChickenBreed::SilverMontazah => "Silver Montazah".to_string(),
            ChickenBreed::Styrian => "Styrian".to_string(),
            ChickenBreed::Sultan => "Sultan".to_string(),
            ChickenBreed::Sumatra => "Sumatra".to_string(),
            ChickenBreed::Sussex => "Sussex".to_string(),
            ChickenBreed::SwissHen => "Swiss Hen".to_string(),
            ChickenBreed::Welsummer => "Welsummer".to_string(),
            ChickenBreed::WhiteFacedBlackSpanish => "White Faced Black Spanish".to_string(),
            ChickenBreed::Wyandotte => "Wyandotte".to_string(),
            ChickenBreed::Yokohama => "Yokohama".to_string(),
        }
    }
}

/// Converts a string to a ChickenBreed enum.
///
/// # Examples
/// ``` rust
/// use livestock_rs::chicken::ChickenBreed;
/// use std::str::FromStr;
///
/// let breed = ChickenBreed::from_str("Buff orpington").unwrap();
/// println!("{:?}", breed);
/// ```

impl FromStr for ChickenBreed {
    type Err = std::io::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "ac" => Ok(ChickenBreed::AC),
            "ameraucana" => Ok(ChickenBreed::Ameraucana),
            "ancona" => Ok(ChickenBreed::Ancona),
            "andalusian" => Ok(ChickenBreed::Andalusian),
            "appenzell bearded hen" => Ok(ChickenBreed::AppenzellBeardedHen),
            "appenzell pointed hood hen" => Ok(ChickenBreed::AppenzellPointedHoodHen),
            "araucana" => Ok(ChickenBreed::Araucana),
            "aseel" => Ok(ChickenBreed::Aseel),
            "australorp" => Ok(ChickenBreed::Australorp),
            "baheij" => Ok(ChickenBreed::Baheij),
            "bandara" => Ok(ChickenBreed::Bandara),
            "barnevelders" => Ok(ChickenBreed::Barnevelders),
            "brahma" => Ok(ChickenBreed::Brahma),
            "buckeye" => Ok(ChickenBreed::Buckeye),
            "buttercup" => Ok(ChickenBreed::Buttercup),
            "campine" => Ok(ChickenBreed::Campine),
            "catalana" => Ok(ChickenBreed::Catalana),
            "chantecler" => Ok(ChickenBreed::Chantecler),
            "cochin" => Ok(ChickenBreed::Cochin),
            "cornish" => Ok(ChickenBreed::Cornish),
            "crevecoeur" => Ok(ChickenBreed::Crevecoeur),
            "cubalaya" => Ok(ChickenBreed::Cubalaya),
            "delaware" => Ok(ChickenBreed::Delaware),
            "dominiques" => Ok(ChickenBreed::Dominiques),
            "dorking" => Ok(ChickenBreed::Dorking),
            "dutch bantam" => Ok(ChickenBreed::DutchBantam),
            "faverolles" => Ok(ChickenBreed::Faverolles),
            "friesian" => Ok(ChickenBreed::Friesian),
            "frizzle" => Ok(ChickenBreed::Frizzle),
            "gimmizah" => Ok(ChickenBreed::Gimmizah),
            "golden montazah" => Ok(ChickenBreed::GoldenMontazah),
            "hamburg" => Ok(ChickenBreed::Hamburg),
            "holland" => Ok(ChickenBreed::Holland),
            "houdan" => Ok(ChickenBreed::Houdan),
            "java" => Ok(ChickenBreed::Java),
            "jersey giant" => Ok(ChickenBreed::JerseyGiant),
            "la fleche" => Ok(ChickenBreed::LaFleche),
            "lakenvelder" => Ok(ChickenBreed::Lakenvelder),
            "lamona" => Ok(ChickenBreed::Lamona),
            "langshan" => Ok(ChickenBreed::Langshan),
            "legbar" => Ok(ChickenBreed::Legbar),
            "leghorn" => Ok(ChickenBreed::Leghorn),
            "marans" => Ok(ChickenBreed::Marans),
            "malay" => Ok(ChickenBreed::Malay),
            "matrouh" => Ok(ChickenBreed::Matrouh),
            "minorca" => Ok(ChickenBreed::Minorca),
            "modern game" => Ok(ChickenBreed::ModernGame),
            "naked neck" | "turken" => Ok(ChickenBreed::NakedNeck),
            "nankin" => Ok(ChickenBreed::Nankin),
            "new hampshire red" => Ok(ChickenBreed::NewHampshireRed),
            "old english game" => Ok(ChickenBreed::OldEnglishGame),
            "orpington" | "buff orpington" | "lavender orpington" | "buff" => Ok(ChickenBreed::Orpington),
            "plymouth rock" => Ok(ChickenBreed::PlymouthRock),
            "polish" => Ok(ChickenBreed::Polish),
            "red cap" => Ok(ChickenBreed::RedCap),
            "rhode island red" => Ok(ChickenBreed::RhodeIslandRed),
            "russian orloff" => Ok(ChickenBreed::RussianOrloff),
            "sasso" => Ok(ChickenBreed::Sasso),
            "sebright" => Ok(ChickenBreed::Sebright),
            "shamo" => Ok(ChickenBreed::Shamo),
            "silkie bantam" => Ok(ChickenBreed::SilkieBantam),
            "silver montazah" => Ok(ChickenBreed::SilverMontazah),
            "styrian" => Ok(ChickenBreed::Styrian),
            "sultan" => Ok(ChickenBreed::Sultan),
            "sumatra" => Ok(ChickenBreed::Sumatra),
            "sussex" => Ok(ChickenBreed::Sussex),
            "swiss hen" | "schweizerhuhn" => Ok(ChickenBreed::SwissHen),
            "welsummer" => Ok(ChickenBreed::Welsummer),
            "white faced black spanish" => Ok(ChickenBreed::WhiteFacedBlackSpanish),
            "wyandotte" => Ok(ChickenBreed::Wyandotte),
            "yokohama" => Ok(ChickenBreed::Yokohama),
            _ => Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Unknown breed")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_string() {
        // we don't have any special cases, just test a few random ones
        let breeds = [
            (ChickenBreed::AC, "AC"),
            (ChickenBreed::Ameraucana, "Ameraucana"),
            (ChickenBreed::Ancona, "Ancona"),
            (ChickenBreed::Andalusian, "Andalusian"),
            (ChickenBreed::AppenzellBeardedHen, "Appenzell Bearded Hen"),
            (ChickenBreed::AppenzellPointedHoodHen, "Appenzell Pointed Hood Hen"),
            (ChickenBreed::Araucana, "Araucana"),
            (ChickenBreed::Aseel, "Aseel"),
            (ChickenBreed::Australorp, "Australorp"),
            (ChickenBreed::Baheij, "Baheij")
        ];

        for (breed, expected) in breeds.iter() {
            assert_eq!(breed.to_string(), *expected);
        }
    }

    #[test]
    fn test_from_str() {
        let breeds = [
            ("buff orpington", ChickenBreed::Orpington),
            ("buff", ChickenBreed::Orpington),
            ("lavender orpington", ChickenBreed::Orpington),
            ("turken", ChickenBreed::NakedNeck),
            ("schweizerhuhn", ChickenBreed::SwissHen),
            ("rhode island red", ChickenBreed::RhodeIslandRed),
            ("new hampshire red", ChickenBreed::NewHampshireRed),
            ("dutch bantam", ChickenBreed::DutchBantam),
            ("appenzell bearded hen", ChickenBreed::AppenzellBeardedHen),
            ("appenzell pointed hood hen", ChickenBreed::AppenzellPointedHoodHen),
            ("white faced black spanish", ChickenBreed::WhiteFacedBlackSpanish),
            ("silver montazah", ChickenBreed::SilverMontazah),
            ("golden montazah", ChickenBreed::GoldenMontazah)
        ];

        for (breed, expected) in breeds.iter() {
            assert_eq!(ChickenBreed::from_str(*breed).unwrap(), *expected);
        }
    }
}
