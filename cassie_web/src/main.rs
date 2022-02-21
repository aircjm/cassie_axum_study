use axum::{extract::extractor_middleware, response::IntoResponse, routing::get, Router};
use cassie_common::RespVO;
use cassie_web::{
    config::{log::init_log, CASSIE_CONFIG},
    middleware::auth::Auth,
    routers::{admin, api}
};
use log::info;
use tokio::time;

pub async fn index() -> impl IntoResponse {
    RespVO::from(&"hello world".to_string()).resp_json()
}

/**
 *method:main
 *desc:程序主入口方法 admin 管理端api api:小程序,h5,app使用
 *author:String
 *email:348040933QQ.com
 */
#[tokio::main]
async fn main() {
    init_log();
    info!(
        " - Local:   http://{}",
        CASSIE_CONFIG.server.replace("0.0.0.0", "127.0.0.1")
    );
    //绑定端口 初始化 路由
    let app = Router::new()
        .route("/index", get(index))
        .nest(
            "/admin",
            admin::routers().layer(extractor_middleware::<Auth>()),
        )
        .nest("/api", api::routers());
        tokio::spawn(print());
    axum::Server::bind(&CASSIE_CONFIG.server.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
        
       
}
async fn print() {
    let mut interval = time::interval(time::Duration::from_secs(1));
    loop {
        interval.tick().await;
        println!("2333");
    }
}