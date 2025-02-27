# Traits: Defining Shared Behavior

트레이트는 특정한 타입이 가지고 있으면서 다른 타입과 공유할 수 있는 기능을 정의.
트레이트를 사용하면 공통된 기능을 추상적으로 정의할 수 있다.
트레이트 바운드 (trait bound)를 이용하면 어떤 제네릭 타입 자리에 특정한 동작을 갖춘 타입이 올 수 있음을 명시할 수 있다.

- 다른 언어의 인터페이스와 유사

## 트레이트 정의하기

트레이트 정의는 메서드 시그니처를 그룹화하여 특정 목적을 달성하는 데 필요한 일련의 동작을 정의하는 것이다.

예를 들어 다양한 종류 및 분량의 텍스트를 갖는 여러 구조체가 있다 하자: `NewsArticle` 구조체는 특정 지역에서 등록된 뉴스 기사를 저장하고, `Tweet` 구조체는 최대 280자의 콘텐츠와 해당 트윗이 새 트윗인지, 리트윗인지, 다른 트윗의 대답인지를 나타내는 메타데이터를 저장한다.

`NewsArticle`이나 `Tweet` 인스턴스에 저장된 데이터를 종합해 보여주는 종합 미디어 라이브러리 크레이트 `aggregator`를 만든다고 가정하자. 이를 위해서 각 타입의 요약 정보를 얻어와야 하는데, 인스턴스에서 `summarize` 메서드를 호출하여 이요약 정보를 가져오려고 한다.

```rust
pub trait Summary {
    fn summarize(&self) -> String;
}
```

`trait` 키워드 다음에 트레이트의 이름 `Summary`를 작성해 트레이트를 선언.
트레이트를 `pub`로 선언ㅇ하여 이 크레이트에 의존하는 다른 크레이트가 이 크레이트를 사용할 수 있도록 했다.
중괄호 안에는 이 트레이트를 구현할 타입의 동작을 묘사하는 메서드 시그니처를 선언.

메서드 시그니처 뒤에는 중괄호로 시작하여 메서드를 구현하는 대신 세미콜론을 넣었다.

이 트레이트를 구현하는 각 타입이 메서드에 맞는 동작을 직접 제공해야 한다.

컴파일러는 `Summary`트레이트가 있는 모든 타입에 정확히 이와 같은 시그니처의 `summarize` 메서드를 가지고 있도록 강제할 것이다.

트레이트는 본문에 여러 메서드를 가질 수 있다: 메서드 시그니처는 한 줄에 하나씩 나열되며, 각 줄은 세미콜론으로 끝난다.

## 특정 타입에 트레이트 구현하기

```rust
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
```

트레이트는 `impl` 뒤에 구현하고자 하는 트레이트 이름을 적고, 그 다음 `for` 키워드와 트레이트를 구현할 타입명을 명시한다.

라이브러리가 `NewsArticle`과 `Tweet`에 대한 `Summary` 트레이트를 구현했으니, 크레이트 사용자는 `NewsArticle`과 `Tweet` 인스턴스에 대해 보통의 메서드를 호출하는 것과 같은 방식으로 트레이트 메서드를 호출할 수 있다.

유일한 차이점은 크레이트 사용자가 타입 뿐만 아니라 트레이트도 스코프로 가져와야 한다는 것이다.

```rust
use aggregator::{Summary, Tweet};

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
}
```

트레이트 구현에는 한 가지 제약사항이 있는데, 이는 트레이트나 트레이트를 구현할 타입 둘 중 하나는 반드시 자신의 크레이트 것이어야 해당 타입에 대한 트레이트를 구현할 수 있다.

예를 들어 우리가 만든 `aggregator` 크레이트의 일부 기능으로 `Tweet` 타입에 표준 라이브러리 트레이트인 `Display` 등을 구현할 수 있다. `Tweet` 타입이 우리가 만든 `aggregator` 크레이트의 타입이기 때문이다.
또한`aggregator` 크레이트에서 `Vec<T>` 타입에 `Summary` 트레이트를 구현할 수도 있다. 마찬가지로 `Summary` 트레이트가 우리가 만든 `aggregator` 크레이트의 트레이트이기 때문인다.

