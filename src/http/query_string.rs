use std::collections::HashMap;

// 우린 키와 값이 모두 우리가 request를 읽은 그 버퍼에 있을 것이라는 것을 안다.

// a=1&b=2&c&d=&e===&d=7&d=abc
// c는 비어 있는 값으로 삽입할 것이고, d는 기호가 있지만 여전히 값은 없다.
// 하지만 뒤를 보면 더 값이 있는 것으로 보아 d는 값이 array가 되길 원한다.
#[derive(Debug)]
pub struct QueryString<'buf> {
    data: HashMap<&'buf str, Value<'buf>>,
}

#[derive(Debug)]
pub enum Value<'buf> {
    Single(&'buf str),
    // Mutiple은 어레이를 감싸야 하는데 문제는 일반적인 어레이는 길이의 값을 정해야 하지만 우리는 모른다.
    // 이럴때는 역동적으로 커질 수 있는 힙을 사용해서 어레이를 할당해야 하다.
    // 러스트에서는 힙 할당 동적 어레이를 벡터라고 한다.
    Multiple(Vec<&'buf str>),
}

// QueryString 구현 블록은 수명을 지정해줘야 한다. 왜냐하면 QueryString이
// 수명에 대해 제네릭하기 때문이다.
impl<'buf> QueryString<'buf> {
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.data.get(key)
    }
}

// a=1&b=2&c&d=&e===&d=7&d=abc
// 이것은 실패할 수 없기 때문에 TryFrom이 아니라 From을 사용한다.
impl<'buf> From<&'buf str> for QueryString<'buf> {
    fn from(s: &'buf str) -> Self {
        let mut data = HashMap::new();
        // split()은 문자열 슬라이스에서 우리가 넣어준 패턴으로 구분된 모든 하위 문자열을 반복하는
        // 이터레이터를 리턴하는 일을 한다.
        for sub_str in s.split('&') {
            let mut key = sub_str;
            let mut val = "";
            if let Some(i) = sub_str.find('=') {
                // =는 1바이트 이므로 아래와 같이 해도 안전하다.
                key = &sub_str[..i];
                val = &sub_str[i + 1..];

                data.entry(key)
                    .and_modify(|existing: &mut Value| match existing {
                        Value::Single(prev_val) => {
                            // let mut vec = Vec::new();
                            // vec.push(val);
                            // vec.push(pre_val);
                            // 위 3줄처럼 작성할 수도 있지만, 벡터에는 매크로가 있다.

                            // 여기서 existing은 단순히 어딘가에 있는 어떤 메모리를 지시하는 주소에 불과함
                            // 아래처럼 existing 변수에 지정하면 우린 여기에있는 메모리를 교환하는게 아니다.
                            // 단순히 주소를 교환한것에 불과하다. 따라서 existing이 지시하고 있는 메모리를 교환하기 위해서는
                            // 포인터의 참조를 해제하고 거기 있는 메모리 주소에 값을 지정해야 한다. 이는 참조앞에 *를 추가하면 된다.
                            // existing = Value::Multiple(vec![prev_val, val]);
                            // 따라서 아래 처럼 하면 그 포인터를 따라가서 이 새 값을 그 포인터가 예전에 지시하던것에 덮어쓰라고 하는게 된다.
                            // 그럼 기존에는 1바이트였는데, 새로 값을 덮어씌운게 3바이트이면 옆의 메모리를 침범하지 않나라는 의문이 들수 있다.
                            // 하지만 안전한 이유가 enum의 variant 들은 모두 동일한 공간을 차지한다.
                            // 따라서 우린 Multiple variant가 Single variant 공간에 맞게 들어갈 것임을 알수 있다.
                            *existing = Value::Multiple(vec![prev_val, val]);
                        }
                        Value::Multiple(vec) => vec.push(val),
                    })
                    .or_insert(Value::Single(val));
            }
        }
        QueryString { data }
    }
}
