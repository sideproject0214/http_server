use super::QueryString;
use super::{method::MethodError, Method};
use std::convert::TryFrom;
use std::error::Error;
use std::str;
use std::str::Utf8Error;
// Result는 이미 모든 범위에서 import 되기에 여기에서는 as를 사용해 별칭을 만든다.

use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

// 문자열 슬라이스 대신에 String을 저장하는게 어떤 단점이 있을까?
// 불필요한 힙에 복사를 해버린다. 장점을 소유권을 가지기에 변화시킬 수 있다는 것인데
// 우리는 그대로 받아 들일 것이므로 문자열 슬라이스로 바꿔준다.

// 구조체에 수명을 사용하려면 구조체를 제네릭하게 만들어야 한다.
// 수명에 대해서 제네릭할 것이다.
// Request의 수명은 buffer의 수명과 같다.
// Request를 제네릭하게 바꿔주었기에 나머지도 모두 바꿔줘야 한다.

#[derive(Debug)]
pub struct Request<'buf> {
    path: &'buf str,
    // Request에 Debug를 구현했으면 그 아래도 Debug를 구현해야 하는데, 구현안해주면 에러표시가 나온다.
    query_string: Option<QueryString<'buf>>,
    method: Method,
}

// 러스트 규약에 따르면 게터의 이름은 필드 앞에 get 이라는 단어를 쓰지 않고 필드 위에 써야 한다.
impl<'buf> Request<'buf> {
    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn method(&self) -> &Method {
        &self.method
    }

    // 호출자는 Option에는 관심이 없다. 오히려 Option이 감싸고 있는 것에만 관심이 있다.
    pub fn query_string(&self) -> Option<&QueryString> {
        // as_ref() : Converts from &Option<T> to Option<&T>.
        // 이런식으로 메서드를 구현하면 훨씬 더 유연하다.
        self.query_string.as_ref()
    }
}

// impl<'buf> Request<'buf> {
//     // from_byte_array는 실제로 실패할 수도 있기에 우리는 연산에 실패할 수있을 때는 Result를 리턴해야 한다.
//     // traits는 다른 언의 인터페이스와 유사하다. 우리 타입에 구현해야 할 추상적인 함수라고 생각하면 된다.
//     fn from_byte_array(buf: &[u8]) -> Result<Self, String> {
//         unimplemented!()
//     }
// }

