use rbatis::rbdc::datetime::DateTime;
use serde::{Deserialize, Serialize};

use crate::client::entity::auth_site::AuthSite;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AddAuthSite {
    pub id: Option<u64>,
    pub user_id: Option<u64>,
    pub site_domain: Option<String>,
    pub site_summary_key: Option<String>,
}

impl AddAuthSite {
    pub fn new() -> AddAuthSite {
        AddAuthSite {
            id: None,
            user_id: None,
            site_domain: None,
            site_summary_key: None,
        }
    }
}

impl Into<AuthSite> for AddAuthSite {
    fn into(self) -> AuthSite {
        AuthSite {
            id: None,
            user_id: self.user_id,
            site_domain: self.site_domain,
            site_summary_key: self.site_summary_key,
            active: Some(1),
            created_time: Some(DateTime::now()),
            updated_time: Some(DateTime::now()),
        }
    }
}
