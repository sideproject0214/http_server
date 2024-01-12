// 우리가 트레이트로부터 함수를 호출하기 전에 트레이트를 우리 범위로 가져와야 한다.

use std::convert::TryFrom;

// 여기서 crate 키워드를 사용한다는 것은 전체 크레이트의 루트를 의미한다.
use crate::http::{request, ParseError, Request, Response, StatusCode};
use std::io::{Read, Write};
use std::net::TcpListener;
// Read, Write 트레이트는 Rust에서 IO 연산의 중심에 있다.
pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;
    // Handler 구현자는 이것을 구현하길 원하지 않을 수 있다. 왜냐하면 상당히 제네릭 하기 때문이다.
    fn handle_bad_request(&mut self, e: &ParseError) -> Response {
        println!("Failed to parse request: {}", e);
        Response::new(StatusCode::BadRequest, None)
    }
}

pub struct Server {
    addr: String,
}

// 우리가 항상 array에 원소가 몇 개나 있을지 항상 알아야 한다면 그것은 힘든일이다.
// // 그럴때 이렇게 참조로 하게 되면
// // fn arr(a: [u8;5]){} 이런식으로 크기를 안 정해줘도 된다.
// //  그리고 아래와 같은 참조를 슬라이스라고 부른다. 문자열 슬라이스는 u8 슬라이스에 비해 추가적인 기능이 있는 래퍼에 불과함
// fn arr(a: &[u8]) {}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr: addr }
    }

    pub fn run(self, mut handler: impl Handler) {
        println!("Listening on {}", self.addr);
        // Result가 Ok이면, OK가 감싸고 있는 값을 리턴한다.
        // 하지만 Result가 Err라면 프로그램을 종료하고 오류를 화면에 로깅할 것이다.
        let listener = TcpListener::bind(&self.addr).unwrap();
        // 다른 언어처럼 break를 써서 반복문에서 나갈수 있고, continue를 써서 반복문의 다음 반복으로 넘어갈수 있다.
        // 안쪽 loop의 본문에서 바깥쪽 loop를 break 하려면 '레이블'을 이용해 loop에 주석을 달 수 있다.
        //
        // 'outer: loop {
        //     loop{
        //         break 'outer; 또는
        //         continue 'outer;
        //     }
        // }
        //

        loop {
            // 이것은 복구 가능한 오류로써 연결 하나에 실패하면 다음 연결을 시도해야 한다.
            // 아래 처럼 작성할 수 있지만, enum을 활용하는 좋은 예제는 아니다.
            // let res = listener.accept();
            // if res.is_err() {
            //     continue;
            // }

            // let (stream, addr) = res.unwrap();
            // 실행하려는 코드가 단일 구문이라면 중괄호를 쓰지 않고 바로 적을 수 있다.
            match listener.accept() {
                Ok((mut stream, addr)) => {
                    // let a = [1, 2, 3, 4, 5, 5, 5, 5, 5]; // array의 구체적인 타입은 항상 그 안에 있는 값의 타입과
                    // 거기 포함된 값의 개수를 더한 것. 왜냐하면 컴파일일러가 array가 얼마나 큰지 알아야 하기 때문
                    // 만약 Ok에서 Result 사용하길 원치 안으면 _로 무시할 수 있다.
                    // enum 요소에 매칭할때 그것들의 값을 연결할 수 있다.

                    // traits에서 가능을 사용하려면 먼저 그걸 우리의 범위로 가져와야 한다.
                    // stream.read();

                    // arr(&a[1..3]);
                    let mut buffer = [0; 1024]; // 요소들의 값이 똑같은 어레이를 생성하기 위한 구문
                                                // 1킬로바이트 약간 넘는다. 1024개의 u8로 저장하기에 u8 1개당 1바이트임(8비트)
                                                // C의 경우에는 1024 바이트로 된 어레이를 만들려면 우린 그 공간을 담을 수 있는 큰 공간을 받는다.
                                                // 이때 array 내용에는 이전에 이 주소에 있던 랜덤 메모리를 포함하고 있다.
                                                // 우리가 그 메모리를 다룰려면 메모리 손상으로 인해 프로그램이 충돌하게 된다.
                                                // 러스트에서는 그것을 방지하기 위해 사용하기 전에 모든 메모리를 초기화 하는 방식으로 그것을 방지함
                                                // 따라서 이 경우 1024 바이트 저장가능한 충분한 메모리 덩어리를 제공하지만, 우리가 사용하기 전에 모든 메모리가 초기화 된다

                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            // 우리는 버퍼를 실제 텍스트로 변환해서 그걸 화면에 프린트하고 필요하다면 디버깅을 해야 한다.
                            // from_utf8() 이라는 함수는 바이트가 담긴 버퍼를 파라미터로 예상하고 그 바이트는 유효한 utf-8이어야 한다.
                            // 이것은 Result를 반환하는데 왜냐하면 여기에 유효하지 않은 utf-8 바이트가 포함되어 있을 수 있기 때문이다.
                            // from_utf8_lossy는 유효하지 않은 바이트까지 포함해서 변환시킨다.
                            println!("Receiced a request: {}", String::from_utf8_lossy(&buffer));
                            // Request::try_from(&buffer as &[u8]); 아래와 동일하다. 우리는 어레이 전체가 담긴 슬라이스를 원하기에
                            // 하단과 상단 경계선을 생략하고 그냥 .. 라고만 적으면 어레이 전체가 담긴 바이트 슬라이스가 생성된다.
                            let response = match Request::try_from(&buffer[..]) {
                                Ok(request) => {
                                    // 라이프 타임을 주게 되면 위 try_from에서 이미 버퍼는 사용이 끝났기에
                                    // 다음과 같이 수정하더라도 문제가 없기에 다음은 허용이 된다.
                                    // buffer[1] = 0;
                                    // 하지만 아래와 같이 다시 이를 할당한다면 버퍼사용이 끝나지 않았기에 위의 수정은 허용되지 않는다.
                                    // let a = request;
                                    // dbg!(request);
                                    // 아래는 NotFound 이므로 반환할 것이 아무것도 없으므로 None을 반환한다.
                                    // let response = Response::new(
                                    //     StatusCode::Ok,
                                    //     Some("<h1>It works!!!</h1>".to_string()),
                                    // );
                                    // let response = Response::new(StatusCode::NotFound, None);
                                    // write!(stream, "HTTP/1.1 404 Not Found\r\n\r\n");
                                    // response.send(&mut stream);
                                    handler.handle_request(&request)
                                }
                                Err(e) => {
                                    // println!("Failed to parse a request {}", e);
                                    handler.handle_bad_request(&e)
                                }
                            };

                            if let Err(e) = response.send(&mut stream) {
                                println!("Failed to send response : {}", e)
                            }

                            // let res: &Result<Request, _> = &buffer[..].try_into();
                            // 바이트 슬라이스에 대한 TryInto 구현체는 많을 수 있으니 그냥은 실행안되고
                        }
                        Err(e) => println!("Failed to read from connection : {}", e),
                    };
                    // read는 우리를 기다리고 있는 소켓에서 바이트르를 모두 읽어낸다. 그리고 그걸 buffer 안에 복사한다
                    // 프로덕션 서버에서는 훨씬더 스마트해야한다. 왜냐하면 버퍼가 너무 작아서 데이터의 일부만 읽으면 안되기 때문이다
                }
                // _ => 이하 사례에 대해 신겨읏지 않는다면 여기에 _을 넣으면 만능 패턴 역할을 해서 대응해
                // 수작업으로 매칭시 키지 않은 모든 요소를 잡아 줄 것이다.
                Err(e) => println!("ERR {}", e),
            }

            // enum 뿐만 아니라 일반 switch 구문으로도 활용할 수 있다.
            // match "abcd" {
            //     "abd" => {},
            //     "a" | "b" => {},
            //     _ => {}
            // }
        }
    }
}
