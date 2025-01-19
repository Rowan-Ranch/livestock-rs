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
    Africander,
    Akaushi,
    Alberes,
    Alentejana,
    Allmogekor,
    AmericanBreed,
    AmericanBraford,
    AmericanWhitePark,
    Amerifax,
    AmritMahal,
    AnatolianBlack,
    AndalusianBlack,
    AndalusianGrey,
    Angeln,
    Angus,
    Ankole,
    AnkoleWatusi,
    ArgentineCriollo,
    AsturianMountain,
    AsturianValley,
    Aubrac,
    AulieAta,
    AustralianBraford,
    AustralianFriesianSahiwal,
    AustralianLowline,
    AustralianMilkingZebu,
    Ayrshire,
    Azaouak,
    Bachaur,
    Baladi,
    BaltataRomaneasca,
    Barka,
    Barzona,
    Bazadais,
    Bearnais,
    Beefalo,
    Beefmaker,
    Beefmaster,
    BelarusRed,
    BelgianBlue,
    BelgianRed,
    BelmontAdaptaur,
    BelmontRed,
    BeltedGalloway,
    Bengali,
    Berrendas,
    Bhagnari,
    BlackHereford,
    BlacksidedTrondheimAndNorland,
    BlancaCacerena,
    BlancoOrejinegro,
    BlondeDAquitaine,
    Bonsmara,
    Boran,
    Bordelais,
    Brahman,
    Brahmousin,
    Brangus,
    Braunvieh,
    BritishWhite,
    BrownSwiss,
    Busa,
    Cachena,
    CanadianHighland,
    Canadienne,
    CanaryIsland,
    Canchim,
    CarinthianBlond,
    Caucasian,
    Channi,
    Charbray,
    Charolais,
    Chianina,
    Chinampo,
    ChineseBlackAndWhite,
    ChineseMongolian,
    ChineseXinjiangBrown,
    Cholistani,
    Corriente,
    CostenoConCuernos,
    Dajal,
    Damascus,
    Damietta,
    Dangi,
    DanishJersey,
    DanishRed,
    Deoni,
    Devon,
    Dexter,
    Dhanni,
    Djali,
    Dolafe,
    Droughtmaster,
    Dulong,
    DutchBelted,
    DutchFriesian,
    EastAnatolianRed,
    EnderbyIsland,
    EnglishLonghorn,
    EstonianRed,
    Evolene,
    Fighting,
    Finnish,
    Fjall,
    FloridaCracker,
    GalicianBlond,
    Galloway,
    Gaolao,
    Gascon,
    Gelbray,
    Gelbvieh,
    GermanAngusMoiled,
    GermanRedPied,
    Gir,
    Glan,
    Gloucester,
    Gobra,
    GreekShorthorn,
    GreekSteppe,
    Groningen,
    Guernsey,
    Guzerat,
    Hallikar,
    Hariana,
    Harton,
    HaysConverter,
    Hereford,
    Herens,
    Highland,
    Hinterwald,
    HolandoArgentino,
    Holstein,
    Horro,
    HungarianGrey,
    Icelandic,
    Illawarra,
    IndoBrazilian,
    IrishMoiled,
    IsraeliHolstein,
    IsraeliRed,
    Istoben,
    JamaicaBlack,
    JamaicaHope,
    JamaicaRed,
    Jaulan,
    Jersey,
    Kangayam,
    Kankrej,
    KaranFries,
    KaranSwiss,
    Kazakh,
    Kenwariya,
    Kerry,
    Kherigarh,
    Khillari,
    Kholmogory,
    Kilis,
    KrishnaValley,
    KurdiBlack,
    Kuri,
    LatvianBrown,
    Limousin,
    Limpurger,
    LincolnRed,
    LithuanianRed,
    Lohani,
    Lourdais,
    Luing,
    MadagascarZebu,
    MaineAnjou,
    Malvi,
    Mandalong,
    Marchigiana,
    Maremmana,
    Masai,
    Mashona,
    Maure,
    Mazandarani,
    MeuseRhineYssel,
    Mewati,
    MilkingDevon,
    MilkingShorthorn,
    MiniatureZebu,
    Mirandesa,
    Modicana,
    Montbeliard,
    Morucha,
    Murboden,
    MurrayGrey,
    Muturu,
    Ndama,
    Nagori,
    Nanyang,
    Nelore,
    Nguni,
    Nimari,
    Normande,
    NorwegianRed,
    Ongole,
    OrmaBoran,
    Oropa,
    Ovambo,
    Parthenais,
    PhilippineNative,
    Piedmontese,
    Pinzgauer,
    PolishRed,
    PolledHereford,
    Ponwar,
    Qinchuan,
    Rath,
    Rathi,
    RatienGray,
    RedAngus,
    RedBrangus,
    RedFulani,
    RedPiedFriesian,
    RedPoll,
    RedPolledOstland,
    RedSindhi,
    RedSteppe,
    Reggiana,
    Retinta,
    Rojhan,
    Romagnola,
    Romosinuano,
    RussianBlackPied,
    RX3,
    Sahiwal,
    Salers,
    Salorn,
    SanMartinero,
    Sanhe,
    SantaCruz,
    SantaGertrudis,
    Sarabi,
    Senepol,
    Shetland,
    Shorthorn,
    Siboney,
    Simbrah,
    Simmental,
    Siri,
    SlovenianCika,
    SouthDevon,
    SudaneseFulani,
    Sussex,
    SwedishFriesian,
    SwedishRedPolled,
    SwedishRedAndWhite,
    Tarentaise,
    Telemark,
    TexasLonghorn,
    Texon,
    Tharparkar,
    Tswana,
    Tuli,
    TurkishGreySteppe,
    UkrainianBeef,
    UkrainianGrey,
    UkrainianWhitehead,
    Umblachery,
    UralBlackPied,
    VestlandFjord,
    VestlandRedPolled,
    Vosges,
    Wagyu,
    WelshBlack,
    WhitePark,
    Yanbian,
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
            CattleBreed::AnkoleWatusi => "Anokole-Watusi".to_string(),
            CattleBreed:AulieAta => "Aulie-Ata".to_string(),
            CattleBreed::Bearnais => "Béarnais".to_string(),
            CattleBreed::BlancaCacerena => "Blanca Cacereña/White Cáceres".to_string(),
            CattleBreed::BlondeDAquitaine => "Blonde d'Aquitaine".to_string(),
            CattleBreed::ChineseBlackAndWhite => "Chinese Black-and-White".to_string(),
            CattleBreed::CostenoConCuernos => "Costeño con Cuernos".to_string(),
            CattleBreed::DutchBelted => "Dutch Belted (Lakenvelder)".to_string(),
            _ => format!("{:?}", self),
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
