pub mod entity;
pub mod model;
pub mod service;

#[cfg(test)]
mod user_tests {

    use log::info;

    use crate::{
        client::{
            entity::{auth_site::AuthSite, user::User},
            model::{user_model::AddUser, auth_site_model::AddAuthSite},
        },
        db, setting,
        utils::uuid,
    };

    async fn init() {
        setting::log_init();
        let conn_string = setting::get_conn_string();
        info!("{}", conn_string);
        db::init_connections(conn_string.as_str()).await.unwrap();
    }

    #[tokio::test]
    async fn test_add_user() {
        init().await;
        let mut add_user = AddUser::new();
        add_user.account = Some("tes1t_add".to_string());
        add_user.password = Some("1234567".to_string());
        add_user.summary_key = Some(uuid::new_summary_key());

        let x = User::add_user(add_user).await;
        println!();
        info!("{:?}", x);
    }

    #[tokio::test]
    async fn test_find_by_account() {
        init().await;
        let user = User::find_by_account("test_add").await;
        println!();
        info!("{:?}", user);
    }

    #[tokio::test]
    async fn test_find_all_user() {
        init().await;
        let vec = User::all().await.unwrap();
        println!();
        info!("{:?}", vec);
    }

    #[tokio::test]
    async fn test_add_auth_site() {
        init().await;
        let mut add = AddAuthSite::new();
        add.site_domain = Some("www.baidu.com".to_string());
        add.user_id = Some(1);
        add.site_summary_key = Some(uuid::new_summary_key());
        let x =AuthSite::add_auth_site(add).await;
        info!("{:?}",x);
    }


    #[tokio::test]
    async fn test_find_by_user_id(){
        init().await;
        let sites = AuthSite::find_active_by_user_id(1).await.unwrap();
        info!("all active sites found {:?}",sites);

        let sites = AuthSite::find_by_user_id(1).await.unwrap();
        info!("all  sites found {:?}",sites);
    }
}
