use num_enum::TryFromPrimitive;
use serde::{Deserialize, Serialize};
use std::fmt;

#[repr(u8)]
#[derive(Debug, Clone, Serialize, Deserialize, TryFromPrimitive)]
pub enum ActivityType {
    Washing,
    Mopping,
    Cleaning,
    Vacuuming,
    Dusting,
    GarbageDisposal,
    Other,
}
impl From<i32> for ActivityType {
    fn from(item: i32) -> Self {
        ActivityType::try_from(item as u8).expect("Unknown criticality type")
    }
}
impl fmt::Display for ActivityType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
#[repr(u8)]
#[derive(Debug, Clone, Serialize, Deserialize, TryFromPrimitive)]
pub enum CriticalityType {
    Low = 0,
    Normal = 1,
    High = 2,
}
impl From<i32> for CriticalityType {
    fn from(item: i32) -> Self {
        CriticalityType::try_from(item as u8).expect("Unknown criticality type")
    }
}
impl fmt::Display for CriticalityType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
#[repr(u8)]
#[derive(Debug, Clone, Serialize, Deserialize, TryFromPrimitive)]
pub enum AppUserRole {
    Administrator = 0,
    User = 1,
}
impl From<i32> for AppUserRole {
    fn from(item: i32) -> Self {
        AppUserRole::try_from(item as u8).expect("Unknown user role")
    }
}
impl fmt::Display for AppUserRole {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
#[repr(u8)]
#[derive(Debug, Clone, Serialize, Deserialize, TryFromPrimitive)]
pub enum RoomType {
    Bathroom,
    Bedroom,
    LivingRoom,
    Kitchen,
    Balcony,
    WorkRoom,
    Garage,
    Cellar,
    Other,
}
impl From<i32> for RoomType {
    fn from(item: i32) -> Self {
        RoomType::try_from(item as u8).expect("Unknown room role")
    }
}
impl fmt::Display for RoomType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
