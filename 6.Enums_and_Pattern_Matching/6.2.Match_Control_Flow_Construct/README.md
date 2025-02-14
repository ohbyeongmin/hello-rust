# The match Control Flow Construct

러스트의 `match`는 일련의 패턴에 대해 어떤 값을 비교한 뒤 어떤 패턴에 매칭되었는지를 바탕으로 코드를 수행한다.

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

각 갈래와 연관된 코드는 표현식이고, 이 매칭 갈래에서의 표현식의 결과로써 생기는 값은 전체 `match` 표현식에 대해 반환되는 값이다.

갈래의 코드가 짧다면 중괄호는 보통 사용하지 않는다. 만약 여러줄의 코드를 실행하고 싶으면 중괄호를 사용한다. 그렇게 되면 갈래 뒤에 붙이는 쉼표는 옵션이 된다.

```rust
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

## 값을 바인딩 하는 패턴

매치 갈래의 또 다른 유용한 기능은 패턴과 매칭된 값들의 일부분을 바인딩 할 수 있다.

```rust
#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --생략--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
```

```rust
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}
```

## Option<T> 를 이용하는 매칭

```rust
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
```

러스트의 매치는 철저(exhaustive)하기 때문에 모든 경우를 다 다루어야 한다. 그렇지 않을 경우 러스트가 알려준다.

## 포괄 패턴과 \_ 자리표시자

```rust
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn move_player(num_spaces: u8) {}

```

위 코드는 `u8` 이 가질 수 있는 모든 값을 나열하지 않아도 컴파일 된다. 왜냐하면 나머지 모든 값에 대해 마지막 패턴이 매칭되기 때문이다. 이러한 포괄(catch-all) 패턴은 `match`의 철저함을 만족시킨다.

패턴들은 순차적으로 평가되므로 마지막에 포괄적인 갈래를 위치시켜야 한다.

만약 포괄 패턴의 값을 사용할 필요가 없을 경우에는 `_`를 매칭한다.

```rust
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn reroll() {}
```

```rust
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
```
