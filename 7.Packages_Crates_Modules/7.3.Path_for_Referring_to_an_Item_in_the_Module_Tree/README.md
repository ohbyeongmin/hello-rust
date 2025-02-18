# Paths for Referring to an Item in the Module Tree

- 절대 경로(absolute path): 크레이트 루트로부터 시작되는 전체 경로. 외부 크레이트로부터의 코드에 대해서는 해당 크레이트 이름으로 절대 경로가 시작되고 현재의 크레이트로부터의 코드에 대해서는 `crate` 리터럴로부터 시작된다.

- 상대 경로(relative path): 현재의 모듈을 시작점으로 하여 `self`, `super` 혹은 현재 모듈 내의 식별자를 사용.

```rust
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // 절대 경로
    crate::front_of_house::hosting::add_to_waitlist();

    // 상대 경로
    front_of_house::hosting::add_to_waitlist();
}
```

러스트에서는 모든 아이템이 기본적으로 부모 모듈에 대해 비공개 이다. 부모 모듈 내 아이템은 자식 모듈 내 비공개 아이템을 사용할 수 없지만, 자식 모듈 내 아이템은 부모 모듈 내 아이템을 사용할 수 잇다.

`pub` 키워드를 사용하여 자식 모듈의 내부 구성 요소를 공개 함으로써 외부의 상위 모듈로 노출할 방법을 제공한다.

## pub 키워드로 경로 노출하기

다음과 같이 hosting 모듈을 공개하면 어떻게 될까?

```rust
mod front_of_house {
    pub mod hosting {
        fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // 절대 경로
    crate::front_of_house::hosting::add_to_waitlist();

    // 상대 경로
    front_of_house::hosting::add_to_waitlist();
}
```

에러가 발생한다. `pub` 키워드를 추가하여 모듈을 공개했지만, `hosting` 모듈의 내용은 여전히 비공개이다. 모듈을 공개했다고 해서 내용까지 공개되지는 않는다.

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // 절대 경로
    crate::front_of_house::hosting::add_to_waitlist();

    // 상대 경로
    front_of_house::hosting::add_to_waitlist();
}
```

## super 로 시작하는 상대 경로

`super` 로 시작하면 현재 모듈 혹은 크레이트 루트 대신 자기 부모 모듈부터 시작되는 상대 경로를 만들 수 있다. 이는 파일시스템 경로에서 `..` 로 시작하는 것과 동일하다.

```rust
fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}
```

## 구조체, 열거형을 공개하기

구조체 정의에 `pub`를 쓰면 구조체는 공개되지만, 구조체의 필드는 비공개로 유지된다. 공개 여부는 각 필드마다 정할 수 있다.

```rust
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    // 호밀 (Rye) 토스트를 곁들인 여름철 조식 주문하기
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // 먹고 싶은 빵 바꾸기
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // 다음 라인의 주석을 해제하면 컴파일되지 않습니다; 식사와 함께
    // 제공되는 계절 과일은 조회나 수정이 허용되지 않습니다
    // meal.seasonal_fruit = String::from("blueberries");
}
```

반대로, 열거형은 공개로 지정할 경우 모든 배리언트가 공개된다.

```rust
mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
```

열거형은 그 배리언트가 공개되지 않는다면 큰 쓸모가 없다.