하지만 외부 타입에 외부 크레이트를 구현할 수는 없다. 예를 들어, 우리가 만든 `aggregator` 크레이트에서는 `Vec<T>` 에 대한 `Display` 트레이트를 구현할 수 없다. `Vec<T>`, `Display` 모두 우리가 만든 크레이트가 아닌 표준 라이브러리에 정의되어 있기 때문이다. 이 제약은 프로그램의 특성 중 하나인 일관성(coherence), 보다 자세히는 고아 규칙(orphan rule)에서 나온다.

이 규칙이 없다면 두 크레이트가 동일한 타입에 동일한 트레이트를 구현할 수 있게 되고, 러스트는 어떤 구현체를 사용해야 할 지 알 수 없게 된다.

## 기본 구현

타입에 트레이트를 구현할 때마다 모든 메서드를 구현할 필요는 없도록 기본 동작을 제공할 수 있다.
특정한 타입에 트레이트를 구현할 때 기본 동작을 유지할지 혹은 오버라이드(override) 할지 선택할 수 있다.

```rust
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}
```

기본 구현을 사용하려면 `impl Summary for NewsArticle {}` 처럼 비어있는 `impl` 블록을 명시하면 된다.

기본 구현 안쪽에서 트레이트의 다른 메서드를 호출할 수도 있다. 호출할 다른 메서드가 기본 구현을 제공하지 않아도 상관없다. 이런방식으로 트레이트는 구현자에게 작은 부분만 구현을 요구하면서 유용한 기능을 많이 제공할 수 있다.

```rust
pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}
```

위 `Summary`를 어떤 타입에 구현할 때는 `summarize_author`만 정의하면 된다.

```rust
impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}
```

## 매개변수로서의 트레이트

`Summary` 트레이트를 사용하여, `Summary` 트레이트를 구현하는 어떤 타입의 `item` 매개변수에서 `summarize` 메서드를 호출하는 `notify` 함수를 정의 해보자:

```rust
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
```

`item` 매개변수의 구체적 타입 명시 대신 `impl` 키워드와 트레이트 이름을 명시했다.

### 트레이트 바운드 문법

`impl Trait` 문법은 간단하지만, 이는 트레이트 바운드(trait bound)의 syntax sugar 이다.

트레이트 바운드는 다음과 같이 생겼다:

```rust
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
```

`impl Trait` 문법이 단순한 상황에서는 편리하고 코드를 간결하게 만들어 주는 반면, 트레이트 바운드 문법은 더 복잡한 상황을 표현할 수 있다.

예를 들어, `Summary` 를 구현하는 두 매개변수를 전달받는 함수를 구현할 때 `impl Trait` 문법으로 표현하면 다음과 같다:

```rust
pub fn notify(item1: &impl Summary, item2: &impl Summary) {
```

`item1`과 `item2`가 서로 다른 타입이어도 상관없다면 `impl Trait` 문법을 사용해도 적절하나 만약 두 매개변수가 같은 타입으로 강제되어야 한다면 아래와 같이 트레이트 바운드를 사용해야 한다.

```rust
pub fn notify<T: Summary>(item1: &T, item2: &T) {
```

`item1`, `item2` 인수 값의 구체적인 타입이 반드시 동일하도록 제한한다.

### + 구문으로 트레이트 바운드를 여럿 지정하기

트레이트 바운드는 여러 개 지정될 수도 있다. `notify`에서 `item`의 `summarize` 메서드뿐만 아니라 출력 포맷팅까지 사용하고 싶다고 가정해 보자: 즉 `notify`를 정의할 때 `item`이 `Display`, `Summary`를 모두 구현해야 하도록 지정해야 한다.
`+` 문법을 사용하면 트레이트를 여러개 지정할 수 있다:

```rust
pub fn notify(item: &(impl Summary + Display)) {
```

`+` 구문은 제네릭 타입의 트레이트 바운드에도 사용할 수 있다:

```rust
pub fn notify<T: Summary + Display>(item: &T) {
```

### where 조항으로 트레이트 바운드 정리하기

너무 많은 트레이트 바운드가 있으면 가독성을 해치기 때문에 러스트는 트레이트 바운드를 함수 시그니처 뒤의 `where` 조항에 명시하는 대안을 제공한다:

```rust
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
```

이것을 다음곽 같이 `where` 조항을 사용할 수 있다:

```rust
fn some_function<T, U>(t: &T, u: :&U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{}
```

## 트레이트를 구현하는 타입을 반환하기

