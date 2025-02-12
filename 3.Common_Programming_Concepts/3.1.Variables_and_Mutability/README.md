# Variables and Mutability

Rust에서 변수는 기본적으로 불변(immutable) 이다. 하지만 가변(mutable) 으로 만들 수 있다.

`mut` 을 붙여서 가변으로 만들 수 있다.

어떤것이 불변인지 가변인지 알 수 있기 때문에 변수를 추적하기 쉽다!

## 상수(constant)

상수는 불변 변수와 비슷하지만 약간 다르다.

`mut` 와 사용할 수 없다.

`const` 키워드로 선언 하며 값의 타입이 반드시 명시되어 있어야 한다.

상수는 전역 스코프를 포함한 어떤 스코프에서도 선언 가능하다.

상수는 반드시 상수 표현식으로만 설정될 수 있고 런타임에서만 계산될 수 있는 결과값으로는 안된다.

```rust
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
```

대문자이름\_대문자이름 이런식으로 네이밍 한다.

## Shadowing

새 변수를 이전 변수명과 같은 이름으로 선언할 수 있다.

```rust
fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}
```

`mut` 와 다른점은 같은 변수명으로 다른 타입의 값을 저장할 수 있다.
