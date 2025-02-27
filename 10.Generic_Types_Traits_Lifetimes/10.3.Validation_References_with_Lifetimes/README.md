# Validating References with Lifetimes

라이프타임은 어떤 타입이 원하는 동작이 구현되어 있음을 보장하는게 아니라 어떤 참조자가 필요한 기간 동안 유효함을 보장하도록 한다.

러스트의 모든 참조자는 라이프타임이라는 참조자의 유효성을 보장하는 범위를 갖는다. 대부분의 상황에서 타입이 암묵적으로 추론되듯, 라이프타임도 암묵적으로 추론된다.

하지만 여러 타입이 될 가능성이 있는 상황에서는 타입을 명시해 주어야 하듯, 참조자의 수명이 여러 방식으로 서로 연관될 수 있는 경우에는 라이프타임을 명시해 주어야 한다.

러스트에서 런타임에 사용되는 실제 참조자가 반드시 유효할 것임을 보장하려면 제네릭 라이프타임 매개변수로 이 관계를 명시해야 한다.

## 라이프타임으로 댕글림 참조 방지하기

라이프타임의 주목적은 댕글링 참조 방지이다. 댕글링 참조는 프로그램이 참조하려고 한 데이터가 아닌 엉뚱한 데이터를 참조하게 되는 것이 원인이다.

```rust
fn main() {
    let r;

    {
        let x = 5;
        r = &x;
    }

    println!("r: {}", r);
}
```

이 코드는 컴파일 되지 않는다. `r` 이 참조하는 값이 사용하려는 시점에 이미 자신의 스코프를 벗어났기 때문이다.
에러 메시지는 다음과 같다:

```bash
$ cargo run
   Compiling chapter10 v0.1.0 (file:///projects/chapter10)
error[E0597]: `x` does not live long enough
 --> src/main.rs:6:13
  |
6 |         r = &x;
  |             ^^ borrowed value does not live long enough
7 |     }
  |     - `x` dropped here while still borrowed
8 |
9 |     println!("r: {}", r);
  |                       - borrow later used here

For more information about this error, try `rustc --explain E0597`.
error: could not compile `chapter10` due to previous error
```

변수 `x` 가 출분히 오래 살지 못했습니다. `x` 는 안쪽 스코프를 벗어났지만 `r`은 바깥쪽 스코프에서 유효하기 때문이다. 스코프가 더 클수록 '더 오래 산다(lives longer)'고 표현한다.

## 대여 검사기

러스트 컴파일러는 대여 검사기(borrow checker)로 스코프를 비교하여 대여의 유효성을 판단한다.

```rust
fn main() {
    let r;                // ---------+-- 'a
                          //          |
    {                     //          |
        let x = 5;        // -+-- 'b  |
        r = &x;           //  |       |
    }                     // -+       |
                          //          |
    println!("r: {}", r); //          |
}                         // ---------+
```

`r`의 라이프 타임은 `'a`, `x`의 라이프타임은 `'b` 이다. 안쪽 `'b` 블록은 바깥쪽 `'a` 라이프타임 블록보다 작다. 러스트는 컴파일 타임에 두 라이프타임의 크기를 비교하고, `'a` 라이프타임을 갖는 `r`이 `'b` 라이프타임을 갖는 메모리를 참조하고 있음을 인지한다. 하지만 `'b`가 `'a`보다 짧으니, 즉 참조 대상이 참조자보다 오래 살지 못하니 러스트 컴파일러는 이 프로그램을 컴파일 하지 않는다.

```rust
fn main() {
    let x = 5;            // ----------+-- 'b
                          //           |
    let r = &x;           // --+-- 'a  |
                          //   |       |
    println!("r: {}", r); //   |       |
                          // --+       |
}                         // ----------+
```

여기서 `x`의 라이프타임 `'b`는 `'a`보다 더 길다. 러스트는 참조자 `r`이 유효한 동안에는 `x`도 유효하다는 것을 알고 있으므로, `r`은 `x`를 참조할 수 있다.

## 함수에서의 제네릭 라이프타임

두 문자열 슬라이스 중 긴 쪽을 반환하는 함수를 작성:

```rust
fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}
```

아래 처럼 `longest` 함수를 구현할 경우, 컴파일 에러가 발생한다.

