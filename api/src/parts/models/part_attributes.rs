use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_enum_str::{Deserialize_enum_str, Serialize_enum_str};
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, PartialEq)]
pub enum PartAttributes {
    CNC(CNCAttributes),
}

impl Default for PartAttributes {
    fn default() -> Self {
        Self::CNC(CNCAttributes::default())
    }
}

impl Serialize for PartAttributes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            PartAttributes::CNC(attrs) => attrs.serialize(serializer),
        }
    }
}

impl<'de> Deserialize<'de> for PartAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let attrs = CNCAttributes::deserialize(deserializer)?;
        Ok(PartAttributes::CNC(attrs))
    }
}

impl Display for PartAttributes {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PartAttributes::CNC(attr) => write!(f, "{}", attr),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CNCAttributes {
    pub material: String,
    pub tolerance: Tolerance,
}

impl Default for CNCAttributes {
    fn default() -> Self {
        Self {
            material: String::from("Aluminum 6061-T6"),
            tolerance: Tolerance::PlusMinus005Inch013mm,
        }
    }
}

impl Display for CNCAttributes {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Process: CNC, Material: {}, Tolerance: {})",
            self.material, self.tolerance
        )
    }
}

#[derive(Serialize_enum_str, Deserialize_enum_str, Clone, Debug, PartialEq)]
pub enum Tolerance {
    #[serde(rename = "+/- .005\" (+/- 0.13mm)")]
    PlusMinus005Inch013mm,
    Other,
}
