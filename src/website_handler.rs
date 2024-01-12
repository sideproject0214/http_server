use super::http::{Method, Request, Response, StatusCode};
use super::server::Handler;
use std::fs;

pub struct WebsiteHandler {
    public_path: String,
}

impl WebsiteHandler {
    pub fn new(public_path: String) -> Self {
        Self { public_path }
    }

    fn read_file(&self, file_path: &str) -> Option<String> {
        let path = format!("{}/{}", self.public_path, file_path);

        // ../../../../name 이런 경로를 /name으로 바꿔준다.
        match fs::canonicalize(path) {
            Ok(path) => {
                if path.starts_with(&self.public_path) {
                    fs::read_to_string(path).ok()
                } else {
                    println!("Directory Traversal Attack Attempted!");
                    None
                }
            }
            Err(_) => None,
        }
        // ok()메서드는 Result를 살펴본 다음에 ok 라면 그 값을 받고 그걸 Option으로 변환하게 된다.
        // fs::read_to_string(path).ok()
    }
}

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        match request.method() {
            Method::GET => match request.path() {
                "/" => Response::new(StatusCode::Ok, self.read_file("index.html")),
                "/hello" => Response::new(StatusCode::Ok, self.read_file("hello.html")),
                "/hello2" => Response::new(StatusCode::Ok, Some("<h1>Hello</h1>".to_string())),
                // 아래는 그대로 하면 데렉터리 횡단 취약성을 가지게 된다. 공격자는 서버가 실행되는 시스템에서 임의의 파일을 읽을수 있기 때문이다
                // path => match self.read_file(path) {
                //     Some(contents) => Response::new(StatusCode::Ok, Some(contents)),
                //     None => Response::new(StatusCode::NotFound, None),
                // },
                path => match self.read_file(path) {
                    Some(contents) => Response::new(StatusCode::Ok, Some(contents)),
                    None => Response::new(StatusCode::NotFound, None),
                },
            },
            _ => Response::new(StatusCode::NotFound, None),
        }
    }
}
