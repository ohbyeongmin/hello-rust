# Data Types

- Scalar
- Compound

러스트는 정적 타입의(statically typed) 언어이다. 모든 변수의 타입이 컴파일 시점에 반드시 정해져 있어야 한다.

## Scalar Types

스칼라 타입은 하나의 값을 표현. integers, floating-point numbers, Booleans and characters.

### Integer Types

- 8bit | i8 | u8
- 16bit | i16 | u16
- 32-bit | i32 | u32
- 64-bit | i64 | u64
- 128-bit | i128 | u128
- arch | isize | usize

정수형 리터럴

- Decimal: 98_222
- Hex: 0xff
- Octal: 0o77
- Binary: 0b1111_0000
- Byte(`u8` only): b'A'

어떤 정수를 사용할지 확실치 않으면 일반적으로 `i32`

### Floating-Point Types

기본적으로 `f64` 타입

### 수치 연산

```rust
fn main() {
    // 덧셈
    let sum = 5 + 10;

    // 뺄셈
    let difference = 95.5 - 4.3;

    // 곱셈
    let product = 4 * 30;

    // 나눗셈
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // 결괏값은 -1입니다

    // 나머지 연산
    let remainder = 43 % 5;
}
```

### Boolean Types

- `true`
- `false`

### The Character Type

```rust
fn main() {
    let c = 'z';
    let z: char = 'ℤ'; // 명시적인 타입 어노테이션
    let heart_eyed_cat = '😻';
}
```

문자열 리터럴이 큰따옴표, `char` 타입은 작은따옴표.

4bytes 크기를 가지고 있다.

## Compound Types

복합 타입은 여러 값을 하나의 타입으로 묶을 수 있다. tuple, array 두가지 복합 타입이 있다.

### Tuple type

튜플은 다양한 타입의 여러 값을 묶어 하나의 복합 타입으로 만든다.

튜플은 고정된 길이를 갖는다. 즉, 한번 선언되면 그 크기를 늘리거나 줄일 수 없다.

```rust
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
}
```

```rust
fn main() {
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {y}");
}
```

```rust
fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;
}
```

### Array Type

여러 값의 집합체를 만든다. 튜플과 달리 모든 요소는 같은 타입이어야 한다.

러스트는 고정된 배열의 길이를 갖는다.

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];
}
```

```rust
let a: [i32; 5] = [1, 2, 3, 4, 5];
```

```rust
let a = [3; 5];

// let a = [3, 3, 3, 3, 3]
```

유효하지 않은 인덱스 배열에 접근하려고 하는 경우 러스트는 패닉을 일으키고 실행을 종료함으로써 보호한다.
