use crate::{client::model::user_model::AddUser, db};
use log::info;
use rbatis::{impl_select, rbdc::datetime::DateTime};
use serde::{Deserialize, Serialize};

use crate::error::Result as MyResult;

#[derive(sqlx::FromRow, Debug, Clone, Default, Serialize, Deserialize)]
pub struct User {
    pub id: Option<u64>,
    pub account: Option<String>,
    pub password: Option<String>,

    pub tokens: Option<u64>,
    pub summary_key: Option<String>,
    pub openai_key: Option<String>,

    pub active: Option<u64>,
    pub created_time: Option<DateTime>,
    pub updated_time: Option<DateTime>,
}

rbatis::crud!(User {}, "user");
impl_select!(User{select_one_by_account(account:&str) -> Option => "`where account = #{account}`"});

impl User {
    pub fn new() -> User {
        User {
            id: None,
            account: None,
            password: None,
            tokens: None,
            summary_key: None,
            openai_key: None,
            active: None,
            created_time: None,
            updated_time: None,
        }
    }

    pub async fn add_user(add_user: AddUser) -> MyResult<()> {
        info!("add user: {:?}", add_user);
        let user = add_user.into();
        User::insert(&mut db::get_rb(), &user).await?;
        Ok(())
    }

    pub async fn find_by_account(account: &str) -> MyResult<Option<User>> {
        let x = User::select_one_by_account(&mut db::get_rb(), account).await?;
        Ok(x)
    }

    pub async fn all() -> MyResult<Vec<User>> {
        let x = User::select_all(&mut db::get_rb()).await?;
        Ok(x)
    }
}
