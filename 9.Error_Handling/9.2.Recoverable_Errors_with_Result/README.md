# Recoverable Errors with Result

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

```rust
use std::fs::File;

fn main() {
    let greeting_file_result = File::open("hello.txt");
}
```

`File::open` 의 반환 타입

- `T`: `std::fs::File`
- `E`: `std::io::Error`

```rust
use std::fs::File;

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
}
```

## 서로 다른 에러에 대해 매칭

```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };
}
```

```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}
```

## 패닉을 위한 숏컷: `unwrap` and `expect`

`unwrap`: `Ok`라면 `Ok` 값 반환, `Err`이라면 `panic!` 매크로 호출.

```rust
use std::fs::File;

fn main() {
    let greeting_file = File::open("hello.txt").unwrap();
}
```

`expect`: `panic!` 에러 메시지 선택.

```rust
use std::fs::File;

fn main() {
    let greeting_file = File::open("hello.txt")
        .expect("hello.txt should be included in this project");
}
```

## 에러 전파하기

함수를 호출하는 코드 쪽에서 에러를 처리하도록 한다.

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}
```

## 에러 전파 숏컷: `?`

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}
```

`match` 표현식과 `?` 연산자의 차이점은 `?` 연산자는 `from` 함수를 거친다는 것이다.

`from` 함수는 어떤 값의 타입을 다른 타입으로 변환하는 데 사용한다.

`?` 연산자가 `from` 함수를 호출하면, `?` 연산자가 얻게 되는 에러를 `?` 연산자가 사용된 현재 함수의 반환 타입에 정의된 에러 타입으로 변환 한다.

예를 들어, 위 예제에 커스텀 에러를 작성하여 커스텀 에러 타입을 반환하도록 고칠 수 있다.

`?`는 많은 양의 보일러 플레이트를 제거해 주고 함수의 구현을 더 단순하게 만들어 준다.

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}
```

```rust
use std::fs;
use std::io;

fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
```

## `?` 연산자가 사용될 수 있는 곳

`?`는 `?`이 사용된 값과 호환 가능한 반환 타입을 가진 함수에서만 사용가능 하다.

```rust
use std::fs::File;

fn main() {
    let greeting_file = File::open("hello.txt")?;
}
```

위의 메인함수와 `?` 연산자가 반환하는 타입이 다르므로 에러가 난다.

```bash
$ cargo run
   Compiling error-handling v0.1.0 (file:///projects/error-handling)
error[E0277]: the `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements `FromResidual`)
 --> src/main.rs:4:48
  |
3 | fn main() {
  | --------- this function should return `Result` or `Option` to accept `?`
4 |     let greeting_file = File::open("hello.txt")?;
  |                                                ^ cannot use the `?` operator in a function that returns `()`
  |
  = help: the trait `FromResidual<Result<Infallible, std::io::Error>>` is not implemented for `()`

For more information about this error, try `rustc --explain E0277`.
error: could not compile `error-handling` due to previous error
```

위 에러는`?` 연산자가 `Result`, `Option` 혹은 `FromResidual`을 구현한 타입을 반환하는 함수에서만 사용될 수 있음을 말한다.

`Option<T>` 도 마찬가지로 적용된다.

```rust
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
```

`main` 함수는 `Result<(), E>`도 반환할 수 있다.

```rust
use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    let greeting_file = File::open("hello.txt")?;

    Ok(())
}
```