// 어떤 타입에 대해 트레이트 구현하는 방법
// impl 뒤에 해당 트레이트의 이름을 넣어주고 다음에 for를 넣고 우리가 트레이트를 구현하길 원하는 타입을 넣어준다.
// 이렇게 하면 이 구현 블록의 본문에서 실제로 그 트레이트가 정의하는 기능을 구현하거나 추출하게 된다.
// // // 전체적으로 보면 buffer의 주기동안에 &str 등이 존재하게 되기에 'buf라는 라이프 타임을 주게 된다.
// // // 우리가 지정하는 수명은 우리가 컴파일러에게 제공하는 메타 데이터와 비슷하다.
// // // 그럼 컴파일러는 무엇이 진행되고 있는지 맥락을 더 잘알게 된다.
impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
    type Error = ParseError;
    // 우리는 아직 구현하 준비가 되지 않은 어떤 함수에도 unimplemented!() 매크로를 호출할 수 있다.
    // 그러면 컴파일러 불평이 억제되지만, 실제로 실행하게 되면 panic에 빠지게 된다.

    // GET /search?name=abc&sort=1 HTTP/1.1
    // // // 아래 함수는 buf 라는 파라미터를 하나 받는다. 이게 buf 라는 변수다.
    // // // 함수의 수명은 일반적으로 로컬 변수의 수명과 같다.
    fn try_from(buf: &'buf [u8]) -> Result<Request<'buf>, Self::Error> {
        // TryFrom을 구현하게 되면 바이트 슬라이스에 대한 TryInto를 함축하게 된다. 즉, TryInto 트레이트를 구현하는
        // 코드를 생성한다는 의미이다. 컴파일러가 추가적으로 TryInto를 구현해준다.
        // match str::from_utf8(buf) {
        //     Ok(request) => {}
        //     Err(_) => return Err(ParseError::InvalidEncoding),
        // }
        // // 아래의 패턴이 보다 많이 사용된다.
        // match str::from_utf8(buf).or(Err(ParseError::InvalidEncoding)) {
        //     Ok(request) => {}
        //     Err(e) => return Err(e),
        // }
        // 물음표 연산자는 이 매칭 구문이 하는 일과 거의 비슷한 일을 한다.
        // 우리가 넣어주는 Result를 살펴보고 Result가 ok 라면 그냥 Ok()가 감싸고 있는 값을 리턴한다.
        // 그렇지 않고 Result가 오류라면 우리 함수에서 오류를 리턴한다.
        // match와 다른 점은 물음표의 경우 리턴할 오류 타입이 매칭되지 않으면 받는 오류 타입을 변환하려 할 것이라는 점이다
        // str::from_utf8(buf)?; 만약 이렇게 되면 물음표는 Utf8Error에 바로 호출된다.
        // 그러면 UtfError를 ParseError로 변환하려 하게 될 것이다.
        let request = str::from_utf8(buf)?;

        // match get_next_word(request) {
        //     Some((method, request)) => {}
        //     None => return Err(ParseError::InvalidRequest),
        // }
        // 우리가 이런 let 구문에 기존의 변수 이름을 다시 사용하면 request 변수를 덮어쓰게 된다.
        // 이것을 변수 쉐도잉이라고 부른다. 이렇게 되면 기존 것은 더이상 사용할 수 없게 된다.
        // GET /search?name=abc&sort=1 HTTP/1.1\r\n 이렇게 정보가 들어오면
        // 아래 첫번재 쉐도잉에서 GET 과 그 이하로 나뉘고 이렇게 나뉜 뒤에것을 다시 쉐도잉을 통해 path와 나머지로 나눈다.
        // 그리고 마지막에는 \r(캐리지 리턴)이 있으므로
        // // // 컴파일러는 스마트하게 path변수의 수명이 우리가 get_next_word() 함수에 주는 파라미터 수명과 같아야 한다고 추론한다.
        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        // protocol 다음은 무시할 것이기 때문에 _로 처리한다.
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }
        // 여기서는 에러가 type Error = ParseError;이렇게 해놨기에 아래에서 ParseError에 MethodError를 추가해야 한다.
        let method: Method = method.parse()?;

        let mut query_string = None;

        // find는 option을 리턴한다. 왜냐하면 문자열에 매칭되는게 없을 수도 있기 때문이다.
        // 찾지 못하면 None을 리턴하고 찾으면 인덱스를 감싸고 있는 Some을 리턴한다.
        // match path.find('?') {
        //     Some(i) => {
        //         // 기억할 것은 1을 더하는 것은 글자 1개가 아닌, 1바이트를 추가하는 것이다
        //         // ?는 1바이트 이므로 이는 유효한 방법이 된다.
        //         query_string = Some(&path[i + 1..]);
        //         path=&path[..i];
        //     None => {}
        // }
        // let q = path.find('?');

        // if q.is_some(){
        //     let i = q.unwrap();
        //     query_string = Some(&path[i + 1..]);
        //     path=&path[..i];
        // }

        // 이렇게 작성해도 되지만, RUST에는 it let 이라는 기능이 있다.
        // 이 구문을 사용하면 일반적인 if문을 쓸 수 있지만 조건에 직접 패턴 매칭을 사용할 수 있다.

        // if문은 이 find() 함수의 Result를 보고 우리가 제공한 패턴에 매칭하려 할 것이다.
        if let Some(i) = path.find('?') {
            // 여기서는 참조했던 &path에서 참조 표시를 삭제한다.
            // 왜냐하면 더 이상 필요하지 않기 때문이다
            query_string = Some(QueryString::from(&path[i + 1..]));
            path = &path[..i];
        }

        Ok(Self {
            path,
            query_string,
            method,
        })
    }
}

