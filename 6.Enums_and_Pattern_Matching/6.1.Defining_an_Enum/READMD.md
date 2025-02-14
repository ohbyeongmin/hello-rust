# Defining an Enum

어떤 값이 여러개의 가능한 값의 집합 중 하나라는 것을 나타내는 방법.

```rust
enum IpAddrKind {
    V4,
    V6,
}
```

열거형을 정의할 때의 식별자로 네임스페이스가 만들어지기 때문에 각 배리언트 앞에 이종콜론(`::`) 을 붙여야 한다.

```rust
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
```

열거형 배리언트에 데이터를 직접 넣는 방식을 사용해서 열거형의 구조체의 일부로 사용하는 것보다 더 간결하게 동일한 개념을 표현할 수 있다.

```rust
    enum IpAddr {
        V4(String),
        V6(String),
    }

    let home = IpAddr::V4(String::from("127.0.0.1"));

    let loopback = IpAddr::V6(String::from("::1"));
```

각 배리언트는 다른 타입과 다른 양의 연관된 데이터를 가질 수 있다.

```rust
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IpAddr::V4(127, 0, 0, 1);

    let loopback = IpAddr::V6(String::from("::1"));
```

표준 라이브러리에 정의된 IP 주소

```rust
struct Ipv4Addr {
    // --생략--
}

struct Ipv6Addr {
    // --생략--
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}
```

열거형은 모든 배리언트가 하나의 열거형 타입으로 묶인다. 이때 함수의 인자를 받거나 `impl` 로 메서드를 작성할 때 하나의 타입으로 편하게 작성할 수 있다.

```rust
    impl Message {
        fn call(&self) {
            // 메서드 본문이 여기 정의될 것입니다
        }
    }

    let m = Message::Write(String::from("hello"));
    m.call();
```

## Option 열거형이 널 값보다 좋은 점들

`Option` 타입은 값이 있거나 없을 수 있는 아주 흔한 상황을 나타낸다.

러스트는 다른 언어에서 흔히 볼 수 있는 Null 개념이 없다.

널 값으로 발생하는 문제는, 널 값을 널이 아닌 값처럼 사용하려고 할 때 여러 종류의 에러가 발생할 수 있다는 것이다.

러스트는 값의 존재 혹은 부재의 개념을 표현할 수 있는 `Option<T>` 열거형이 있다.

```rust
enum Option<T> {
    None,
    Some(T),
}
```

`Option<T>` 열거형은 너무나 유용하기 때문에 러스트가 기본적으로 임포트하는 목록인 프렐루드에 포함되어 있다.

그렇기 대문에 배리언트 앞에 `Option::` 을 붙이지 않아도 된다.

```rust
    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;
```

왜 Null 보다 나을까?

`Option<T>`와 `T` 는 다른 타입이기 때문에, 컴파일러는 `Option<T>`값을 명백하게 유효한 값처럼 사용하지 못하도록 한다.

```rust
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = x + y;
```

```bash
$ cargo run
   Compiling enums v0.1.0 (file:///projects/enums)
error[E0277]: cannot add `Option<i8>` to `i8`
 --> src/main.rs:5:17
  |
5 |     let sum = x + y;
  |                 ^ no implementation for `i8 + Option<i8>`
  |
  = help: the trait `Add<Option<i8>>` is not implemented for `i8`
  = help: the following other types implement trait `Add<Rhs>`:
            <&'a i8 as Add<i8>>
            <&i8 as Add<&i8>>
            <i8 as Add<&i8>>
            <i8 as Add>

For more information about this error, try `rustc --explain E0277`.
error: could not compile `enums` due to previous error
```

둘은 다른 타입이기 때문에 러스트가 어떻게 더해야 하는지 모른다고 에러가 나난다.

러스트에서 `i8` 과 같은 타입의 값을 가질 때, 컴파일러는 항상 유효한 값을 가지고 있다는 것을 보장한다. `Option<i8>` 은 값이 있을지 없을지 컴파일러가 모른다.

연산을 하기 위해서는 `Option<T>` 를 `<T>` 로 바꿔줘야 한다. 이런 방식은 널로 인해 발생하는 사장 흔한 문제인, 실제로는 널인데 널이 아니라고 가정하는 상황을 발견하는데 도움이 된다.
