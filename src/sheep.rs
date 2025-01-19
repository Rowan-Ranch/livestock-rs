use inflector::Inflector;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

/// An enum representing the different breeds of sheep.
///
/// Initial data from: <https://breeds.okstate.edu/sheep/>
///
/// # Examples
/// ``` rust
/// use livestock_rs::sheep::SheepBreed;
///
/// let breed = SheepBreed::Dorper;
/// println!("{:?}", breed);
/// ```
#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum SheepBreed {
    Acipayam,
    Adal,
    AfghanArabi,
    Africana,
    Alai,
    Alcarrena,
    AlgarveChurro,
    AlgerianArab,
    Altai,
    Altay,
    AmericanBlackbelly,
    Apennine,
    Arabi,
    ArapawaIsland,
    Awassi,
    Balkhi,
    Baluchi,
    BalwenWelshMountain,
    BarbadosBlackbelly,
    BavarianForest,
    BentheimerLandschaf,
    Bergamasca,
    BeulahSpeckledFace,
    Bibrik,
    Biellese,
    BlackWelshMountain,
    BlackheadPersian,
    BleuDuMaine,
    BluefacedLeicester,
    Bond,
    BooroolaMerino,
    BorderLeicester,
    Boreray,
    Bovska,
    BraunesBergschaf,
    BrazilianSomali,
    BrecknockHillCheviot,
    BritishMilk,
    Brillenschaf,
    BundnerOberland,
    CaliforniaRed,
    CaliforniaVariegatedMutant,
    CampanianBarbary,
    CastlemilkMoorit,
    Charollais,
    Cheviot,
    Chios,
    Cholistani,
    ClunForest,
    CoburgerFuchsschaf,
    Columbia,
    Comeback,
    Comisana,
    Coopworth,
    Cormo,
    Corriedale,
    Cotswold,
    Criollo,
    Daglic,
    Dala,
    Dalesbred,
    Damani,
    Damara,
    DanishLandrace,
    Dartmoor,
    Debouillet,
    DelaineMerino,
    DerbyshireGritstone,
    Dorper,
    DevonClosewool,
    DeutschesBlaukoepfigesFleischschaf,
    DorsetDown,
    Dorset,
    Drysdale,
    Elliottdale,
    ExmoorHorn,
    Fabrianese,
    Faeroes,
    Finnsheep,
    FonthillMerino,
    FriesianMilk,
    Galway,
    GansuAlpineFineWool,
    GentileDiPuglia,
    GermanBlackheadedMutton,
    GermanMountain,
    GermanMuttonMerino,
    GermanWhiteheadedMutton,
    Gotland,
    GraueGehoernteHeidschnucke,
    Gromark,
    GulfCoastNative,
    Gute,
    Hampshire,
    Han,
    Harnai,
    HashtNagri,
    Hazaragie,
    Hebridean,
    Herdwick,
    HillRadnor,
    HogIsland,
    Hu,
    Icelandic,
    IleDeFrance,
    IstrianPramenka,
    Jacob,
    JezerskoSolcava,
    Kachhi,
    Kajli,
    Karakul,
    Katahdin,
    KerryHill,
    Kooka,
    Langhe,
    Lati,
    LeicesterLongwool,
    Leineschaf,
    Lincoln,
    Llanwenog,
    Lleyn,
    Lohi,
    Lonk,
    Luzein,
    ManxLoaghtan,
    Masai,
    Massese,
    MediumWoolMerino,
    Mehraban,
    Merinolandschaf,
    Moghani,
    Montadale,
    MoradaNova,
    Mouflon,
    NavajoChurro,
    NorfolkHorn,
    NorthCountryCheviot,
    NorwegianFur,
    OldNorwegian,
    Orkney,
    Ossimi,
    Oxford,
    Pagliarola,
    Pelibuey,
    Perendale,
    Pinzirita,
    PittIsland,
    PollMerino,
    Polwarth,
    Polypay,
    PomeranianCoarsewool,
    Portland,
    Priangan,
    Qashqai,
    QinghaiBlackTibetan,
    QinghaiSemifinewool,
    Quadrella,
    QuanglinLargeTail,
    RaboLargo,
    Racka,
    Rambouillet,
    RasaAragonesa,
    RedEngadine,
    Rhoenschaf,
    RideauArcott,
    Romanov,
    Romney,
    RougeDeIQuest,
    RoughFell,
    RoyalWhite,
    Rya,
    Ryeland,
    Rygja,
    Sahel,
    SantaCruz,
    SantaInes,
    Sardinian,
    SarPlanina,
    ScottishBlackface,
    SicilianBarbary,
    Shetland,
    Shropshire,
    Skudde,
    Soay,
    Somali,
    Sopravissana,
    SouthAfricanMerino,
    SouthAfrianMuttonMerino,
    SouthSuffolk,
    Southdown,
    SouthWalesMountain,
    Spaeslau,
    Spiegel,
    StCroix,
    Steigar,
    Steinschaf,
    StrongWoolMerino,
    Suffolk,
    Sumavska,
    Swaledale,
    SwedishFur,
    Targhee,
    Teeswater,
    Texel,
    Thalli,
    Tong,
    Touabire,
    Tsurcana,
    Tunis,
    TyrolMountain,
    Uda,
    Ujumqin,
    Ushant,
    ValaisBlacknose,
    Vendeen,
    Walachenschaf,
    WallisCountry,
    Waziri,
    WeisseHornloseHeidschnucke,
    WelshHillSpeckledFace,
    WelshMountain,
    WelshMountainBadgerFaced,
    Wensleydale,
    WestAfricanDwarf,
    WhiteSuffolk,
    WhitefaceDartmoor,
    WhitefaceWoodland,
    WiltshireHorn,
    XinjiangFinewool,
    Yankasa,
    YemenWhite,
    Yemeni,
    Yiecheng,
    Yoroo,
    YunnanSemifinewool,
    Zaghawa,
    Zagoria,
    Zaian,
    ZaireLongLegged,
    Zakynthos,
    ZeelandMilk,
    Zel,
    Zelazna,
    Zemmour,
    ZetaYellow,
    Zlatusha,
    Zoulay
}