// 러스트에는 No 타입은 없지만 Option으로  None을 리턴할 수 있다.
fn get_next_word(request: &str) -> Option<(&str, &str)> {
    // 문자열 슬라이스의 문자들을 반복하려면 chars() 메서드를 사용해야 한다.
    // 이것은 next() 메서를 호출할때 마다 Option을 돌려받게 되어서, 다음에 원소가 담겨 있거나 None을 받는다.
    // None 이게 되면 더 이상 원소가 없다는 의미이다.
    // let mut iter = request.chars();
    // loop {
    //     let item = iter.next();
    //     match item {
    //         Some(c) => {}
    //         None => break,
    //     }
    // }

    // 반복자만이 아니라 인덱스까지 받아야 하므로 enumerate를 추가한다.
    for (i, c) in request.chars().enumerate() {
        // 공백을 표현하려고 할때 " " 이렇게 쌍따옴표를 써서는 안된다.
        if c == ' ' || c == '\r' {
            // 공백 앞에 있는 모든 문자를 저장, 두번째는 공백을 제외한 다음문자를 추가하기 위해서 i + 1를 해준다.
            // 문자열에 대해 범위를 사용할때 인덱스만 추가하면 프로그램이 충돌할 수 있어서 매우 위험하다.
            // 여기서 i+1를 한다는 것은 1글자를 건너뛰는게 아니라, 1바이트를 추가한다는 의미이다.
            // 키릴 문자나 이모티콘이 들어가게 되면 유효하지 않은 utf-8 문자열을 생성하게 된다.
            // 하지만 이 경우에 우리는 인덱스 i에 있는 문자가 공백이라는 것을 알고 공백은 길이가 1바이트 라는 것을 알기에
            // 이렇게 사용해도 된다.
            return Some((&request[..i], &request[i + 1..]));
        }
    }

    None
}

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl ParseError {
    fn message(&self) -> &str {
        // 여기에서 self는 enum 이기에 match를 취할 수 있다.
        // 이게 함수 본문의 마지막 식이기 때문에 리턴 값이 자동으로 함수에서 리턴된다.
        match self {
            Self::InvalidRequest => "InvalidRequest",
            Self::InvalidEncoding => "InvalidEncoding",
            Self::InvalidProtocol => "InvalidProtocol",
            Self::InvalidMethod => "InvalidMethod",
        }
    }
}

impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        Self::InvalidMethod
    }
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

// 구현하기 원하는 트레이트로 가기 위해 Display를 클릭하고 우리가 정의하기를 기대하는 함수의 시그니처를 복사해서 붙여넣는다.
impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        // Display 매크로를 위해 format 함수를 구현하려면 우리가 출력하려는 문자열을 생성해야 한다.
        // 그걸 여기 있는 Formatter 에 기록해야 한다.
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        // Display 매크로를 위해 format 함수를 구현하려면 우리가 출력하려는 문자열을 생성해야 한다.
        // 그걸 여기 있는 Formatter 에 기록해야 한다.
        write!(f, "{}", self.message())
    }
}

// 이 에러를 더 관용적으로 만들기 위해 error 라는 표준 라이브럴리부터 트레이트를 구현해야 한다
// Error 트레이트를 구현함으로써 오류 타입에 관한 기본적인 기대를 충족하도록 강제한다.

impl Error for ParseError {}

// trait Encrypt {
//     fn encrypt(&self) -> Self;
// }

// impl Encrypt for String {
//     fn encrypt(&self) -> Self {
//         unimplemented!()
//     }
// }

// impl Encrypt for &[u8] {
//     fn encrypt(&self) -> Self {
//         unimplemented!()
//     }
// }
