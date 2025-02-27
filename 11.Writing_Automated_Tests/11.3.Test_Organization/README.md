# Test Organization

## 유닛 테스트

유닛테스트의 목적은 각 코드 단위를 나머지 코드와 분리하여, 제대로 작동하지 않는 코드가 어느 부분인지 빠르게 파악하는 것이다.

유닛테스트는 src 디렉터리 내의 각 파일에 테스트 대상이 될 코드와 함께 작성한다.

각 파일에 `tests` 모듈을 만들고 `cfg(test)` 를 어노테이션하는게 일반적인 관례다.

### 테스트 모듈과 `#[cfg(test)]`

테스트 모듈에 어노테이션하는 `#[cfg(test)]` 은 이 코드가 `cargo build` 명령어가 아닌 `cargo test` 명령어 실행시에만 컴파일 및 실행될 것을 러스트에게 전달한다.

`cfg` 속성은 설정을 의미하며, 러스트는 이 아이템을 특정 설정 옵션 적용 시에만 포함한다.

이 경우 옵션 값은 러스트에서 테스트를 컴파일, 실행하기 위해 제공하는 `test` 이다.

### 비공개 함수 테스트하기

러스트의 비공개 규칙은 비공개 함수를 테스트하도록 허용한다.

```rust
pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
}
```

## 통합 테스트

통합 테스트는 내가 만든 라이브러리와 완전히 분리되어 있다. 통합 테스트는 외부 코드와 마찬가지로, 내가 만든 라이브러리의 공개 API만 호출 가능하다.

통합 테스트의 목적은 라이브러리의 여러 부분을 함께 사용했을 때 제대로 작동하는지 확인하는 것이다.

통합 테스트를 작성하려면 먼저 tests 디렉터리를 만들어야 한다.

### tests 디렉터

```bash
adder
├── Cargo.lock
├── Cargo.toml
├── src
│   └── lib.rs
└── tests
    └── integration_test.rs
```

파일명: tests/integration_test.rs

```rust
use adder;

#[test]
fn it_adds_two() {
    assert_eq!(4, adder::add_two(2));
}
```

`tests` 디렉터리의 각 파일은 별개의 크레이트 이므로, 각각의 테스트 크레이트의 스코프로 우리가 만든 라이브러리를 가져올 필요가 있다.

`#[cfg(test)]`가 필요 없다. 카고는 `tests` 디렉터리를 특별 취급하여 `cargo test` 시에만 컴파일 한다.

```bash
$ cargo test
   Compiling adder v0.1.0 (file:///projects/adder)
    Finished test [unoptimized + debuginfo] target(s) in 1.31s
     Running unittests src/lib.rs (target/debug/deps/adder-1082c4b063a8fbe6)

running 1 test
test tests::internal ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/integration_test.rs (target/debug/deps/integration_test-1082c4b063a8fbe6)

running 1 test
test it_adds_two ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests adder

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

통합 테스트도 마찬가지로 `cargo test` 명령어에 테스트 함수명을 인수로 전달해 특정 통합 테스트 함수를 실행할 수 있다.

특정 통합 테스트 파일의 모든 테스트를 실행하려면, `cargo test` 명령어에 `--test` 인수로 파일명을 전달하면 된다.

```bash
$ cargo test --test integration_test
   Compiling adder v0.1.0 (file:///projects/adder)
    Finished test [unoptimized + debuginfo] target(s) in 0.64s
     Running tests/integration_test.rs (target/debug/deps/integration_test-82e7799c1bc62298)

running 1 test
test it_adds_two ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

### 통합 테스트 내 서브 모듈

통합 테스트를 추가하다 보면, 조직화를 위해 tests 디렉터리에 더 많은 파일이 필요할 수도 있다.

tests 내 각 파일은 각각의 크레이트로 컴파일되는데, 이는 각 통합 테스트 파일이 각각의 크레이트로 취급된다는 점 때문에 우리가 만든 크레이트를 사용할 때 실제 사용자처럼 분리된 스코프를 만들어 내는 데에는 유용하다.

그러나 이는 src 디렉터리에서 코드를 모듈과 파일로 분리하여 동일한 동작은 공유하는 것을 tests 디렉터리 내 파일에서는 할 수 없음을 의미한다.

파일명: tests/common.rs

```rust
pub fn setup() {
    // 여기에 라이브러리 테스트와 관련된 설정 코드를 작성하려고 합니다
}
```

```bash
$ cargo test
   Compiling adder v0.1.0 (file:///projects/adder)
    Finished test [unoptimized + debuginfo] target(s) in 0.89s
     Running unittests src/lib.rs (target/debug/deps/adder-92948b65e88960b4)

running 1 test
test tests::internal ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/common.rs (target/debug/deps/common-92948b65e88960b4)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/integration_test.rs (target/debug/deps/integration_test-92948b65e88960b4)

running 1 test
test it_adds_two ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests adder

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

```

원하던 것은 다른 통합 테스트 파일에서 사용할 코드를 공유하는 것이지 common 파일리 테스트되어 출력되는게 아니었다.

테스트 출력 결과에서 `common`을 제외하려면 tests/common.rs 파일 대신 tests/common/mod.rs 파일을 생성해야 한다.

```bash
├── Cargo.lock
├── Cargo.toml
├── src
│   └── lib.rs
└── tests
    ├── common
    │   └── mod.rs
    └── integration_test.rs
```

이러한 방식의 파일명 규칙을 따르는 파일은 통합 테스트 파일로 취급하지 않는다.

tests 디렉터리의 서브 디렉터리 내 파일은 별도 크레이트로 컴파일되지 않고, 테스트 결과 출력에서 별도의 출력 절이 생성되지도 않는다.

tests/common/mod.rs 파일을 생성하고 나면 다른 통합 테스트 파일에서 모듈처럼 사용할 수 있다.

```rust
use adder;

mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, adder::add_two(2));
}
```

### 바이너리 크레이트에서의 통합 테스트

src/lib.rs 파일이 없고 src/main.rs 파일만 있는 바이너리 크레이트라면, tests 디렉터리에 통합 테스트를 만들어서 src/main.rs 파일에 정의된 함수를 `use` 구문으로 가져올 수 없다.

다른 크레이트에서 사용할 수 있도록 함수를 노출하는 건 라이브러리 크레이트 뿐이다. 바이너리 크레이트는 자체적으로 실행되게 되어있다.

바이너리를 제공하는 러스트 프로젝트들이 src/main.rs 파일은 간단하게 작성하고, 로직은 src/lib.rs 파일에 위치시키는 이유 중 하나가 이 때문이다.
