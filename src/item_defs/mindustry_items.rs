/// Module for mindustry item definitions
#[repr(u16)]
#[derive(Clone, Copy, Eq, EnumString)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[allow(missing_docs)]
enum Item {
    #[strum(serialize = "id")]
    Id = logicId,
    #[strum(serialize = "0")]
    0 = 0,
    #[strum(serialize = "1")]
    1 = 1,
    #[strum(serialize = "2")]
    2 = 2,
    #[strum(serialize = "3")]
    3 = 3,
    #[strum(serialize = "4")]
    4 = 4,
    #[strum(serialize = "5")]
    5 = 5,
    #[strum(serialize = "6")]
    6 = 6,
    #[strum(serialize = "7")]
    7 = 7,
    #[strum(serialize = "8")]
    8 = 8,
    #[strum(serialize = "9")]
    9 = 9,
    #[strum(serialize = "10")]
    10 = 10,
    #[strum(serialize = "11")]
    11 = 11,
    #[strum(serialize = "12")]
    12 = 12,
    #[strum(serialize = "13")]
    13 = 13,
    #[strum(serialize = "14")]
    14 = 14,
    #[strum(serialize = "15")]
    15 = 15,
    #[strum(serialize = "16")]
    16 = 16,
    #[strum(serialize = "17")]
    17 = 17,
    #[strum(serialize = "18")]
    18 = 18,
    #[strum(serialize = "19")]
    19 = 19,
    #[strum(serialize = "20")]
    20 = -1,
    #[strum(serialize = "21")]
    21 = -1,
}