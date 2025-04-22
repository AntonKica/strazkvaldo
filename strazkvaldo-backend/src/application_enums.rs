use num_enum::TryFromPrimitive;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

pub trait EnumModelResponseTrait {
    fn to_text(&self) -> &str;
}
#[repr(u8)]
#[derive(Debug, Clone, Serialize, Deserialize, TryFromPrimitive)]
pub enum CriticalityType {
    Low = 0,
    Medium = 1,
    High = 2,
}
impl fmt::Display for CriticalityType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl FromStr for CriticalityType {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "Low" => Ok(CriticalityType::Low),
            "Medium" => Ok(CriticalityType::Medium),
            "High" => Ok(CriticalityType::High),
            _ => Err(()),
        }
    }
}
impl EnumModelResponseTrait for CriticalityType {
    fn to_text(&self) -> &str {
        match self {
            CriticalityType::Low => "nízka",
            CriticalityType::Medium => "stredná",
            CriticalityType::High => "vysoká",
        }
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AppUserRole {
    Admin = 0,
    User = 1,
}
impl fmt::Display for AppUserRole {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl FromStr for AppUserRole {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "Admin" => Ok(AppUserRole::Admin),
            "User" => Ok(AppUserRole::User),
            _ => Err(()),
        }
    }
}
impl EnumModelResponseTrait for AppUserRole {
    fn to_text(&self) -> &str {
        match self {
            AppUserRole::Admin => "administrátor",
            AppUserRole::User => "používateľ",
        }
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Periodicity {
    Day,   // => no unit needed
    Week,  // => day of week
    Month, // => day of month
    Year,  // => day of year
}
impl fmt::Display for Periodicity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl FromStr for Periodicity {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "Day" => Ok(Periodicity::Day),
            "Week" => Ok(Periodicity::Week),
            "Month" => Ok(Periodicity::Month),
            "Year" => Ok(Periodicity::Year),
            _ => Err(()),
        }
    }
}
impl EnumModelResponseTrait for Periodicity {
    fn to_text(&self) -> &str {
        match self {
            Periodicity::Day => "ďeň",
            Periodicity::Week => "týždeň",
            Periodicity::Month => "mesiac",
            Periodicity::Year => "rok",
        }
    }
}
