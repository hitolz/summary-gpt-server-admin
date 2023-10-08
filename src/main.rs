use actix_web::App;
use actix_web::HttpServer;
use log::info;
use summary_gpt_server_admin::api;
use summary_gpt_server_admin::db;
use summary_gpt_server_admin::error::Result;
use summary_gpt_server_admin::setting;

#[actix_web::main]
async fn main() -> Result<()> {
    setting::log_init();
    let conn_string = setting::get_conn_string();
    info!("conn_string:{}", conn_string);
    db::init_connections(conn_string.as_str()).await?;

    let config = &*setting::SETTING;
    let app = &config.app;
    info!("server listening at http://{}:{}", app.host, app.port);

    HttpServer::new(move || {
        App::new()
            .service(api::routes())
    })
    .bind((app.host.as_str(), app.port))?
    .run()
    .await?;

    Ok(())
}
