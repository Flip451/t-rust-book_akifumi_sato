use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgRow, FromRow, Row};
use uuid::Uuid;
use validator::Validate;

use crate::repositories::labels::ILabelRepository;

// label
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Label {
    id: LabelId,
    name: LabelName,
}

impl<'r> FromRow<'r, PgRow> for Label {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        let id = row.try_get("id")?;
        let name = LabelName::from_row(row)?;
        Ok(Self { id, name })
    }
}

impl Label {
    pub fn new(name: LabelName) -> Self {
        let id: LabelId = Uuid::new_v4();
        Self { id, name }
    }

    pub fn get_id(&self) -> &LabelId {
        &self.id
    }

    pub fn get_name(&self) -> &LabelName {
        &self.name
    }

    pub fn set_name(&mut self, new_text: LabelName) {
        self.name = new_text;
    }

    pub fn is_duplicated(&self) {}
}

impl PartialEq for Label {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Label {}

// LabelId
pub type LabelId = Uuid;

// labelName
#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct LabelName {
    #[validate(length(min = 1, message = "Can not be empty"))]
    #[validate(length(max = 15, message = "Over text length"))]
    value: String,
}

impl<'r> FromRow<'r, PgRow> for LabelName {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        let text = row.try_get("name")?;
        Ok(LabelName { value: text })
    }
}

impl LabelName {
    pub fn new(s: &str) -> Self {
        Self {
            value: s.to_string(),
        }
    }
}

impl ToString for LabelName {
    fn to_string(&self) -> String {
        self.value.clone()
    }
}

// domain service
pub struct LabelDomainService<T>
where
    T: ILabelRepository,
{
    repository: T,
}

impl<T> LabelDomainService<T>
where
    T: ILabelRepository,
{
    pub async fn is_duplicated(&self, label: Label) -> Result<bool> {
        if let Some(label_found) = self.repository.find_by_name(label.get_name()).await? {
            Ok(label_found != label)
        } else {
            Ok(false)
        }
    }
}
