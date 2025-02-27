# Generic Types, Traits, and Lifetimes

중복되는 개념을 효율적으로 처리하기 위한 도구. 제네릭은 다른 제네릭과 관계를 표현하여 컴파일과 실행 시점에 제네릭들이 실제로 무슨 타입으로 채워지는지 알필요 없다.

## 함수를 추출하여 중복 없애기

제네릭은 여러 가지 타입을 나타내는 자리표시자의 위치에 특정 타입을 집어넣는 것으로 코드 중복을 제거할 수 있게 해준다.

리스트에서 가장 큰 숫자를 찾아내는 프로그램:

```rust
fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let mut largest = &number_list[0];

    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);
}
```

두 개의 다른 숫자 리스트에서 가장 큰 숫자를 찾아내는 프로그램:

```rust
fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let mut largest = &number_list[0];

    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let mut largest = &number_list[0];

    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);
}
```

위 코드는 잘 동작하지만, 중복된 코드를 생성하는 것은 지루하고 에러가 발생할 가능성도 커진다. 또한, 로직을 바꾸고 싶을 때 수정해야 할 부분이 여러 군데임을 기억해야 한다.

먼저, 이러한 중복을 제거하기 위해, 정수 리스트를 매개변수로 전달받아 동작하는 함수를 정의하여 추상화 할 수 있는 방법이 있다:

```rust
fn largest(list: &[i32]) -> &i32 {
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

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = largest(&number_list);
    println!("The largest number is {}", result);
}
```

1. 중복된 코드를 식별
2. 중복된 코드를 함수의 본문으로 분리하고, 함수의 시그니처 내에 해당 코드의 입력값 및 반환 값을 명시.
3. 중복됐었던 두 지점의 코드를 함수 호출로 변경.

다음에는 제네릭으로 이 과정을 그대로 진행하여 중복된 코드를 제거해보자.
