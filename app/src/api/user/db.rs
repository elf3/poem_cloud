use super::structures::{CreateUserRequest, LoginRequest};
use crate::models::{prelude::User, user};
use anyhow::{anyhow, Result};
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::Utc;
use library::jwt::{self, Claims};
use sea_orm::{
    entity::prelude::*,
    ActiveValue::{NotSet, Set},
    ColumnTrait, EntityTrait, QueryFilter,
};
pub async fn login(db: &DatabaseConnection, req: LoginRequest) -> Result<String> {
    let model = User::find().filter(user::Column::Username.eq(req.username.clone()));
    let data: Option<user::Model> = model.one(db).await?;
    let res = match data {
        Some(val) => val,
        None => return Err(anyhow!("账号或密码错误")),
    };
    match verify(req.password, &res.password)? {
        true => {
            let claims = jwt::Claims::new(res.username, res.id);
            let token = jwt::create_jwt(claims);
            let tokenstr = match token {
                Ok(val) => val,
                Err(_) => return Err(anyhow!("gen token error")),
            };
            Ok(tokenstr)
        }
        false => Err(anyhow!("账号或密码错误")),
    }
}

pub async fn logout(db: &DatabaseConnection, user: Claims) -> Result<()> {
    let model = User::find().filter(user::Column::Username.eq(user.username));
    let data: Option<user::Model> = model.one(db).await?;
    let res = match data {
        Some(val) => val,
        None => return Err(anyhow!("账号或密码错误")),
    };

    Ok(())
}

pub async fn get_userinfo(db: &DatabaseConnection, user: Claims) -> Result<user::Model> {
    let model = User::find().filter(user::Column::Id.eq(user.user_id));
    let data: Option<user::Model> = model.one(db).await?;
    let res = match data {
        Some(val) => val,
        None => return Err(anyhow!("用户已禁用或不存在")),
    };
    Ok(res)
}

pub async fn create_user(db: &DatabaseConnection, req: CreateUserRequest) -> Result<String> {
    let find: Option<user::Model> = User::find()
        .filter(user::Column::Username.eq(req.username.clone()))
        .one(db)
        .await?;
    match find {
        Some(_) => return Err(anyhow!("该账号已存在")),
        None => {
            let user = user::ActiveModel {
                id: NotSet,
                username: Set(req.username.to_owned()),
                password: Set(hash(&req.password, DEFAULT_COST).unwrap()),
                status: Set(1.to_string()),
                created_at: Set(Utc::now()),
            };
            user.insert(db).await?;
            Ok("".to_string())
        }
    }
}
