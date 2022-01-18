mod pd;

use axum::{extract::Path, handler::get, http::StatusCode, Router};
use percent_encoding::percent_decode_str;
use serde::Deserialize;
use std::convert::TryInto;
use std::error::Error;

// 引入 protobuf 生成的代码，我们暂且不用太关心他们

use crate::pd::ImageSpec;

// 参数使用 serde 做 Deserialize，axum 会自动识别并解析
#[derive(Deserialize)]
struct Params {
    spec: String,
    url: String,
}

// #[tokio::main]
// async fn main() {

//     // 构建路由
//     let app = Router::new()
//         // `GET /image` 会执行 generate 函数，并把 spec 和 url 传递过去
//         .route("/image/:spec/:url", get(generate));
//     // 运行 web 服务器
//     let addr = "127.0.0.1:9000".parse().unwrap();
//     tracing::debug!("listening on {}", addr);
//     println!("listening on {}", addr);
//     axum::Server::bind(&addr)
//         .serve(app.into_make_service())
//         .await
//         .unwrap();
//     println!("...................")
// }

///
/// D:\workspace\language\github\rust-learn\target\debug>http_cli.exe get http://127.0.0.1:3000/image/CgoKCAjYBBCgBiADCgY6BAgUEBQKBDICCAM/images.pexels.com
/// [34mHTTP/1.1 200 OK[0m
///
/// [32mcontent-type[0m: "text/plain"
/// [32mcontent-length[0m: "807"
/// [32mdate[0m: "Tue, 18 Jan 2022 03:51:15 GMT"
///
/// url: images.pexels.com
///  spec: ImageSpec {
///     specs: [
///         Spec {
///             data: Some(
///                 Resize(
///                     Resize {
///                         width: 600,
///                         height: 800,
///                         rtype: Normal,
///                         filter: CatmullRom,
///                     },
///                 ),
///             ),
///         },
///         Spec {
///             data: Some(
///                 Watermark(
///                     Watermark {
///                         x: 20,
///                         y: 20,
///                     },
///                 ),
///             ),
///         },
///         Spec {
///             data: Some(
///                 Filter(
///                     Filter {
///                         filter: Marine,
///                     },
///                 ),
///             ),
///         },
///     ],
/// }
///
#[tokio::main]
async fn main() {
    // 初始化 tracing
    tracing_subscriber::fmt::init();
    // build our application with a single route
    let app = Router::new()
        .route("/image/:spec/:url", get(generate))
        .route("/", get(|| async { "Hello, World!" }));

    // 运行 web 服务器
    let addr = "127.0.0.1:3000".parse().unwrap();
    tracing::debug!("listening on {}", addr);
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// 目前我们就只把参数解析出来
async fn generate(Path(Params { spec, url }): Path<Params>) -> Result<String, ()> {
    println!(".......{}", &url);
    let url = percent_decode_str(&url).decode_utf8_lossy();
    let spec: ImageSpec = spec
        .as_str()
        .try_into()
        .map_err(|_| StatusCode::BAD_REQUEST)
        .unwrap(); //这个地方先 暴力处理一下
    Ok(format!("url: {}\n spec: {:#?}", url, spec))
}

// fn main() {
//     println!("Hello, world!");
// }