```rust
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

나타나는 에러는 라이프타임과 고나련되어 있다.

```bash
$ cargo run
   Compiling chapter10 v0.1.0 (file:///projects/chapter10)
error[E0106]: missing lifetime specifier
 --> src/main.rs:9:33
  |
9 | fn longest(x: &str, y: &str) -> &str {
  |               ----     ----     ^ expected named lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `x` or `y`
help: consider introducing a named lifetime parameter
  |
9 | fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
  |           ++++     ++          ++          ++

For more information about this error, try `rustc --explain E0106`.
error: could not compile `chapter10` due to previous error

```

반환 타입에 제네릭 라이프타임 매개변수가 필요하다는 내용이다. 반환할 참조자가 `x`인지 `y`인지 러스트가 알 수 없기 때문이다.

위 코드는 스코프를 살펴보는 것만으로는 반환할 참조자의 유효성을 보장할 수 없다. 대여 검사기도 `x`, `y` 라이프타임이 반환 값의 라이프타임과 어떤 연관이 있는지 알지 못하니 마찬가지다.
따라서, 참조자 간의 관계를 제네릭 라이프타임 매개변수로 정의하여 대여 검사기가 분석할 수 있도록 해야한다.

## 라이프타임 명시 문법

라이프타임을 명시한다고 해서 참조자의 수명이 바뀌진 않는다. 그보다는 여러 참조자에 대한 수명에 영향을 주지 않으면서 서로 간 수명의 관계가 어떻게 되는지에 대해 기술하는 것이다.

함수에 제네릭 라이프타임 매개변수를 명시하면 어떠한 라이프타임을 갖는 참조자라도 전달할 수 있다.

라이프타임 매개변수의 이름은 어퍼스트로피(`'`)로 시작해야 하며, 보통은 제네릭 타입처럼 짧은 소문자로 정한다.
대부분의 사람들은 첫 번째 라이프타임을 명시할 때 `'a`를 사용한다.
라이프 타임 매개변수는 참조자의 `&` 뒤에 위치하며, 공백을 한 칸 입력하여 참조자의 타입과 분리한다.

```rust
&i32        // 참조자
&'a i32     // 명시적인 라이프타임이 있는 참조자
&'a mut i32 // 명시적인 라이프타임이 있는 가변 참조자
```

## 함수 시그니처에서 라이프타임 명시하기

제네릭 타입 매개변수를 사용할 때처럼 함수명과 매개변수 목록 사이의 꺾쇠괄호 안에 제네릭 라이프타임 매개변수를 선언해야 한다.

시그니처에서는 다음과 같은 제약사항을 표현하려고 한다: 두 매개변수의 참조자 모두가 유효한 동안에는 반환된 참조자도 유효할 것이다. 이는 매개변수들과 반환 값 간의 라이프타임 관계이다.

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

이 코드는 정상적으로 컴파일이 된다.

이 함수 시그니처는 러스트에게, 함수는 두 매개변수를 갖고 둘 다 적어도 라이프타임 `'a` 만큼 살아있는 문자열 슬라이스이며, 반환하는 문자열 슬라이스도 라이프타임 `'a` 만큼 살아있다는 정보를 알려준다.

이것의 실제 의미는, `longest`함수가 반환하는 참조자의 라이프타임은 함수 인수로서 참조된 값들의 라이프타임 중 작은 것과 동일하다는 의미이다.

함수 시그니처에 라이프타임 매개변수를 지정한다고 해서, 전달되는 값이나 반환 값의 라이프타임이 변경되는 건 아니라는 점을 기억.

어떤 값이 제약 조건을 지키지 않았을 때 대여 검사기가 불합격 판정을 내릴 수 있도록 명시할 뿐이다. `longest` 함수는 `x`와 `y`가 얼마나 오래 살지 정확히 알 필요는 없고, 이 시그니처를 만족하는 어떤 스코프 `'a`로 대체할 수 있다는 점만 알고 있으면 된다.

라이프타임 명시는 함수 시그니처의 타입들과 마찬가지로 함수에 대한 계약서의 일부가 된다. 함수 시그니처가 라이프타임 계약을 가지고 있다는 것은 러스트 컴파일러가 수행하는 분석이 좀 더 단순해질 수 있음을 의미한다.

만일 함수가 명시된 방법이나 함수가 호출된 방법에 문제가 있다면, 컴파일러 에러가 해당 코드의 지점과 제약사항을 좀 더 정밀하게 짚어낼 수 있다.

`longest` 함수에 구체적인 참조자들이 넘겨질 때 `'a` 에 대응되는 구체적인 라이프타임은 `x` 스코프와 `y` 스코프가 겹치는 부분이다. 다시 말해, `x` 라이프타임과 `y` 라이프타임 중 더 작은 쪽이 제네릭 라이프타임 `'a`의 구체적인 라이프타임이 된다.

반환하는 참조자도 동일한 라이프타임 매개변수 `'a`를 명시했으므로, `x`, `y` 중 더 작은 라이프타임 내에서는 `longest`가 반환한 참조자의 유효함을 보장할 수 있다.

```rust
fn main() {
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
}
```

대여 검사기는 위 코드를 문제 삼지 않는다.

반면 아래 코드는 에러가 발생한다:

```rust
fn main() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);
}
```

```bash
$ cargo run
   Compiling chapter10 v0.1.0 (file:///projects/chapter10)
error[E0597]: `string2` does not live long enough
 --> src/main.rs:6:44
  |
6 |         result = longest(string1.as_str(), string2.as_str());
  |                                            ^^^^^^^^^^^^^^^^ borrowed value does not live long enough
7 |     }
  |     - `string2` dropped here while still borrowed
8 |     println!("The longest string is {}", result);
  |                                          ------ borrow later used here

For more information about this error, try `rustc --explain E0597`.
error: could not compile `chapter10` due to previous error
```

함수 매개변수와 반환 값에 모두 동일한 라이프타임 매개변수 `'a`를 명시했으므로, 러스트는 문제를 정확히 파악할 수 있다.

우리 눈으로 보기에는 `string1`의 문자열이 `string2` 문자열 보다 길기 때문에 문제가 없어 보이지만 컴파일러는 이 점을 알아챌 수 없다. 러스트는 `longest`함수가 반환할 참조자의 라이프타임은 매개변수의 라이프타임 중 작은 것과 동일하다 라는 내용이었으니 대여 검사기는 위 코드가 잠재적으로 유효하지 않은 참조자를 가질 수도 있다고 판단한다.

## 라이프타임의 측면에서 생각하기

라이프타임 매개변수 명시의 필요성은 함수가 어떻게 동작하느지에 따라 달라진다.

```rust
fn longest<'a>(x: &'a str, y: &str) -> &'a str {
    x
}
```

위 `y`의 라이프타임은 `x`나 반환 값의 라이프타임과 전혀 관계없으므로, 매개변수 `y`에는 `'a` 를 지정하지 않는다.

참조자를 반환하는 함수를 작성할 때는 반환 타입의 라이프타임 매개변수가 함수 매개변수 중 하나와 일치해야 한다. 반환할 참조자가 함수 매개변수중 하나를 참조하지 않을 유일한 가능서ㅓㅇ은 함수 내부에서 만들어진 값의 참조자를 반환하는 경우다.

하지만 이 값은 함수가 끝나는 시점에 스코프를 벗어나므로 댕글링 참조가 될 것이다.

```rust
fn longest<'a>(x: &str, y: &str) -> &'a str {
    let result = String::from("really long string");
    result.as_str()
}
```

```bash
$ cargo run
   Compiling chapter10 v0.1.0 (file:///projects/chapter10)
error[E0515]: cannot return reference to local variable `result`
  --> src/main.rs:11:5
   |
11 |     result.as_str()
   |     ^^^^^^^^^^^^^^^ returns a reference to data owned by the current function

For more information about this error, try `rustc --explain E0515`.
error: could not compile `chapter10` due to previous error
```

`resut`는 `longest` 함수가 끝나면서 스코프를 벗어나 정리되는데, 함수에서 `result`의 참조자를 반환하려고 하니 문제가 발생한다. 여기서 댕글링 참조가 발생하지 않도록 라이프타임 매개변수를 지정할 방법은 없다.

이런 상황을 해결하는 가장 좋은 방법은 참조자 대신 값의 소유권을 갖는 데이터 타입을 반환하여 함수를 호출한 함수 측에서 값을 정리하도록 하는 것이다.

라이프타임 문법의 근본적인 역할은 함수의 다양한 매개변수와 반환 값의 라이프타임을 연결하는 데에 있다. 한번 라이프타임을 연결해 주고 나면, 러스트는 해당 정보를 이용해 댕글링 포인터 생성을 방지하고, 메모리 안전 규칙을 위배하는 연산을 배제한다.

## 구조체 정의에서 라이프타임 명시하기

지금까지 정의해 본 구조체들은 모두 소유권이 있는 타입을 들고 있었다. 구조체가 참조자를 들고 있도록 할 수도 있지만, 이 경우 구조체 정의 내 모든 참조자에 라이프타임을 명시해야 된다.

```rust
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}
```

구조체의 제네릭 라이프타임 매개변수의 선언 방법은 제네릭 데이터 타입과 마찬가지로, 제네릭 라이프타임 매개변수의 이름을 구조체 이름 뒤 꺾쇠괄호 내에 선언하고 구조체 정의 본문에서 라이프타임 매개변수를 사용한다.

위 코드는 '`ImportantExcerpt` 인스턴스는 `part` 필드가 보관하는 참조자의 라이프타임보다 오래 살 수 없다'라는 의미이다.

`ImportantExcerpt` 인스턴스가 스코프를 벗어나기 전에는 `novel`이 스코프를 벗어나지 않으니, `ImportantExcerpt` 인스턴스는 유효하다.

## 라이프타임 생략

모든 참조자는 라이프타임을 가지며, 참조자를 사용하는 함수나 구조체는 라이프타임 매개변수를 명시해야한다. 하지만 이전 예제중에 라이프타임 명시가 없어도 컴파일 할 수 있었다.

```rust
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
```

이 함수는 라이프타임을 명시하지 않아도 컴파일 된다. 그 이유는 러스트 역사에 있다.

초기 버전 러스트에서는 이 코드를 컴파일할 수 없었다. 모든 참조자는 명시적인 라이프타임이 필요했다. 그 당시 함수 시그니처는 다음과 같이 작성했다.

```rust
fn first_word<'a>(s: &'a str) -> &'a str {
```

수많은 러스트 코드를 작성하고 난 후, 러스트 팀은 러스트 프로그래머들이 특정한 상황에서 똑같은 라이프타임 명시를 계속 똑같이 작성하고 있다는 것을 알아 냈다. 이 상황들은 예측 가능한 상황들이었으며, 몇 가지 결정론적인(deterministic) 패턴을 따르고 있었다. 따라서 러스트 팀은 컴파일러 내에 이 패턴들을 프로그래밍 하여, 라이프타임을 명시하지 않아도 대여 검사기가 추론할 수 있도록 했다.

러스트의 참조자 분석 기능에 프로그래밍 된 이 패턴들을 라이프타임 생략 규칙 (lifetime elision rules) 이라고 부른다.

생략 규칙이 완전한 추론 기능을 제공하는 것은 아니다. 만약 러스트가 이 규칙들을 적용했는데도 라이프타임이 모호한 참조자가 있으면, 컴파일러는 이 참조자의 라이프타임을 추측하지 않는다. 컴파일러는 추측 대신 에러를 발생시켜서, 라이프타임 명시를 추가하여 문제를 해결하도록 할 것이다.

- 함수나 메서드 매개변수의 라이프타임은 입력 라이프타임(input lifetime)
- 반환 값의 라이프타임은 출력 라이프타임 (output lifetime)

라이프타임 명시가 ㅇ벗을 때 컴파일러가 참조자의 라이프타임을 알아내는 데 사용하는 규칙은 세 개 이다. 첫 번째 규칙은 입력 라이프타임에 적용되고, 두번 째 및 세 번째 규칙은 출력 라이프타임에 적용된다.

세가지 규칙을 모두 적용했음에도 라이프타임을 알 수 없는 참조자가 있다면 컴파일러는 에러와 함께 작동을 멈춘다. 이 규칙은 `fn` 정의는 물론 `impl` 블록에도 적용되다.

- 첫번째 규칙: 컴파일러가 참조자인 매개변수 각각에게 라이프타임 매개변수를 할당한다는 것이다. `fn foo<'a>(x: &'a i32)` 처럼 매개변수가 하나인 함수는 하나의 라이프타임 매개변수를 갖고, `fn foo<'a, 'b>(x: &'a i32, y: &'b i32)` 처럼 매개변수가 두 개인 한수는 두 개의 개별 라이프타임 매개변수를 갖는 식이다.

- 두번째 규칙: 만약 입력 라이프타임 매개변수가 딱 하나라면, 해당 라이프타임이 모든 출력 라이프타임에 대입된다는 것이다: `fn foo<'a>(x: &'a i32) -> &'a i32`

- 세번째 규칙: 입력 라이프타임 매개변수가 여러 개인데, 그중 하나가 `&self` 나 `&mut self` 라면, 즉 메서드라면 `self`의 라이프타임은 모든 출력 라이프타임 매개변수에 대입된다. 이 규칙은 메서드 코드를 깔끔하게 만드는 데 기여한다.

아래 함수 시그니처 속 참조자의 라이프 타임을 이 규칙들로 알아보자

```rust
fn first_word(s: &str) -> &str {
```

첫번째 규칙을 적용해, 각각의 매개변수에 라이프타임을 지정하자. 시그니처는 이제 다음과 같다:

```rust
fn first_wor<'a>(s: &'a str) -> &str {
```

입력 라이프타임이 딱 하나 밖에 없으니 두번 째 규칙대로 출력 라이프타임에 입력 매개변수의 라이프타임을 대입하고 나면 다음과 같다:

```rust
fn first_word<'a>(s: &'a str) -> &'a str {
```

함수 시그니처의 모든 참조자가 라이프타임을 갖게 됐으니, 컴파일러는 프로그래머에게 이 함수의 라이프타임 명시를 요구하지 ㅇ낳고도 계속 코드를 분석할 수 있다.

이번엔 다른 예제로 넘어가자:

```rust
fn longest(x: &str, y: &str) -> &str {
```

첫번째 규칙을 적용해 각각의 매개변수에 라이프타임을 지정해보다. 이번에는 매개변수가 두 개니, 두 개의 라이프타임이 생긴다:

```rust
fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &str {
```

입력 라이프타임이 하나가 아니므로 두번째 규칙은 적용하지 않는다. `longest` 함수는 메서드가 아니니, 세 번째 규칙도 적용할 수 없다. 세 가지 규칙을 모두 적용했는데도 반환 타입의 라이프타임을 알아내지 못했다. 그렇기 때문에 컴파일 에러가 발생 했다.

세 번째 규칙은 메서드 시그니처에만 적용되니, 메서드에서의 라이프타임을 살펴보고, 왜 세 번째 규칙 덕분에 메서드 시그니처의 라이프타임을 자주 생략할 수 있는지 알아보자.

## 메서드 정의에서 라이프타임 명시하기

라이프타임 매개변수의 선언 및 사용 위치는 구조체 필드나 메서드 매개변수 및 반환 값과 연관 있느냐 없느냐에 따라 달라진다.

라이프타임이 구조체 타입의 일부가 되기 때문에, 구조체 필드의 라이프타임 이름은 `impl` 키워드 뒤에 선언한 다음 구조체 이름 뒤에 사용해야 된다.

`impl` 블록 안에 있는 메서드 시그니처의 참조자들은 구조체 필드에 있는 참조자의 라이프타임과 관련되어 있을 수도 있고, 독립적일 수도 있다. 또한 라이프타임 생략 규칙으로 인해 메서드 시그니처에 라이프타임을 명시 않아도 되는 경우도 있다.

```rust
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}
```

`impl` 뒤에서 라이프타임 매개변수를 선언하고 타입명 뒤에서 사용하는 과정을 필수적이지만, 첫번째 생략 규칙으로 인해 `self` 참조자의 라이프 타임을 명시할 필요는 없다.

다음은 세 번째 규칙이 적용되는 예시이다:

```rust
impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}
```

두 개의 입력 라이프타임이 있으니 첫 번째 라이프타임 생략 규칙 대로 `self`, `announcement` 에 각각의 라이프타임을 부여한다. 그 다음, 매개변수 중 하나가 `&self` 이니 반환 타입에 `&self` 의 라이프타임을 부여한다.

## 정적 라이프타임

정적 라이프타임(static lifetime) 은 `'static` 이라는 특별한 라이프타임 이다. `'static` 라이프타임은 해당 참조자가 프로그램의 전체 생애주기 동안 살아있음을 의미한다. 모든 문자열 리터럴은 `'static` 라이프타임을 가지며 다음과 같이 명시할 수 있다.

```rust
let s: &'static str = "I have a static lifetime.";
```

이 문자열의 텍스트는 프로그램의 바이너리 내에 직접 저장되기 때문에 언제나 사용할 수 있다. 따라서 모든 문자열 리터럴 라이프타임은 `'static` 이다.

`'static` 라이프타임을 제안하는 에러 메시지는 대부분의 경우 댕글링 참조를 만들다가 발생하거나, 사용 가능한 라이프타임이 잘못 짝지어져서 발생한다. 이러한 경우 바람직한 해결책은 그런 문제를 고치는 것이지, `'static` 라이프타임이 아니다.

# 제네릭 타입 매개변수, 트레이트 바운드, 라이프타임을 한곳에 사용해 보기

```rust
use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```
