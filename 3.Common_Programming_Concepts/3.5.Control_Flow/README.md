# Control Flow

## if

러스트에서 표현식에는 항상 boolean 값만 허용된다. 다른 언어 처럼 0 같은거 허용 안됨.

`else if` 표현식이 너무 많으면 복잡해질 수 있으므로, 표현식이 두 개 이상이면 코드를 리팩터링 하는게 좋다.

```rust
fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}
```

### let 구문에서 if 사용하기

```rust
fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
}
```

## 반복문

`loop`, `while`, `for` 세 종류의 반복문 존재

### loop

그만두라고 명시적으로 알려주기 전까지 계속 반복

```rust
fn main() {
    loop {
        println!("again!");
    }
}
```

**반복문에서 값 반환하기**

`break` 표현식 뒤에 반환하고자 하는 값을 넣으면 된다.

```rust
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}
```

**루프 라벨**

```rust
fn main() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}
```

### while

```rust
fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}
```

### for

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}
```

```rust
// Range 타입 사용. 어떤 숫자에서 시작하여 다른 숫자 종료 전까지의 모든 숫자를 차례로 생성
// rev 메서드는 범위값을 역수로 만들어준다.
fn main() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
```
