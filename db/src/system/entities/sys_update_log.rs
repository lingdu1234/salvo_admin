//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.1

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "sys_update_log"
    }
}

#[derive(Clone, Debug, PartialEq, DeriveModel, DeriveActiveModel, Eq, Serialize, Deserialize)]
pub struct Model {
    pub id: String,
    pub app_version: String,
    pub backend_version: String,
    pub title: String,
    pub content: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub deleted_at: Option<DateTime>,
    pub updated_by: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    Id,
    AppVersion,
    BackendVersion,
    Title,
    Content,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
    UpdatedBy,
}

#[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
pub enum PrimaryKey {
    Id,
}

impl PrimaryKeyTrait for PrimaryKey {
    type ValueType = String;
    fn auto_increment() -> bool {
        false
    }
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl ColumnTrait for Column {
    type EntityName = Entity;
    fn def(&self) -> ColumnDef {
        match self {
            Self::Id => ColumnType::String(Some(32u32)).def(),
            Self::AppVersion => ColumnType::String(Some(10u32)).def(),
            Self::BackendVersion => ColumnType::String(Some(10u32)).def(),
            Self::Title => ColumnType::String(Some(100u32)).def(),
            Self::Content => ColumnType::Text.def(),
            Self::CreatedAt => ColumnType::DateTime.def(),
            Self::UpdatedAt => ColumnType::DateTime.def(),
            Self::DeletedAt => ColumnType::DateTime.def().null(),
            Self::UpdatedBy => ColumnType::String(Some(32u32)).def(),
        }
    }
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No RelationDef")
    }
}

impl ActiveModelBehavior for ActiveModel {}
