//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.1

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "sys_job"
    }
}

#[derive(Clone, Debug, PartialEq, DeriveModel, DeriveActiveModel, Eq, Serialize, Deserialize)]
pub struct Model {
    pub job_id: String,
    pub task_id: i64,
    pub task_count: i64,
    pub run_count: i64,
    pub job_name: String,
    pub job_params: Option<String>,
    pub job_group: String,
    pub invoke_target: String,
    pub cron_expression: String,
    pub misfire_policy: String,
    pub concurrent: Option<String>,
    pub status: String,
    pub create_by: String,
    pub update_by: Option<String>,
    pub remark: Option<String>,
    pub last_time: Option<DateTime>,
    pub next_time: Option<DateTime>,
    pub end_time: Option<DateTime>,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub deleted_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    JobId,
    TaskId,
    TaskCount,
    RunCount,
    JobName,
    JobParams,
    JobGroup,
    InvokeTarget,
    CronExpression,
    MisfirePolicy,
    Concurrent,
    Status,
    CreateBy,
    UpdateBy,
    Remark,
    LastTime,
    NextTime,
    EndTime,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
}

#[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
pub enum PrimaryKey {
    JobId,
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
            Self::JobId => ColumnType::String(Some(32u32)).def(),
            Self::TaskId => ColumnType::BigInteger.def(),
            Self::TaskCount => ColumnType::BigInteger.def(),
            Self::RunCount => ColumnType::BigInteger.def(),
            Self::JobName => ColumnType::String(Some(64u32)).def(),
            Self::JobParams => ColumnType::String(Some(200u32)).def().null(),
            Self::JobGroup => ColumnType::String(Some(64u32)).def(),
            Self::InvokeTarget => ColumnType::String(Some(500u32)).def(),
            Self::CronExpression => ColumnType::String(Some(255u32)).def(),
            Self::MisfirePolicy => ColumnType::Char(Some(1u32)).def(),
            Self::Concurrent => ColumnType::Char(Some(1u32)).def().null(),
            Self::Status => ColumnType::Char(Some(1u32)).def(),
            Self::CreateBy => ColumnType::String(Some(32u32)).def(),
            Self::UpdateBy => ColumnType::String(Some(32u32)).def().null(),
            Self::Remark => ColumnType::Text.def().null(),
            Self::LastTime => ColumnType::DateTime.def().null(),
            Self::NextTime => ColumnType::DateTime.def().null(),
            Self::EndTime => ColumnType::DateTime.def().null(),
            Self::CreatedAt => ColumnType::DateTime.def().null(),
            Self::UpdatedAt => ColumnType::DateTime.def().null(),
            Self::DeletedAt => ColumnType::DateTime.def().null(),
        }
    }
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No RelationDef")
    }
}

impl ActiveModelBehavior for ActiveModel {}