impl ToString for SheepBreed {
    /// Converts the SheepBreed enum to a human readable string.
    ///
    /// # Examples
    /// ``` rust
    /// use livestock_rs::sheep::SheepBreed;
    ///
    /// let vendeen = SheepBreed::Vendeen;
    /// println!("{}", vendeen.to_string());
    /// ```
    fn to_string(&self) -> String {
        match self {
            SheepBreed::BeulahSpeckledFace => "Beulah Speckled-Face".to_string(),
            SheepBreed::BleuDuMaine => "Bleu du Maine".to_string(),
            SheepBreed::BundnerOberland => "Bündner Oberland".to_string(),
            SheepBreed::GansuAlpineFineWool => "Gansu Alpine Fine-Wool".to_string(),
            SheepBreed::IleDeFrance => "Ile-de-France".to_string(),
            SheepBreed::JezerskoSolcava => "Jezersko-Solčava".to_string(),
            SheepBreed::NavajoChurro => "Navajo-Churro".to_string(),
            SheepBreed::QuanglinLargeTail => "Quanglin Large-Tail".to_string(),
            SheepBreed::RougeDeIQuest => "Rouge de l'Ouest".to_string(),
            SheepBreed::Sahel => "Sahel-type".to_string(),
            SheepBreed::SantaInes => "Santa Inês".to_string(),
            SheepBreed::StCroix => "St. Croix (Virgin Island White)".to_string(),
            SheepBreed::Vendeen => "Vendéen".to_string(),
            SheepBreed::ZaireLongLegged => "Zaire Long-Legged".to_string(),
            _ => format!("{:?}", self).to_title_case(),
        }
    }
}

/// Converts a string to a SheepBreed enum.
///
/// # Examples
/// ``` rust
/// use livestock_rs::sheep::SheepBreed;
/// use std::str::FromStr;
///
/// let breed = SheepBreed::from_str("Rouge de l'Ouest").unwrap();
/// println!("{:?}", breed);
/// ```

