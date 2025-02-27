# Generic Data Types

제네릭을 사용하면 함수 시그니처나 구조체의 아이템에 다양한 구체적 데이터 타입을 사용할 수 있다.

## 제네릭 함수 정의

제네릭 함수를 정의할 때는, 함수 시그니처 내 매개변수와 반환 값의 데이터 타입 위치에 제네릭을 사용한다.

아래 예제의 코드를 제네릭을 사용하여 이 함수들을 하나의 함수로 묶겠다.

```rust
fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {}", result);
}
```

시그니처 내 타입을 매개변수화 하려면 이름을 지어줘야 한다.

- 타입 이름을 지어줄 때는 대문자로 시작하는 UpperCamelCase 를 따른다.
- 타입 매개변수의 이름은 짧게 짓는것이 관례.
- 대부분 'type' 을 줄인 `T` 를 사용한다.

타입 매개변수를 사용하기 전에도 타입 매개변수의 이름을 선언해야 한다.

```rust
fn largest<T>(list: &[T]) -> &T {
```

이 정의는 '`largest` 함수는 어떤 타입 `T`에 대한 제네릭 함수' 라고 읽는다.

```rust
fn largest<T>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}
```

이 코드는 에러가 발생한다:

```bash
$ cargo run
   Compiling chapter10 v0.1.0 (file:///projects/chapter10)
error[E0369]: binary operation `>` cannot be applied to type `&T`
 --> src/main.rs:5:17
  |
5 |         if item > largest {
  |            ---- ^ ------- &T
  |            |
  |            &T
  |
help: consider restricting type parameter `T`
  |
1 | fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
  |             ++++++++++++++++++++++

For more information about this error, try `rustc --explain E0369`.
error: could not compile `chapter10` due to previous error
```

이 에러는 `largest`의 본문이 `T`가 될 수 있는 모든 타입에 대해 동작할 수 없을을 뜻한다.
함수 본문에서 `T` 타입 값들에 대한 비교가 필요하므로, 여기에는 값을 비교할수 있는 타입에 대해서만 동작할 수 있다.
비교가 가능하도록 표준 라이브러리는 임의의 타입에 대해 구현 가능한 `std::cmp::PartialOrd` 트레이트를 제공한다.

따라서 `T`가 `PartialOrd`를 구현한 것일 때만 유효하도록 제한을 두면 이 예제는 컴파일 된다.

## 제네릭 구조체 정의

`<>` 문법으로 구주체 필드에서 제네릭 타입 매개변수를 사용하도록 구조체를 정의할 수도 있다.

```rust
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
}
```

이 선언은 `Point<T>`가 어떤 타입 `T`에 대한 제네릭이며 `x`, `y` 필드는 실제 타입이 무엇이건 간에 둘 다 동일한 타입이라는것을 의미한다.

제네릭 구조체의 필드가 서로 다은 타입일 수 있도록 정의하고 싶다면 여러 개의 제네릭 타입 매개변수를 사용해야 된다.

```rust
struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };
}
```

## 제네릭 열거형 정의

```rust
enum Option<T> {
    Some(T),
    None,
}
```

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

작성한 코드에서 보유하는 값의 타입만 다른 구조체나 열거형이 여러 새 있음을 발견했을 때는 제네릭 타입을 사용해 코드 중복을 제거할 수 있다.

## 제네릭 메서드 정의

```rust
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn main() {
    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());
}
```

`impl` 뒤에 `T`를 선언하여 `Point<T>` 타입에 메서드를 구현한다고 명시했음을 주의. 이렇게 하면 러스트는 `Point`의 꺾쇠괄호 내 타입이 구체적인 타입이 아닌 제네릭 타입임을 인지한다.

이 타입의 메서드를 정의할 때 제네릭 타입에 대한 제약을 지정할 수도 있다. 예를 들면, 임의의 제네릭 타입 `Point<T>` 인스턴스가 아닌 `Point<f32>` 인스턴스에 대한 메서드만을 정의할 수 있다.

아래 예제는 구체적 타입 `f32`을 사용하였는데 `impl` 뒤에는 어떤 타입도 선언하지 않았다.

```rust
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
```

구조체 정의에서 사용한 제네릭 타입 매개변수와, 구조체의 메서드 시그니처 내에서 사용하는 제네릭 타입 매개변수가 항상 같은 것은 아니다.

```rust
struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
```

위 예제는 제네릭 매개변수 중 일부가 `impl` 에 선언되고 일부는 메서드 정의에 선언되는 경우를 보여주기 위한 예제이다. 여기서 제네릭 매개변수 `X1`, `Y1`는 구조체 정의와 한 묶음이니 `impl` 뒤에 선언했지만, 제네릭 매개변수 `X2`, `Y2`는 `mixup` 메서드에만 연관되어 있으므로 `fn mixup` 뒤에 선언한다.

## 제네릭 코드의 성능

제네릭 타입의 사용이 구체적인 타입을 사용했을 때와 비교해서전혀 느리지 않다.

러스트는 컴파일 타입에 제네릭을 사용하는 코드를 단형성화(monomorphization) 한다. 단형상화란 제네릭 코드를 실제 구체 타입으로 채워진 특정한 코드로 바꾸는 과정이다. 컴파일러는 제네릭 코드가 호출된 곳을 전부 찾고, 제네릭 코드가 호출될 때 사용된 구체 타입으로 코드를 생성한다.

```rust
let integer = Some(5);
let float = Some(5.0);
```

컴파일러는 `Option<T>` 인스턴스에 사용된 값을 읽고, `i32`, `f64` 두 종류의 `Option<T>` 가 있다는 것을 인지하고 제네릭 정의를 `i32`와 `f64`에 대해 특성화 시킨 정의로 확장한다.

```rust
enum Option_i32 {
    Some(i32),
    None,
}

enum Option_f64 {
    Some(f64),
    None,
}

fn main() {
    let integer = Option_i32::Some(5);
    let float = Option_f64::Some(5.0);
}
```

러스트 컴파일러가 제네릭 코드를 각 인스턴스의 명시적인 타입으로 변경해주기 때문에 굳이 런타임 비용을 줄이기 위해 수동으로 직접 각 타입마다 중복된 코드를 작성할 필요가 없다. 단형성화 과정으로 러스트 제네릭을 런타임에 극도로 효율적으로 만들어 준다.
