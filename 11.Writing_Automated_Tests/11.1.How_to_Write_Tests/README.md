# How to Write Tests

테스트란, 테스트할 코드가 의도대로 기능하는지 검증하는 함수이다. 테스트 함수는 보통 본문에서 세 가지 동작을 수행한다.

1. 필요한 데이터나 상태 설정
2. 테스트할 코드 실행
3. 의도한 결과가 나오는지 확인

## 테스트 함수 파헤치기

러스트에서 테스트란 `test` 속성이 어노테이션된 함수이다. 속성은 러스트 코드 조각에 대한 메타데이터이다.
함수의 `fn` 이전 줄에 `#[test]` 를 추가하면 테스트 함수로 변경된다.
테스트는 `cargo test` 명령어로 실행되며, 이 명령을 실행하면 러스트는 속성이 표시된 함수를 실행하고 결과를 보고하는 테스트 실행 바이너리를 빌드한다.

카고로 새 라이브러리 프로젝트를 생성할 때마다 테스트 함수가 포함된 테스트 모듈이 자동 생성된다.

이 모듈이 테스트 작성을 위한 템플릿을 제공하므로, 새 프로젝트를 시작할 때마다 정확한 구조 및 테스트 함수 문법을 찾아볼 필요는 없다.

```bash
$ cargo new adder --lib
     Created library `adder` project
$ cd adder
```

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
```

`#[test]` 어노테이션은 해당 함수가 테스트 함수임을 표시한다.

예제 함수 본문에서는 `assert_eq!` 매크로를 사용하여 `result` 에 대한 단언(assert) 을 했는데, 이 변수의 내용물이 2와 2를 더한 결과인 4와 같다는 것이다.

`cargo test` 명령어로 테스트를 실행 합니다.

## `assert!` 매크로로 결과 검사하기

어떤 조건이 `true` 임을 보장하는 테스트를 작성할 땐 표준 라이브러리가 제공하는 `assert!` 매크로가 유용하다.

`assert!` 매크로는 부울린 값으로 평가되는 인수를 전달 받는다.

`true`일 경우 테스트가 통과하고, `false`일 경우 `panic!` 매크로를 호출하여 테스트를 실패하도록 만든다.

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
```

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }
}
```

`tests` 모듈에 `use super::*;` 줄이 추가되었다. `tests` 모듈 또한 7장에서 다룬 가시성 규칙을 따르는 평범한 모듈이다.
따라서, 내부 모듈인 `tests` 모듈에서 외부 모듈의 코드를 테스트하려면 먼저 내부 스코프로 가져와야 한다.
`tests` 모듈에서는 글롭(`*`)을 사용해 외부 모듈에 정의된 걸 전부 사용할 수 있도록 했다.

위 테스트는 `true`를 반환하여 테스트는 성공한다.

## `assert_eq!`, `assert_ne!` 매크로를 이용한 동등 테스트

`assert_eq!`, `assert_ne!` 매크는 각각 두 인수를 비교하고 동등한지 그렇지 않은지 판단한다.

`assert_eq!` 가 실패 했을 때 다음과 같은 문구를 출력한다.

```bash
$ cargo test
   Compiling adder v0.1.0 (file:///projects/adder)
    Finished test [unoptimized + debuginfo] target(s) in 0.61s
     Running unittests src/lib.rs (target/debug/deps/adder-92948b65e88960b4)

running 1 test
test tests::it_adds_two ... FAILED

failures:

---- tests::it_adds_two stdout ----
thread 'tests::it_adds_two' panicked at 'assertion failed: `(left == right)`
  left: `4`,
 right: `5`', src/lib.rs:11:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    tests::it_adds_two

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass `--lib`

```

테스트가 실패하고, 메시지는 `ssertion failed: `(left == right)``메시지와`left`, `right`가 각각 `4`, `5`였다는 것을 알려준다.

`assert_ne!` 매크로는 전달한 두 값이 서로 같지 않으면 통과하고, 동등하면 실패한다. 어떤 값이 될지는 확신할 수 없고 적어도 이 값은 되지 않아야 함을 알고 있는 경우에 유용하다.

내부적으로 `assert_eq!`, `assert_ne!` 매크로는 각각 `==`, `!=` 연산자를 사용한다. 따라서 매크로로 비교할 값은 `PartialEq`, `Debug` 트레이트를 구현해야 한다.

## 커스텀 실패 메시지 추가하기

`assert!`, `assert_eq!`, `assert_ne!` 매크로에 추가 인수로 실패 메시지에 출력될 내용을 추가할 수 있다.

필수적인 인수들 이후의 인수는 `format!` 매크로로 전달된다.

```rust
    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`",
            result
        );
    }
```

```bash
$ cargo test
   Compiling greeter v0.1.0 (file:///projects/greeter)
    Finished test [unoptimized + debuginfo] target(s) in 0.93s
     Running unittests src/lib.rs (target/debug/deps/greeter-170b942eb5bf5e3a)

running 1 test
test tests::greeting_contains_name ... FAILED

failures:

---- tests::greeting_contains_name stdout ----
thread 'tests::greeting_contains_name' panicked at 'Greeting did not contain name, value was `Hello!`', src/lib.rs:12:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    tests::greeting_contains_name

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass `--lib`
```

실제 테스트 결과값을 볼 수 있으니 의도했던 것과 무엇이 다른지 알 수 있어, 디버깅에 도움이 된다.

## `should_panic` 매크로로 패닉 발생 검사하기

코드의 반환 값을 검사하는 것에 더하여, 예상한대로 에러 조건을 잘 처리하는지 검사하는 것도 중요하다.

패닉 검사 테스트 함수에는 `should_panic` 속성을 추가한다. 이 테스트는 내부에서 패닉이 발생해야 통과되고, 패닉이 발생하지 않으면 실패한다.

```rust
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }
}
```

`should_panic`을 사용하는 테스트는 정확하지 않을 수 있다. 의도한 것과 다른 이유로 패닉이 발생하더라도 `should_panic` 테스트는 통과할 것이다.

`should_panic` 속성에 `expected` 매개변수를 추가해, 포함되어야 하는 메시지를 지정하면 더 꼼꼼한 `should_panic` 테스트를 작성할 수 있다.

```rust
// --생략--

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!(
                "Guess value must be greater than or equal to 1, got {}.",
                value
            );
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {}.",
                value
            );
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }
}
```

## `Result<T, E>` 를 이용한 테스트

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}
```

`Result<T, E>`를 반환하는 테스트에서는 `?` 연산자를 사용할 수 있기 때문에, 내부 작업이 `Err` 을 반환할 경우 실패해야 하는 테스트를 작성하기 편리하다.

`Result<T, E>` 테스트에서는 `#[should_panic]` 어노테이션을 사용할 수 없다. 연산이 `Err` 배리언트를 반환하는 것을 단언하기 위해서는 `Result<T, E>` 값에 물음표 연산자를 사용하지 말고 대신 `assert!(value.is_err())` 을 사용해라.