impl FromStr for SheepBreed {
    type Err = std::io::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().replace("-", " ").as_str() {
            "acipayam" => Ok(SheepBreed::Acipayam),
            "adal" => Ok(SheepBreed::Adal),
            "afghan arabi" => Ok(SheepBreed::AfghanArabi),
            "africana" => Ok(SheepBreed::Africana),
            "alai" => Ok(SheepBreed::Alai),
            "alcarrena" => Ok(SheepBreed::Alcarrena),
            "algarve churro" => Ok(SheepBreed::AlgarveChurro),
            "algerian arab" => Ok(SheepBreed::AlgerianArab),
            "altai" => Ok(SheepBreed::Altai),
            "altay" => Ok(SheepBreed::Altay),
            "american blackbelly" => Ok(SheepBreed::AmericanBlackbelly),
            "apennine" => Ok(SheepBreed::Apennine),
            "arabi" => Ok(SheepBreed::Arabi),
            "arapawa island" => Ok(SheepBreed::ArapawaIsland),
            "awassi" => Ok(SheepBreed::Awassi),
            "balkhi" => Ok(SheepBreed::Balkhi),
            "baluchi" => Ok(SheepBreed::Baluchi),
            "balwen welsh mountain" => Ok(SheepBreed::BalwenWelshMountain),
            "barbados blackbelly" => Ok(SheepBreed::BarbadosBlackbelly),
            "bavarian forest" => Ok(SheepBreed::BavarianForest),
            "bentheimer landschaf" => Ok(SheepBreed::BentheimerLandschaf),
            "bergamasca" => Ok(SheepBreed::Bergamasca),
            "beulah speckled face" | "beulah speckle face"=> Ok(SheepBreed::BeulahSpeckledFace),
            "bibrik" => Ok(SheepBreed::Bibrik),
            "biellese" => Ok(SheepBreed::Biellese),
            "blackwelshmountain" => Ok(SheepBreed::BlackWelshMountain),
            "blackheadpersian" => Ok(SheepBreed::BlackheadPersian),
            "bleu du maine" => Ok(SheepBreed::BleuDuMaine),
            "bluefaced leicester" => Ok(SheepBreed::BluefacedLeicester),
            "bond" => Ok(SheepBreed::Bond),
            "booroola merino" => Ok(SheepBreed::BooroolaMerino),
            "border leicester" => Ok(SheepBreed::BorderLeicester),
            "boreray" => Ok(SheepBreed::Boreray),
            "bovska" => Ok(SheepBreed::Bovska),
            "braunes bergschaf" => Ok(SheepBreed::BraunesBergschaf),
            "brazilian somali" => Ok(SheepBreed::BrazilianSomali),
            "brecknock hill cheviot" => Ok(SheepBreed::BrecknockHillCheviot),
            "british milk" => Ok(SheepBreed::BritishMilk),
            "brillenschaf" => Ok(SheepBreed::Brillenschaf),
            "bundner oberland" | "bündner oberland" => Ok(SheepBreed::BundnerOberland),
            "california red" => Ok(SheepBreed::CaliforniaRed),
            "california variegated mutant" => Ok(SheepBreed::CaliforniaVariegatedMutant),
            "campanian barbary" => Ok(SheepBreed::CampanianBarbary),
            "castlemilk moorit" => Ok(SheepBreed::CastlemilkMoorit),
            "charollais" => Ok(SheepBreed::Charollais),
            "cheviot" => Ok(SheepBreed::Cheviot),
            "chios" => Ok(SheepBreed::Chios),
            "cholistani" => Ok(SheepBreed::Cholistani),
            "clun forest" => Ok(SheepBreed::ClunForest),
            "coburger fuchsschaf" => Ok(SheepBreed::CoburgerFuchsschaf),
            "columbia" => Ok(SheepBreed::Columbia),
            "comeback" => Ok(SheepBreed::Comeback),
            "comisana" => Ok(SheepBreed::Comisana),
            "coopworth" => Ok(SheepBreed::Coopworth),
            "cormo" => Ok(SheepBreed::Cormo),
            "corriedale" => Ok(SheepBreed::Corriedale),
            "cotswold" => Ok(SheepBreed::Cotswold),
            "criollo" => Ok(SheepBreed::Criollo),
            "daglic" => Ok(SheepBreed::Daglic),
            "dala" => Ok(SheepBreed::Dala),
            "dalesbred" => Ok(SheepBreed::Dalesbred),
            "damani" => Ok(SheepBreed::Damani),
            "damara" => Ok(SheepBreed::Damara),
            "danish landrace" => Ok(SheepBreed::DanishLandrace),
            "dartmoor" => Ok(SheepBreed::Dartmoor),
            "debouillet" => Ok(SheepBreed::Debouillet),
            "delaine merino" => Ok(SheepBreed::DelaineMerino),
            "derbyshire gritstone" => Ok(SheepBreed::DerbyshireGritstone),
            "dorper" => Ok(SheepBreed::Dorper),
            "devon closewool" => Ok(SheepBreed::DevonClosewool),
            "deutsches blaukoepfiges fleischschaf" => Ok(SheepBreed::DeutschesBlaukoepfigesFleischschaf),
            "dorset down" => Ok(SheepBreed::DorsetDown),
            "dorset" => Ok(SheepBreed::Dorset),
            "drysdale" => Ok(SheepBreed::Drysdale),
            "elliottdale" => Ok(SheepBreed::Elliottdale),
            "exmoor horn" => Ok(SheepBreed::ExmoorHorn),
            "fabrianese" => Ok(SheepBreed::Fabrianese),
            "faeroes" => Ok(SheepBreed::Faeroes),
            "finnsheep" => Ok(SheepBreed::Finnsheep),
            "fonthill merino" => Ok(SheepBreed::FonthillMerino),
            "friesian milk" => Ok(SheepBreed::FriesianMilk),
            "galway" => Ok(SheepBreed::Galway),
            "gansu alpine fine wool" => Ok(SheepBreed::GansuAlpineFineWool),
            "gentile di puglia" => Ok(SheepBreed::GentileDiPuglia),
            "german blackheaded mutton" => Ok(SheepBreed::GermanBlackheadedMutton),
            "german mountain" => Ok(SheepBreed::GermanMountain),
            "german mutton merino" => Ok(SheepBreed::GermanMuttonMerino),
            "german whiteheaded mutton" => Ok(SheepBreed::GermanBlackheadedMutton),
            "gotland" => Ok(SheepBreed::Gotland),
            "graue gehoernte heidschnucke" => Ok(SheepBreed::GraueGehoernteHeidschnucke),
            "gromark" => Ok(SheepBreed::Gromark),
            "gulf coast native" => Ok(SheepBreed::GulfCoastNative),
            "gute" => Ok(SheepBreed::Gute),
            "hampshire" => Ok(SheepBreed::Hampshire),
            "han" => Ok(SheepBreed::Han),
            "harnai" => Ok(SheepBreed::Harnai),
            "hasht nagri" => Ok(SheepBreed::HashtNagri),
            "hazaragie" => Ok(SheepBreed::Hazaragie),
            "hebridean" => Ok(SheepBreed::Hebridean),
            "herdwick" => Ok(SheepBreed::Herdwick),
            "hill radnor" => Ok(SheepBreed::HillRadnor),
            "hog island" => Ok(SheepBreed::HogIsland),
            "hu" => Ok(SheepBreed::Hu),
            "icelandic" => Ok(SheepBreed::Icelandic),
            "ile de france" => Ok(SheepBreed::IleDeFrance),
            "istrian pramenka" => Ok(SheepBreed::IstrianPramenka),
            "jacob" => Ok(SheepBreed::Jacob),
            "jezersko solcava" | "jezersko solčava" => Ok(SheepBreed::JezerskoSolcava), 
            "kachhi" => Ok(SheepBreed::Kachhi),
            "kajli" => Ok(SheepBreed::Kajli),
            "karakul" => Ok(SheepBreed::Karakul),
            "katahdin" => Ok(SheepBreed::Katahdin),
            "kerry hill" => Ok(SheepBreed::KerryHill),
            "kooka" => Ok(SheepBreed::Kooka),
            "langhe" => Ok(SheepBreed::Langhe),
            "lati" => Ok(SheepBreed::Lati),
            "leicester longwool" => Ok(SheepBreed::LeicesterLongwool),
            "leineschaf" => Ok(SheepBreed::Leineschaf),
            "lincoln" => Ok(SheepBreed::Lincoln),
            "llanwenog" => Ok(SheepBreed::Llanwenog),
            "lleyn" => Ok(SheepBreed::Lleyn),
            "lohi" => Ok(SheepBreed::Lohi),
            "lonk" => Ok(SheepBreed::Lonk),
            "luzein" => Ok(SheepBreed::Luzein),
            "manx loaghtan" => Ok(SheepBreed::ManxLoaghtan),
            "masai" => Ok(SheepBreed::Masai),
            "massese" => Ok(SheepBreed::Massese),
            "medium wool merino" => Ok(SheepBreed::MediumWoolMerino),
            "mehraban" => Ok(SheepBreed::Mehraban),
            "merinolandschaf" => Ok(SheepBreed::Merinolandschaf),
            "moghani" => Ok(SheepBreed::Moghani),
            "montadale" => Ok(SheepBreed::Montadale),
            "morada nova" => Ok(SheepBreed::MoradaNova),
            "mouflon" => Ok(SheepBreed::Mouflon),
            "navajo churro" | "navajo-churro" => Ok(SheepBreed::NavajoChurro),
            "norfolk horn" => Ok(SheepBreed::NorfolkHorn),
            "north country cheviot" => Ok(SheepBreed::NorthCountryCheviot),
            "norwegian fur" => Ok(SheepBreed::NorwegianFur),
            "old norwegian" => Ok(SheepBreed::OldNorwegian),
            "orkney" => Ok(SheepBreed::Orkney),
            "ossimi" => Ok(SheepBreed::Ossimi),
            "oxford" => Ok(SheepBreed::Oxford),
            "pagliarola" => Ok(SheepBreed::Pagliarola),
            "pelibuey" => Ok(SheepBreed::Pelibuey),
            "perendale" => Ok(SheepBreed::Perendale),
            "pinzirita" => Ok(SheepBreed::Pinzirita),
            "pitt island" => Ok(SheepBreed::PittIsland),
            "poll merino" => Ok(SheepBreed::PollMerino),
            "polwarth" => Ok(SheepBreed::Polwarth),
            "polypay" => Ok(SheepBreed::Polypay),
            "pomeranian coarsewool" => Ok(SheepBreed::PomeranianCoarsewool),
            "portland" => Ok(SheepBreed::Portland),
            "priangan" => Ok(SheepBreed::Priangan),
            "qashqai" => Ok(SheepBreed::Qashqai),
            "qinghai black tibetan" => Ok(SheepBreed::QinghaiBlackTibetan),
            "qinghai semifinewool" => Ok(SheepBreed::QinghaiSemifinewool),
            "quadrella" => Ok(SheepBreed::Quadrella),
            "quanglin large tail" | "quanglin large-tail" => Ok(SheepBreed::QuanglinLargeTail),
            "rabo largo" => Ok(SheepBreed::RaboLargo),
            "racka" => Ok(SheepBreed::Racka),
            "rambouillet" => Ok(SheepBreed::Rambouillet),
            "rasa aragonesa" => Ok(SheepBreed::RasaAragonesa),
            "red engadine" => Ok(SheepBreed::RedEngadine),
            "rhoenschaf" => Ok(SheepBreed::Rhoenschaf),
            "rideau arcott" => Ok(SheepBreed::RideauArcott),
            "romanov" => Ok(SheepBreed::Romanov),
            "romney" => Ok(SheepBreed::Romney),
            "rouge de l'ouest" => Ok(SheepBreed::RougeDeIQuest),
            "rough fell" => Ok(SheepBreed::RoughFell),
            "royal white" => Ok(SheepBreed::RoyalWhite),
            "rya" => Ok(SheepBreed::Rya),
            "ryeland" => Ok(SheepBreed::Ryeland),
            "rygja" => Ok(SheepBreed::Rygja),
            "sahel" | "sahel type" => Ok(SheepBreed::Sahel),
            "santa cruz" => Ok(SheepBreed::SantaCruz),
            "santa ines" | "santa inês" => Ok(SheepBreed::SantaInes),
            "sardinian" => Ok(SheepBreed::Sardinian),
            "sar planina" => Ok(SheepBreed::SarPlanina),
            "scottish blackface" => Ok(SheepBreed::ScottishBlackface),
            "sicilian barbary" => Ok(SheepBreed::SicilianBarbary),
            "shetland" => Ok(SheepBreed::Shetland),
            "shropshire" => Ok(SheepBreed::Shropshire),
            "skudde" => Ok(SheepBreed::Skudde),
            "soay" => Ok(SheepBreed::Soay),
            "somali" => Ok(SheepBreed::Somali),
            "sopravissana" => Ok(SheepBreed::Sopravissana),
            "south african merino" => Ok(SheepBreed::SouthAfricanMerino),
            "south afrian mutton merino" => Ok(SheepBreed::SouthAfrianMuttonMerino),
            "south suffolk" => Ok(SheepBreed::SouthSuffolk),
            "southdown" => Ok(SheepBreed::Southdown),
            "south wales mountain" => Ok(SheepBreed::SouthWalesMountain),
            "spaeslau" => Ok(SheepBreed::Spaeslau),
            "spiegel" => Ok(SheepBreed::Spiegel),
            "st croix" | "virgin island white" | "st. croix (virgin island white)" | "st. croix" | "saint croix" => Ok(SheepBreed::StCroix),
            "steigar" => Ok(SheepBreed::Steigar),
            "steinschaf" => Ok(SheepBreed::Steinschaf),
            "strong wool merino" => Ok(SheepBreed::StrongWoolMerino),
            "suffolk" => Ok(SheepBreed::Suffolk),
            "sumavska" => Ok(SheepBreed::Sumavska),
            "swaledale" => Ok(SheepBreed::Swaledale),
            "swedish fur" => Ok(SheepBreed::SwedishFur),
            "targhee" => Ok(SheepBreed::Targhee),
            "teeswater" => Ok(SheepBreed::Teeswater),
            "texel" => Ok(SheepBreed::Texel),
            "thalli" => Ok(SheepBreed::Thalli),
            "tong" => Ok(SheepBreed::Tong),
            "touabire" => Ok(SheepBreed::Touabire),
            "tsurcana" => Ok(SheepBreed::Tsurcana),
            "tunis" => Ok(SheepBreed::Tunis),
            "tyrol mountain" => Ok(SheepBreed::TyrolMountain),
            "uda" => Ok(SheepBreed::Uda),
            "ujumqin" => Ok(SheepBreed::Ujumqin),
            "ushant" => Ok(SheepBreed::Ushant),
            "valais blacknose" => Ok(SheepBreed::ValaisBlacknose),
            "vendeen" | "vendéen" => Ok(SheepBreed::Vendeen),
            "walachenschaf" => Ok(SheepBreed::Walachenschaf),
            "wallis country" => Ok(SheepBreed::WallisCountry),
            "waziri" => Ok(SheepBreed::Waziri),
            "wiess hornlose heidschnucke" => Ok(SheepBreed::WeisseHornloseHeidschnucke),
            "welsh hill speckled face" => Ok(SheepBreed::WelshHillSpeckledFace),
            "welsh mountain" => Ok(SheepBreed::WelshMountain),
            "welsh mountain badger faced" => Ok(SheepBreed::WelshMountainBadgerFaced),
            "wensleydale" => Ok(SheepBreed::Wensleydale),
            "west african dwarf" => Ok(SheepBreed::WestAfricanDwarf),
            "white suffolk" => Ok(SheepBreed::WhiteSuffolk),
            "whiteface dartmoor" => Ok(SheepBreed::WhitefaceDartmoor),
            "whiteface woodland" => Ok(SheepBreed::WhitefaceWoodland),
            "wiltshire horn" => Ok(SheepBreed::WiltshireHorn),
            "xinjiang finewool" => Ok(SheepBreed::XinjiangFinewool),
            "yankasa" => Ok(SheepBreed::Yankasa),
            "yemen white" => Ok(SheepBreed::YemenWhite),
            "yemeni" => Ok(SheepBreed::Yemeni),
            "yiecheng" => Ok(SheepBreed::Yiecheng),
            "yoroo" => Ok(SheepBreed::Yoroo),
            "yunnan semifinewool" => Ok(SheepBreed::YunnanSemifinewool),
            "zaghawa" => Ok(SheepBreed::Zaghawa),
            "zagoria" => Ok(SheepBreed::Zagoria),
            "zaian" => Ok(SheepBreed::Zaian),
            "zaire long legged" => Ok(SheepBreed::ZaireLongLegged),
            "zakynthos" => Ok(SheepBreed::Zakynthos),
            "zeeland milk" => Ok(SheepBreed::ZeelandMilk),
            "zel" => Ok(SheepBreed::Zel),
            "zelazna" => Ok(SheepBreed::Zelazna),
            "zemmour" => Ok(SheepBreed::Zemmour),
            "zeta yellow" => Ok(SheepBreed::ZetaYellow),
            "zlatusha" => Ok(SheepBreed::Zlatusha),
            "zoulay" => Ok(SheepBreed::Zoulay),
            _ => Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Unknown sheep breed")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_string_special_cases() {
        let special_case_breeds = [
            (SheepBreed::BeulahSpeckledFace, "Beulah Speckled-Face"),
            (SheepBreed::BleuDuMaine, "Bleu du Maine"),
            (SheepBreed::BundnerOberland, "Bündner Oberland"),
            (SheepBreed::GansuAlpineFineWool, "Gansu Alpine Fine-Wool"),
            (SheepBreed::IleDeFrance, "Ile-de-France"),
            (SheepBreed::JezerskoSolcava, "Jezersko-Solčava"),
            (SheepBreed::NavajoChurro, "Navajo-Churro"),
            (SheepBreed::QuanglinLargeTail, "Quanglin Large-Tail"),
            (SheepBreed::RougeDeIQuest, "Rouge de l'Ouest"),
            (SheepBreed::Sahel, "Sahel-type"),
            (SheepBreed::SantaInes, "Santa Inês"),
            (SheepBreed::StCroix, "St. Croix (Virgin Island White)"),
            (SheepBreed::Vendeen, "Vendéen"),
            (SheepBreed::ZaireLongLegged, "Zaire Long-Legged"),
        ];

        for (breed, expected) in special_case_breeds.iter() {
            assert_eq!(breed.to_string(), *expected);
        }
    }

