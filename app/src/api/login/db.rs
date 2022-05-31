use anyhow::{Result, anyhow, Ok};
use sea_orm::{DatabaseConnection, QueryFilter, ColumnTrait,EntityTrait, PaginatorTrait};
use super::structures::LoginRequest;
use crate::models::{
    prelude::Admin, admin,
};
pub async fn login(db: &DatabaseConnection, req: LoginRequest) -> Result<admin::Model> {
    let model = Admin::find().filter(admin::Column::Username.eq(req.username));
    let data:Option<admin::Model> = model.one(db).await?;
    let res = match data {
        Some(val) => val,
        None => return  Err(anyhow!("账号或密码错误")),
    };
    Ok(res)
}

