# Variables and Mutability

By default, variables are immutable.

It allow you to write your code in a way that takes advantage of Rust's safety and easy concurrency.

You can make them mutable by adding `mut` in front of the variable name.

## Constants

Constants are always immutable.

You declare contants using the `const` keyword instead of the `let` keyword, and the type of the value must be annotated.

Constants can be declared in any scope, including the global scope, which makes them useful for values that many parts of code need to know about.

The last difference is that constants may be set only to a constant expression, not the result of a value that could only be computed at runtime.

```rust
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
```

Rust’s naming convention for constants is to use all uppercase with underscores between words.

## Shadowing

You can declare a new variable with the same name as a previous variable. Rustaceans say that the first variable is shadowed by the second, which means that the second variable is what the compiler will see when you use the name of the variable.

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

```bash
$ cargo run
   Compiling variables v0.1.0 (file:///projects/variables)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.31s
     Running `target/debug/variables`
The value of x in the inner scope is: 12
The value of x is: 6
```

Shadowing is different from marking a variable as mut because we’ll get a compile-time error if we accidentally try to reassign to this variable without using the let keyword.

The other difference between mut and shadowing is that because we’re effectively creating a new variable when we use the let keyword again, we can change the type of the value but reuse the same name.

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
