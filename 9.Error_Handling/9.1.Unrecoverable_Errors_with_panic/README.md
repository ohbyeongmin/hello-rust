# Unrecoverable Errors with panic!

패닉을 일으키는 방법

- 코드가 패닉을 일으킬 동작을 하는 것(배열 끝부분을 넘어선 접근)
- `panic!` 매크로를 명시적으로 호출하는 것

패닉은 실패 메시지를 출력하고, 되감고(unwind), 스택을 청소하고, 종료한다.

그만두기 방식 사용:

```toml
[profile.release]
panic = 'abort'
```

```rust
fn main() {
    panic!("crash and burn");
}
```

## `panic!` backtrace

```rust
fn main() {
    let v = vec![1, 2, 3];

    v[99];
}
```

`RUST_BACKTRACE` 을 0이 아닌 값으로 설정하여 백트레이스를 설정할 수 있다.

백트레이스를 얻기 위해서는 디버그 심볼이 활성화 되어 있어야 한다.
`cargo build`나 `cargo run`을 `--release` 플래그 없이 실행 했을 때 디버그 심볼이 활성.
