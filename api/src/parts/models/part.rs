use crate::shared::file::File;
use crate::shared::money::Money;
use crate::shared::{CustomerId, PartId, PartQuoteId, ProjectId, QuoteId};
use chrono::{DateTime, Days, Utc};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_enum_str::{Deserialize_enum_str, Serialize_enum_str};
use std::fmt::{Display, Formatter};
use uuid::{ContextV7, Timestamp, Uuid};

static PART_QUOTE_VALID_DAYS: u64 = 30;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Part {
    pub id: PartId,
    pub customer_id: CustomerId,
    pub project_id: ProjectId,
    pub quotation_id: QuoteId,
    pub model_file: File,
    pub render_file: Option<File>,
    pub drawing_file: Option<File>,
    pub process: PartProcess,
    pub attributes: PartAttributes,
    pub quantity: u64,
    pub selected_part_quote_id: Option<PartQuoteId>,
    pub part_quotes: Option<Vec<PartQuote>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Part {
    pub fn new(
        customer_id: String,
        project_id: String,
        quotation_id: String,
        process: PartProcess,
        attributes: PartAttributes,
        model_file: File,
    ) -> Self {
        let now = Utc::now();
        let id = Uuid::new_v7(Timestamp::now(ContextV7::new()));
        let encoded_id = format!("part_{}", bs58::encode(id).into_string());

        Self {
            id: encoded_id,
            customer_id,
            project_id,
            quotation_id,
            model_file,
            render_file: None,
            drawing_file: None,
            process,
            attributes,
            quantity: 1,
            selected_part_quote_id: None,
            part_quotes: None,
            created_at: now,
            updated_at: now,
        }
    }
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
    pub id: PartQuoteId,
    pub unit_price: Money,
    pub sub_total: Money,
    pub workdays_to_complete: u64,
    pub valid_until: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl PartQuote {
    pub fn new(unit_price: Money, sub_total: Money, workdays_to_complete: u64) -> Self {
        let now = Utc::now();
        let id = Uuid::new_v7(Timestamp::now(ContextV7::new()));
        let encoded_id = format!("pq_{}", bs58::encode(id).into_string());
        let valid_until = now
            .checked_add_days(Days::new(PART_QUOTE_VALID_DAYS))
            .expect("error creating `valid_until` date for part quote.");

        Self {
            id: encoded_id,
            unit_price,
            sub_total,
            workdays_to_complete,
            valid_until,
            created_at: now,
            updated_at: now,
        }
    }
}
