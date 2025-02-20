# To panic! or Not to panic!

## 예제, 프로토타입 코드, 테스트

`unwrap`, `expect`

## 컴파일러보다 더 많은 정보를 가지고 있을 때

`Result`가 `Ok` 를 가지고 있을 것이라 확신하지만, 컴파일러가 그 논리를 이해 못할 때는

`unwrap`, `expect`

`expect` 의 문구에 `Err` 배리언트가 있으면 안될 이유를 적어주면 더 좋다.

```rust
    use std::net::IpAddr;

    let home: IpAddr = "127.0.0.1"
        .parse()
        .expect("Hardcoded IP address should be valid");
```

## 에러 처리를 위한 가이드라인

코드가 어떤 가정, 보장, 계약 혹은 불변성이 깨지는 나쁜 상황일 때 `panic!` 이 바람직 하다:

- 나쁜 상황이란 예기치 못한 상황이라면, 이는 사용자가 입력한 데이터가 잘못된 형식이라던가 하는 흔히 발생할 수 있는 것과는 반대되는 것이다.
- 그 시점 이후의 코드는 매번 해당 문제에 대해 검사를 하는 것이 아니라, 이 나쁜 상태에 있지 않아야만 할 필가 있다.
- 지금 사용하고 있는 타입 내에 이 정보를 집어넣을만한 뾰족한 수가 없다. 이러한 의미에 대한 예제는 17장에서 다룸.

나의 제어권에서 벗어난 외부에서 코드를 호출하고 있고, 이것이 고칠 방법이 없는 유효하지 않은 상태를 반환한다면 `panic!` 이 종종 적절하다.

하지만 실패가 충분히 예상되면 `Result` 를 반환하는 것이 더 적절하다.

코드가 유효하지 않은 값에 대해 호출되면 사용자를 위험에 빠뜨릴 수 있는 연산을 수행할 때, 그 코드는 해당 값이 유효한지 먼저검사하고, 만일 그렇지 않다면 `panic!`을 호출해야 된다.

종종 함수에는 입력이 특정 요구사항을 만족시킬 경우에만 함수의 행동이 보장되는 계약이 있다. 이 계약을 위반했을 때는 패닉을 발생시키는게 이치에 맞다.

## 유효성을 위한 커스텀 타입 생성

```rust
    loop {
        // --생략--

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if guess < 1 || guess > 100 {
            println!("The secret number will be between 1 and 100.");
            continue;
        }

        match guess.cmp(&secret_number) {
            // --생략--
    }
```

위 코드를 1 ~ 100 사이의 값을 받았을 때면 인스턴스를 생성하는 타입을 만들어서 개선

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

    pub fn value(&self) -> i32 {
        self.value
    }
}
```
