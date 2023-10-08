use rbatis::rbdc::datetime::DateTime;
use serde::{Deserialize, Serialize};

use crate::{client::entity::user::User, utils::uuid};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AddUser {
    pub id: Option<u64>,
    pub account: Option<String>,
    pub password: Option<String>,

    pub tokens: Option<u64>,
    pub summary_key: Option<String>,
    pub openai_key: Option<String>,
}
impl AddUser {
    pub fn new() -> AddUser {
        AddUser {
            id: None,
            account: None,
            password: None,
            tokens: Some(0),
            summary_key: None,
            openai_key: None,
        }
    }
}

impl Into<User> for AddUser {
    fn into(self) -> User {
        User {
            id: self.id,
            account: self.account,
            password: self.password,
            tokens: self.tokens,
            summary_key: Some(uuid::new_summary_key()),
            openai_key: self.openai_key,
            active: Some(1),
            created_time: Some(DateTime::now()),
            updated_time: Some(DateTime::now()),
        }
    }
}