    #[test]
    fn test_from_string_special_cases() {
        let special_case_breeds = [
            ("Beulah Speckled-Face", SheepBreed::BeulahSpeckledFace),
            ("Bleu du Maine", SheepBreed::BleuDuMaine),
            ("Bündner Oberland", SheepBreed::BundnerOberland),
            ("Gansu Alpine Fine-Wool", SheepBreed::GansuAlpineFineWool),
            ("Ile-de-France", SheepBreed::IleDeFrance),
            ("Jezersko-Solčava", SheepBreed::JezerskoSolcava),
            ("Navajo-Churro", SheepBreed::NavajoChurro),
            ("Quanglin Large-Tail", SheepBreed::QuanglinLargeTail),
            ("Rouge de l'Ouest", SheepBreed::RougeDeIQuest),
            ("Sahel-type", SheepBreed::Sahel),
            ("Santa Inês", SheepBreed::SantaInes),
            ("St. Croix (Virgin Island White)", SheepBreed::StCroix),
            ("Vendéen", SheepBreed::Vendeen),
            ("Zaire Long-Legged", SheepBreed::ZaireLongLegged),
        ];

        for (breed, expected) in special_case_breeds.iter() {
            assert_eq!(SheepBreed::from_str(breed).unwrap(), *expected);
        }
    }
}