use sqlx::{
    prelude::FromRow,
    types::{
        Uuid,
        chrono::{DateTime, Local},
    },
};

use crate::prelude::habiting_proto::Tag;

#[derive(Clone, FromRow, Debug)]
pub(super) struct TagPgRow {
    pub uuid: Uuid,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
    pub name: String,
}

impl From<TagPgRow> for Tag {
    fn from(value: TagPgRow) -> Self {
        Self {
            uuid: value.uuid.to_string(),
            created_at: value.created_at.to_string(),
            updated_at: value.updated_at.to_string(),
            name: value.name,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Add;

    use chrono::TimeDelta;

    use super::*;

    #[test]
    fn test_conversion_mapping() {
        let desc = "Basic test to ensure fields are properly remapped in type conversion.";
        let created_at = Local::now();
        let updated_at = Local::now().add(TimeDelta::days(1));

        let pg_row = TagPgRow {
            uuid: Uuid::nil(),
            created_at,
            updated_at,
            name: "tag_name".into(),
        };

        let row: Tag = pg_row.clone().into();

        assert_eq!(pg_row.uuid.to_string(), row.uuid, "uuid: {desc}");
        assert_eq!(created_at.to_string(), row.created_at, "created at: {desc}");
        assert_eq!(updated_at.to_string(), row.updated_at, "updated at: {desc}");
        assert_eq!(pg_row.name.to_string(), row.name.to_string(), "name {desc}");
    }
}
