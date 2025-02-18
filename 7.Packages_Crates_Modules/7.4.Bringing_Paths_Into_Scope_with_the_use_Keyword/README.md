# Bringing Paths into Scope with the use Keyword

`use` 키워드를 한번 사용하여 어떤 경로의 단축경로를 만들 수 있고, 스코프 안쪽 어디서라도 짧은 이름을 사용할 수 있다.

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```

`use`가 사용된 특정한 스코프에만 단축경로가 만들어진다는 것을 주의.

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

mod customer {
    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
    }
}
```

위는 스코프가 다르므로 에러가 난다.

## 보편적인 use 경로 작성법

함수의 부모 모듈을 `use` 키워드로 가져와서 함수를 호출할 때 부모 모듈을 특정해야 된다. 함수 호출 시 부모 모듈을 특정하면 전체 경로를 반복하는 것을 최소화하면서도 함수가 로컬에 정의되어 있지 않음을 명백히 보여준다.

한편, `use` 키워드로 구조체나 열거형 등의 타 아이템을 가져올 시에는 전체 경로를 작성하는 것이 보편적이다.

## as 키워드로 새로운 이름 제공하기

`use` 키워드로 동일한 이름의 타입을 스코프로 여러개 가져올 경우 해결방법이 있다. 경로 뒤에 `as` 키워드를 작성하고, 새로운 이름이나 타입 별칭을 작성하면 된다.

```rust
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    // --생략--
}

fn function2() -> IoResult<()> {
    // --생략--
}
```

## pub use로 다시 내보내기

`use` 키워드로 이름을 가져올 경우, 해당 이름은 새 위치의 스코프에서 비공개가 된다. `pub`와 `use`를 결합하면 코드를 호출하는 코드가 해당 스코프에 정의된 것처럼 해당 이름을 참조할 수 있다. 이 기법은 아이템을 스코프로 가져오는 동시에 다른 곳에서 아이템을 가져갈 수 있도록 만들기 때문에, 다시 내보내기(re-exporting)라고 한다.

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```

위의 `pub use` 가 루트 모듈로부터 `hosting` 모듈을 다시 내보냈으므로, 외부 코드는 `restaurant::hosing::add_to_waitlist()` 경로를 사용할 수 있다.

## 외부 패키지 사용하기

Cargo.toml

```toml
rand = "0.8.5"
```

```rust
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
}
```

## 중첩 경로를 사용하여 대량의 use 나열을 정리하기

```rust
// --생략--
use std::cmp::Ordering;
use std::io;
// --생략--
```

```rust
// --생략--
use std::{cmp::Ordering, io};
// --생략--
```

```rust
use std::io;
use std::io::Write;

use std::io::{self, Write};
```

## 글롭 연산자

경로에 글롭(glob) 연산자 `*` 를 붙이면 경로안에 정의된 모든 공개 아이템을 가져올 수 있다.

```rust
use std::collections::*;
```
