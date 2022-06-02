use chrono::{Duration, Utc};
use anyhow::{Result, anyhow};
use library::jwt::{Claims, self};
use sea_orm::{DatabaseConnection, QueryFilter, ColumnTrait,EntityTrait, PaginatorTrait};
use super::structures::LoginRequest;
use crate::models::{
    prelude::Admin, admin,
};

pub async fn login(db: &DatabaseConnection, req: LoginRequest) -> Result<String> {
    let model = Admin::find().filter(admin::Column::Username.eq( req.username.clone()));
    let data:Option<admin::Model> = model.one(db).await?;
    let res = match data {
        Some(val) => val,
        None => return  Err(anyhow!("账号或密码错误")),
    };

    let claims = jwt::Claims::new(req.username);
    let token = jwt::create_jwt(claims);
    let tokenstr = match token {
        Ok(val) => val,
        Err(e) => return Err(anyhow!("gen token error")),
    };
    Ok(tokenstr)
}


pub async fn logout(db: &DatabaseConnection, user:Claims) -> Result<()> {
    let model = Admin::find().filter(admin::Column::Username.eq(user.username));
    let data:Option<admin::Model> = model.one(db).await?;
    let res = match data {
        Some(val) => val,
        None => return  Err(anyhow!("账号或密码错误")),
    };
    
    Ok(())
}