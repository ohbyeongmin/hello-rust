# Defining and Instantiating Structs

```rust
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
```

```rust
fn main() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
}
```

```rust
fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");
}
```

가변 인스턴스는 특정 필드의 값을 변경할 수 있다. 일부 필드만 가변으로 만들수 없으니 주의.

```rust
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}
```

## 필드 초기화 축약법 사용하기

변수명과 구조체 필드명이 같을 땐, 필드 초기화 축약법(field init shorthand)을 사용해서 더 적은 타이핑으로 같은 기능을 구현할 수 있다.

```rust
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
```

## 기존 인스턴스를 이용해 새 인스턴스를 만들 때 구조체 업데이트 문법 사용하기

다른 인스턴스의 대부분 값을 유지한 채 새로운 인스턴스를 만들 때 유용하다. (struct update syntax)

```rust
fn main() {
    // --생략--

    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };
}
```

`..` 문법은 따로 명시된 필드를 제외한 나머지 필드를 주어진 인스턴스 필드값으로 설정.

```rust
fn main() {
    // --생략--

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
}
```

구조체 업데이트 문법은 대입처럼 `=` 을 사용하기 때문에 이 구문은 데이터를 이동시킨다.
`user2` 를 생성한 후에는 `user1`을 더이상 사용할 수 없다.

이는 `user1`의 `username` 필드의 `String` 이 `user2`로 이동되기 때문이다. `user2`에 `email`과 `username`의 `String` 모두를 제공하고 `user1`에서는 `active`와 `sign_in_count` 값만 사용한다면, `user2` 를 만든 이후에도 `user1` 은 유효하다.

## 명명된 필드 없는 튜플 구조체를 사용하여 다른 타입 만들기

러스트는 튜플과 유사한 형태의 튜플 구조체(tuple structs)도 지원한다.
튜플 구조체는 구조체 자체에는 이름을 지어 의미를 주지만 이를 구성하는 필드에는 이름을 붙이지 않고 타입만 적어 넣은 형태이다.
튜플 구조체는 튜플 전체에 이름을 지어주거나 특정 튜플을 다은 튜플과 구분하고 싶은데, 그렇다고 각 필드명을 일일히 정해 일반적인 구조체 형태로 만들면 너무 장황하거나 불필요한 경우 유용하다.

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}
```

## 필드가 없는 유사 유닛 구조체

필드가 아예 없는 구조체를 정의할 수도 있다. 이는 튜플 타입에서 다룬 유닛 타입 `()`와 비슷하게 동작하므로 유닛 구조체(unit-like structs)라 지칭한다.
유사 유닛 구조체는 어떤 타입에 대해 트레이트를 구현하고 싶지만 타입 내부에 어떤 데이터를 저장할 필요 없을때 유용하다.

```rust
struct AlwaysEqual;

fn main() {
    let subject = AlwaysEqual;
}
```
