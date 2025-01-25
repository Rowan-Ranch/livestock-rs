use clap::ValueEnum;
use serde::Serialize;

/// The type of livestock 
/// 
/// This enum represents the different types of livestock that can be used in the livestock management system.
/// This list will grow, and more types will be added in the future, as needed.
#[derive(Clone, Serialize, Debug, Eq, PartialEq, ValueEnum)]
pub enum LivestockType {
    Cattle,
    Swine,
    Chicken,
    Rabbit,
    Sheep,
    Goat
}