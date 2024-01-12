use super::StatusCode;
use std::io::{Result as IoResult, Write};
use std::net::TcpStream;

#[derive(Debug)]
pub struct Response {
    status_code: StatusCode,
    // 응답에 본문이 없을 경우를 처리하기 위해 String을 Option으로 감싼다.
    body: Option<String>,
}

impl Response {
    pub fn new(status_code: StatusCode, body: Option<String>) -> Self {
        Response { status_code, body }
    }

    // dyn : dynamic dispatch에서 온 것이다.
    // 컴파일러는 이 함수가 쓰기 기능을 가진 파라미터를 받는 걸 알고 있다.
    // 이 파라미터가 Write 트레이트를 구현할 것이기 때문이다.
    // 하지만 컴파일러는 어떤 write() 함수를 호출할지를 정확히 알지 못한다. 왜냐하면 write() 함수의 실제 구현체는
    // TcpStream, 파일, 벡터 등 안에 있기 때문이다.
    // 모든 구현자는 write() 함수의 다양한 구현체를 가지고 있고 우린 이 타입들 중 어떤 것이라도
    // 이 함수에 파라미터로 넣을 수 있다.
    // 우리는 이함수에 파라미터로 넣을 수 있다. 그리고 컴파일러는 정확히 write() 함수의 어떤 구현체를 호출할지 알아야내야 한다.
    // 이렇게 트레이트의 구체적인 구현체와 트레이트 자체 간에 매핑을 형성하는 방법을 다이내믹 디스패치라고 한다.
    // 즉, 호출할 구체적인 함수 구현체가 런타임에 해결된다.
    // 트레이트의 구체적인 구현체와 트레이트 자체 간에 매핑을 형성하는 방법으로 그 매핑을 V테이블이라고 한다.
    // 우리가 어떤 구체적인 타입에 일반적인 함수를 호출할때 우리가 호출하는 함수의 주소로 점프하는 명령을 만나게 된다.
    // 그게 트레이트 타입이면 v테이블로 점프해야 우리가 원하는 구현체를 가리키는 정확한 함수 포인터로 넘어가게 되는데
    // 이렇게 되면 런타임에 해야할 이들이 많아 비용이 많이 들게 된다.
    // 이를 해결하기 위해서는 dyn이 아니라 impl를 붙이면 된다.
    // 이렇게 되면 런타임에 찾는게 아니라 컴파일할때 해결을 하고 가라는 의미가 된다.
    // 그래서 dyn Write라고 쓰면
    // 우리가 원래 의도한 대로 TcpStream을 dyn은 런타임때 찾는거고, impl를 붙이면 컴파일때 아예 찾고 컴파일이 되어버린다
    // 따라서 impl이 보다 많이 사용되고, 단점이라고 하면 컴파일할때 상황에 맞는 함수를 만들어야 되기에 컴파일이 약간 느리고
    // 많은 바이너리가 생성된다. 함수 하나가 여러번 구현되기 때문이다. 그래서 일부 임베디드 시스템에서는 문제가 된다.
    // 제일 좋은 것은 정확하게 타입을 넣어서 만드는게 좋다.

    pub fn send(&self, stream: &mut impl Write) -> IoResult<()> {
        let body = match &self.body {
            Some(b) => b,
            None => " ",
        };

        write!(
            stream,
            "HTTP/1.1 {} {}\r\nContent-Length: {}\r\n\r\n {}",
            self.status_code,
            self.status_code.reason_phrase(),
            body.len(),
            body
        )
    }
}

// 전에는 Formatter에 기록했는데 이제는 할당이 필요하지 않다.
// 왜냐하면 웹사이트를 위해 main.js 파일을 제공해야 할 수 있으니까

// impl Display for Response {
//     fn fmt(&self, f: &mut Formatter) -> FmtResult {
//         let body = match &self.body {
//             Some(b) => b,
//             None => " ",
//         };

//         write!(
//             f,
//             "HTTP/1.1 {} {}\r\n\r\n {}",
//             self.status_code,
//             self.status_code.reason_phrase(),
//             body
//         )
//     }
// }
