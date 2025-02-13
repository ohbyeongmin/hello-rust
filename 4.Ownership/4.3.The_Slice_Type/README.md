# The Slice Type

슬라이스(slice) 는 컬렉션(collection) 을 통째로 참조하는 것이 아닌, 컬렉션의 연속된 일련의 요소를 참조하도록 해준다. 슬라이스는 참조자의 일종으로서 소유권을 갖지 않는다.

## 문자열 슬라이스

문자열 슬라이스(string slice)는 `String`의 일부를 가리키는 참조자를 말한다. 문자열 슬라이스 타입은 `&str`로 작성한다.

```rust
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];
```

`[starting_index..ending_index]`는 `starting_index`부터 시작해 `ending_index` 직전, 즉 `ending_index` 에서 1을 뺀 위치까지 슬라이스를 생성한다는 의미이다.
슬라이스는 내부적으로 시작 위치, 길이를 데이터 구조에 저장하며, 길이 값은 `ending_index`값에서 `starting_index` 값을 빼서 계산한다.

`..` 범위 표현법은 인덱스 0 부터 시작할 경우 `[..2]` 로 생략할 수 있고, 맨 마지막 바이트를 포함하는 슬라이스는 `[3..]` 로 뒤의 값을 생략할 수 있다.

```rust
fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);

    s.clear(); // 에러!

    println!("the first word is: {}", word);
}
```

## 슬라이스로써의 문자열 리터럴

```rust
let s = "Hello, world!";
```

여기서 `s`는 바이너리의 특정 지점을 가리키는 슬라이스 이다. `&str` 타입이다. `&str`은 불변 참조자이므로, 문자열 리터럴은 왜 변경할 수 없다.

## 문자열 슬라이스를 매개변수로 사용하기

```rust
fn first_word(s: &str) -> &str {
```

`String`에 대한 참조자 대신에 문자열 슬라이스를 매개변수로 하는 함수를 정의하면 기능 면에서 손해보지 않으면서 API를 더 일반적이고 유용하게 만든다.

```rust
fn main() {
    let my_string = String::from("hello world");

    // `first_word`는 `String`의 일부 혹은 전체 슬라이스에 대해 작동합니다
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    // 또한 `first_word`는 `String`의 전체 슬라이스와 동일한 `String`의
    // 참조자에 대해서도 작동합니다
    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    // `first_word`는 문자열 리터럴의 일부 혹은 전체 슬라이스에 대해 작동합니다
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    // 문자열 리터럴은 *곧* 문자열 슬라이스이므로,
    // 아래의 코드도 슬라이스 문법 없이 작동합니다!
    let word = first_word(my_string_literal);
}
```

## 그 외 슬라이스

```rust
let a = [1, 2, 3, 4, 5];

let slice = &a[1..3];

assert_eq!(slice, &[2, 3]);
```
