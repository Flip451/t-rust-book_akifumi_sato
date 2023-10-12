use uuid::Uuid;

use crate::domain::entity::Entity;
use crate::domain::value_object::ValueObject;

use super::label_id::LabelId;
use super::label_name::LabelName;

// entity
#[derive(Debug, Clone, Eq, Hash)]
pub struct Label {
    label_id: LabelId,
    pub label_name: LabelName,
}

impl Label {
    pub fn new(label_name: LabelName) -> anyhow::Result<Self> {
        let label_id = LabelId::new(Uuid::new_v4())?;
        Ok(Self { label_id, label_name })
    }

    pub fn build(label_id: LabelId, label_name: LabelName) -> Self {
        Self { label_id, label_name }
    }

    pub fn label_id(&self) -> &LabelId {
        &self.label_id
    }
}

impl Entity for Label {
    type Identity = LabelId;

    fn identity(&self) -> &Self::Identity {
        &self.label_id
    }
}

impl PartialEq for Label {
    fn eq(&self, other: &Self) -> bool {
        Entity::eq(self, other)
    }
}
