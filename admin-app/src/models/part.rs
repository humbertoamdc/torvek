use crate::models::file::File;
use crate::models::money::Money;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_enum_str::{Deserialize_enum_str, Serialize_enum_str};
use std::fmt::{Display, Formatter};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Part {
    pub id: String,
    pub customer_id: String,
    pub project_id: String,
    pub quotation_id: String,
    pub model_file: File,
    pub render_file: File,
    pub drawing_file: Option<File>,
    pub process: PartProcess,
    pub attributes: PartAttributes,
    pub quantity: u64,
    pub additional_notes: String,
    pub selected_part_quote_id: Option<String>,
    pub part_quotes: Option<Vec<PartQuote>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize_enum_str, Deserialize_enum_str, Clone, Debug, PartialEq)]
pub enum PartProcess {
    CNC,
}

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
    pub tolerance: String,
}

impl Default for CNCAttributes {
    fn default() -> Self {
        Self {
            material: String::from("Aluminum 6061-T6"),
            tolerance: String::from("+/- .005\" (+/- 0.13mm)"),
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

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PartQuote {
    pub id: String,
    pub unit_price: Money,
    pub sub_total: Money,
    pub workdays_to_complete: u64,
    pub valid_until: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
