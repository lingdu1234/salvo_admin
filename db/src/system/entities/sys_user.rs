//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.1

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "sys_user"
    }
}

#[derive(Clone, Debug, PartialEq, DeriveModel, DeriveActiveModel, Eq, Serialize, Deserialize)]
pub struct Model {
    pub id: String,
    pub user_name: String,
    pub user_nickname: String,
    pub user_password: String,
    pub user_salt: String,
    pub user_status: String,
    pub user_email: Option<String>,
    pub sex: String,
    pub avatar: String,
    pub role_id: String,
    pub dept_id: String,
    pub remark: Option<String>,
    pub is_admin: String,
    pub phone_num: Option<String>,
    pub last_login_ip: Option<String>,
    pub last_login_time: Option<DateTime>,
    pub created_at: DateTime,
    pub updated_at: Option<DateTime>,
    pub deleted_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    Id,
    UserName,
    UserNickname,
    UserPassword,
    UserSalt,
    UserStatus,
    UserEmail,
    Sex,
    Avatar,
    RoleId,
    DeptId,
    Remark,
    IsAdmin,
    PhoneNum,
    LastLoginIp,
    LastLoginTime,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
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
            Self::UserName => ColumnType::String(Some(60u32)).def().unique(),
            Self::UserNickname => ColumnType::String(Some(50u32)).def(),
            Self::UserPassword => ColumnType::String(Some(32u32)).def(),
            Self::UserSalt => ColumnType::Char(Some(10u32)).def(),
            Self::UserStatus => ColumnType::Char(Some(1u32)).def(),
            Self::UserEmail => ColumnType::String(Some(100u32)).def().null(),
            Self::Sex => ColumnType::Char(Some(1u32)).def(),
            Self::Avatar => ColumnType::String(Some(255u32)).def(),
            Self::RoleId => ColumnType::String(Some(32u32)).def(),
            Self::DeptId => ColumnType::String(Some(32u32)).def(),
            Self::Remark => ColumnType::String(Some(255u32)).def().null(),
            Self::IsAdmin => ColumnType::Char(Some(1u32)).def(),
            Self::PhoneNum => ColumnType::String(Some(20u32)).def().null(),
            Self::LastLoginIp => ColumnType::String(Some(15u32)).def().null(),
            Self::LastLoginTime => ColumnType::DateTime.def().null(),
            Self::CreatedAt => ColumnType::DateTime.def(),
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
