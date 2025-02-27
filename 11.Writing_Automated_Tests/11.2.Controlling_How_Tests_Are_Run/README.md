# Controlling How Thests Are Run

명령어 옵션은 `cargo test` 에 전달되는 것도 있고, 테스트 바이너리에 전달되는 것도 있다.

이 둘을 구분하기 위해 `cargo test` 에 전달할 인수를 먼저 나열하고, `--` 구분자를 쓰고, 그 뒤에 테스트 바이너리에 전달할 인수를 나열한다.

`cargo test --help` 명령어는 `cargo test` 명령어에 사용 가능한 옵션을 표시하고

`cargo test -- --help` 명령어는 구분자 이후에 사용 가능한 옵션을 표시한다.

## 테스트를 병렬 혹은 순차적으로 실행하기

`--test-threads` 플래그와 함께 테스트 바이너리에서 사용할 스레드 개수를 지정할 수 있다.

```bash
$ cargo test -- --test-threads=1
```

## 함수 출력 표시하기

`--show-output` 옵션을 전달하여 성공한 테스트의 출력을 표시할 수 있다.

```bash
$ cargo test -- --show-output
```

## 이름을 지정해 일부 테스트만 실행

### 테스트 하나만 실행

`cargo test` 명령어에 테스트 함수 이름을 전달하여 해당 테스트만 할 수 있다.

```bash
$ cargo test one_hundred
   Compiling adder v0.1.0 (file:///projects/adder)
    Finished test [unoptimized + debuginfo] target(s) in 0.69s
     Running unittests src/lib.rs (target/debug/deps/adder-92948b65e88960b4)

running 1 test
test tests::one_hundred ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s
```

`2 filtered out` 을 표시하여, 실행한 테스트 이외에 다른 테스트가 존재함을 알려준다.

### 테스트를 필터링하여 여러 테스트 실행하기

테스트 이름의 일부만 지정하여 해당 값에 맞는 모든 테스트가 실행된다.

```bash
$ cargo test add
   Compiling adder v0.1.0 (file:///projects/adder)
    Finished test [unoptimized + debuginfo] target(s) in 0.61s
     Running unittests src/lib.rs (target/debug/deps/adder-92948b65e88960b4)

running 2 tests
test tests::add_three_and_two ... ok
test tests::add_two_and_two ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s
```

테스트가 위치한 모듈도 테스트 이름의 일부라는 점을 기억. 모듈 이름으로 필터링하면 해당 모듈 내 모든 테스트를 실행할 수 있다.

## 특별 요청이 없다면 일부 테스트 무시하기

`ignore` 속성을 어노테이션 하면 된다.

```rust
#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}

#[test]
#[ignore]
fn expensive_test() {
    // code that takes an hour to run
}
```

`cargo test -- --ignored` 명령어를 사용하면 무시된 테스트만 실행할 수 있다.

모든 테스트를 실행하고 싶다면 `cargo test -- --include-ignored`
