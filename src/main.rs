#![allow(dead_code)]
// !의 속성이 그 안에 선언된 아이템에 적용될 것이라는 의미이다.
// 따라서 main 모듈안에 선언되었기에 전체 모듈과 서브 모듈에 적용이 된다.
// !을 안붇이면 그 뒤에 있는 식에만 속성이 적용된다는 의미이다.
use server::Server;
use std::env;
use website_handler::WebsiteHandler;

mod http;
mod server;
mod website_handler;
fn main() {
    // env! : 컴파일링 할때 환경 변수를 읽는데 사용한다.
    // CARGO_MANIFEST_DIR 이라고 하면 Cargo.toml 파일이 있는 위치를 기준으로 한다.
    let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
    println!("public path : {}", public_path);
    let server = Server::new("127.0.0.1:4000".to_string());
    server.run(WebsiteHandler::new(public_path));
}
