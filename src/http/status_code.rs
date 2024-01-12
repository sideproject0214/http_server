use std::fmt::{Display, Formatter, Result as FmtResult};
// https://www.restapitutorial.com/httpstatuscodes.html

// 배리언트에 추가 데이터를 감사고 있지 않는한 간단한 enum은 하나의 숫자로 표현된다.

#[derive(Copy, Clone, Debug)]
pub enum StatusCode {
    Ok = 200,
    BadRequest = 400,
    NotFound = 404,
}

impl StatusCode {
    pub fn reason_phrase(&self) -> &str {
        match self {
            Self::Ok => "OK",
            Self::BadRequest => "BadRequest",
            Self::NotFound => "NotFound",
        }
    }
}

impl Display for StatusCode {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        // write!(f, "{}", self as u16) 이 상태면
        // casting `&status_code::StatusCode` as `u16` is invalid
        // 이런 에러가 나온다. 이것은 우리가 참조를 캐스팅하고 있기 때문인데,
        // 참조는 단순히 포인터에 불과하기 때문이다. 참고로 여기서 '캐스팅 한다'는 의미는 데이터 형식을 다른 형식으로 변환하는 작업을 의미한다.
        // 따라서 실제 이 참조가 지시하고 있는 대상을 캐스팅해야 하기 때문에 참조를 해제해야 한다. 그러기 위해서 *을 넣어줘야 한다.

        // write!(f, "{}", *self as u16)
        // 이 상태에서도 에러가 난다. StatusCode가 Copy 트레이트를 구현하지 않기 때문이라고 나온다. 그 이유는
        // 우리는 프로그램에 있는 값들을 크게 2가지로 구분해 볼 수 있다.
        // 오직 스택에만 있는 값이 있고, 스택과 힙에 있는 값이 있다.
        // 전적으로 스택에만 있는 타입은 그것들의 바이트를 복사해서 쉽게 복사할 수 있다. 예를 들어 정수 타입
        // 정수는 스택에 저장되는 바이트 스퀀스에 불과하다. 이런 것들은 Copy 타입이다.
        // 반면 문자열은 길이 같은 메타 데이터만 저장하고 실제 텍스트가 거주하는 힙은 지시하는 포인터에 저장한다.
        // 그럼 스택에 있는 바이트를 복사하면 문자열 전체가 복사되는게 아니다.
        // 우린 단지 포인터와 메타 데이터만 복사하게 될 뿐이다.
        // 그런 문자열은 Copy 트레이트를 구현할 수 없다.
        // 이런 복합타입을 위해 Clone 이라는 트레이트가 있다.
        // Clone은 어떤 값의 완벽한 사본을 만드는 기능을 제공한다. 완벽한 사본이라는 것은 Clone 함수가 힙 데이터도
        // 복사하기 위해 추가적인 작업을 할 것이라는 의미다.

        // StatusCode 구조체는 정수로 표현된다. 이는 Copy를 구현할 수 있다는 의미이다. 왜냐하면 정수의 바이트를
        // 복사하면 쉽게 복사가 되니까
        // <기준은 스택을 놓고 말을 한다 / 왜냐하면 스택이 빠르니까>
        // 참고로 Copy 를 구현하면 Copy와 같은 작업을 수행하는 Clone에 대한 간단한 구현체도 갖게되기 때문에 이 둘은
        // 항상 같이 구현해야 한다.

        // StatusCode 구조체에 Copy, Clone 을 구현했으면 에러는 발생하지 않는다.
        // 컴파일러는 enum의 내용을 복사하고 enum 자체를 옮기지 않은 채로 그걸 u16으로 다룰 것이기 때문이다.
        write!(f, "{}", *self as u16)
    }
}
