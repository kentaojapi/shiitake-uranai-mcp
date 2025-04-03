use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Constellation {
    Aries,
    Taurus,
    Gemini,
    Cancer,
    Leo,
    Virgo,
    Libra,
    Scorpio,
    Sagittarius,
    Capricorn,
    Aquarius,
    Pisces,
}

impl FromStr for Constellation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "aries" => Ok(Constellation::Aries),
            "taurus" => Ok(Constellation::Taurus),
            "gemini" => Ok(Constellation::Gemini),
            "cancer" => Ok(Constellation::Cancer),
            "leo" => Ok(Constellation::Leo),
            "virgo" => Ok(Constellation::Virgo),
            "libra" => Ok(Constellation::Libra),
            "scorpio" => Ok(Constellation::Scorpio),
            "sagittarius" => Ok(Constellation::Sagittarius),
            "capricorn" => Ok(Constellation::Capricorn),
            "aquarius" => Ok(Constellation::Aquarius),
            "pisces" => Ok(Constellation::Pisces),
            _ => Err(format!("Invalid constellation: {}", s)),
        }
    }
}

impl fmt::Display for Constellation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Constellation::Aries => "aries",
            Constellation::Taurus => "taurus",
            Constellation::Gemini => "gemini",
            Constellation::Cancer => "cancer",
            Constellation::Leo => "leo",
            Constellation::Virgo => "virgo",
            Constellation::Libra => "libra",
            Constellation::Scorpio => "scorpio",
            Constellation::Sagittarius => "sagittarius",
            Constellation::Capricorn => "capricorn",
            Constellation::Aquarius => "aquarius",
            Constellation::Pisces => "pisces",
        };
        write!(f, "{}", s)
    }
}