`impl Trait` 문법을 반환 타입 위치에 써서 어떤 트레이트를 구현한 타입의 값을 반환시키는 데에도 사용할 수 있다:

```rust
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}
```

위의 경우 `returns_summarizable`는 `Tweet`을 반환하지만, 이 함수를 호출하는 쪽의 코드에서는 구체적인 타입을 알 필요가 없다.

구현되는 트레이트로 반환 타입을 명시하는 기능은 클로저 및 반복자의 컨텍스트에서 굉장히 유용하다.
클로저와 반복자는 컴파일러만 아는 타입이나, 직접 명시하기에는 굉장히 긴 타입을 생성한다.
`impl Trait` 문법을 사용하면 굉장히 긴 타입을 직접 작성할 필요 없이 `Iterator` 트레이트를 구현하는 어떤 타입이라고 간결하게 지정할 수 있다.

하지만, `impl Trait` 문법을 쓴다고 해서 다양한 타입을 반환할 수는 없다. 다음은 반환형을 `impl Summary`로 지정하고 `NewsArticle`, `Tweet` 중 하나를 반환하는 코드 예시이다. 이 코드는 컴파일할 수 없다:

```rust
fn returns_summarizable(switch: bool) -> impl Summary {
    if switch {
        NewsArticle {
            headline: String::from(
                "Penguins win the Stanley Cup Championship!",
            ),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best \
                 hockey team in the NHL.",
            ),
        }
    } else {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from(
                "of course, as you probably already know, people",
            ),
            reply: false,
            retweet: false,
        }
    }
}
```

위 코드는 `impl Trait` 문법이 컴파일러 내에 구현된 방식으로 인한 제약 때문에 허용되지 않는다. 이는 17장에서 알아볼 예정.

## 트레이트 바운드를 사용해 조건부로 메서드 구현하기

제네릭 타입 매개변수를 사용하는 `impl` 블록에 트레이트 바운드를 사용하면, 지정된 트레이트를 구현하는 타입에 대해서만 메서드를 구현할 수도 있다.

예를 들어, 아래의 `Pair<T>` 타입은 언제나 새로운 `Pair<T>` 인스턴스를 반환하는 `new` 함수를 구현한다. 하지만 그 다음의 `impl` 블록에서는 어떤 `T` 타입이 비교를 가능하게 해주는 `PartialOrd` 트레이트와 출력을 가능하게 만드는 `Display` 트레이트를 모두 구현한 타입인 경우에 대해서만 `cmp_display` 메서드를 구현하고 있다.

```rust
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
```

타입이 특정 트레이트를 구현하는 경우에만 해당 타입에 트레이트를 구현할 수도 있다.
트레이트 바운드를 만족하는 모든 타입에 대해 트레이트를 구현하는 것을 포괄 구현(blanket implementations)라 하며, 이는 러스트 표준 라이브러리 내에서ㅓ 광범위하게 사용된다.

예를 들어, 표준 라이브러리는 `Display` 트레이트를 구현하는 모든 타입에 `ToString` 트레이트도 구현한다. 표준 라이브러리의 `impl` 블록은 다음과 비슷하게 생겼다:

```rust
impl<T: Display> ToString for T {
    // --생략--
}
```

`Display` 트레이트가 구현된 모든 타입에서 `to_string()` 메서드를 호출할 수 있는건 표준 라이브러리의 이 포괄 구현 덕분이다.
포괄 구현은 트레이트 문서 페이지의 '구현자(Implementors)' 절에 있다.

트레이트와 트레이트 바운드를 사용하면 제네릭 타입 매개변수로 코드 중복을 제거하면서 특정 동작을 하는 제네릭 타입이 필요하다는 사실을 컴파일러에게 전달할 수 있다. 컴파일러는 트레이트 바운드를 이용하여 코드에 사용된 구체적인 타입들이 올바른 동작을 제공하는지 검사한다.
동적 타입 언어에서는 해당 타입이 정의하지 않은 메서드를 호출하면 런타임에 에러가 방생한다. 하지만 러스트는 컴파일 시점에 에러를 발생시켜 코드를 실행하기도 전에 문제를 해결하도록 강제한다. 따라서 럼타임에 해당 동작을 구현하는지 검사하는 코드를 작성할 필요가 없다. 러스트는 제네릭의 유연성과 성능 둘 다 놓치지 않는다.
