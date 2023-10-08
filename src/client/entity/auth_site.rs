use log::info;
use rbatis::{impl_select, rbdc::datetime::DateTime};
use serde::{Deserialize, Serialize};

use crate::{client::model::auth_site_model::AddAuthSite, db, error::Result as MyResult};

#[derive(sqlx::FromRow, Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuthSite {
    pub id: Option<u64>,
    pub user_id: Option<u64>,
    pub site_domain: Option<String>,
    pub site_summary_key: Option<String>,

    pub active: Option<u64>,
    pub created_time: Option<DateTime>,
    pub updated_time: Option<DateTime>,
}

rbatis::crud!(AuthSite {});
impl_select!(AuthSite{select_active_by_user_id(user_id:u64) => "`where user_id = #{user_id} and active = 1`"});

impl AuthSite {
    pub async fn new() -> AuthSite {
        AuthSite {
            id: None,
            user_id: None,
            site_domain: None,
            site_summary_key: None,
            active: None,
            created_time: Some(DateTime::now()),
            updated_time: Some(DateTime::now()),
        }
    }

    pub async fn add_auth_site(add_model: AddAuthSite) -> MyResult<()> {
        let auth_site = add_model.into();
        info!("add auth site: {:?}", auth_site);
        AuthSite::insert(&mut db::get_rb(), &auth_site).await?;
        Ok(())
    }

    pub async fn find_active_by_user_id(user_id: u64) -> MyResult<Vec<AuthSite>> {
        let x = AuthSite::select_active_by_user_id(&mut db::get_rb(), user_id).await?;
        Ok(x)
    }
    pub async fn find_by_user_id(user_id: u64) -> MyResult<Vec<AuthSite>> {
        let x = AuthSite::select_by_column(&mut db::get_rb(), "user_id", user_id).await?;
        Ok(x)
    }
}
