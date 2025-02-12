# Functions

`fn` 키워드 사용.

러스트는 함수나 변수 이름을 snake_case 방식을 사용.

```rust
fn main() {
    println!("Hello, world!");

    another_function();
}

fn another_function() {
    println!("Another function.");
}
```

러스트는 함수 위치를 고려하지 않는다.

## Parameters

```rust
fn main() {
    print_labeled_measurement(5, 'h');
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}
```

## Statements and Expressions

- 구문: 어떤 동작을 수행하고 값을 반환하지 않는 명령어
- 표현식: 결과값을 평가

```rust
// 구문
fn main() {
    let y = 6;
}
```

```rust
// 구문은 값을 반환하지 않기 때문에 에러
fn main() {
    let x = (let y = 6);
}
```

```rust
fn main() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}

// 표현식
{
    let x = 3;
    x + 1
}
```

표현식은 종결을 나타내는 세미콜론을 쓰지 않는다.

만약 세미콜론은 추가하면, 표현식은 구문으로 변경되고 값을 반환하지 않게 된다.

## 반환 값을 갖는 함수

러스트에서 반환은 `->` 로 표현.

러스트에서 함수의 반환 값은 함수 본문의 마지막 표현식의 값과 동일.

`return` 키워드와 값을 지정하여 함수로부터 일찍 값을 반환할 수 있지만, 대부분 함수들은 암묵적으로 마지막 표현식 값을 반환 한다.

```rust
fn main() {
    let x = plus_one(5);

    println!("The value of x is: {x}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
```
