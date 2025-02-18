# Storing Keys with Associated Values in Hash Maps

`HashMap<K,V>` 타입은 `K` 타입의 키와 `V` 타입의 값에 대해 해시 함수를 사용하여 매핑한 것을 저장하는데, 이 해시 함수는 키와 값을 메모리 어디에 저장할지 결정한다.

## 새로운 해시맵 생성하기

빈 해시맵을 생성하는 방법으로 `new` 를 사용한 뒤 `insert`를 이용하여 요소를 추가할 수 있다.

```rust
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
```

`HashMap`은 `use`로 가져와야 한다. 해시맵은 일반적인 컬렉션 중 덜 자주 사용되기 때문에 프렐루드의 자동으로 가져오는 기능에는 포함되어 있지 않다.

해시맵의 모든 키는 서로 같은 타입이어야 하고, 모든 값도 같은 타입이어야 한다.

## 해시맵의 값 접근하기

`get` 메서드에 키를 제공하여 해시맵으로부터 값을 얻을 수 있다.

```rust
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
```

`get` 메서드는 `Option<&V>` 를 반환한다. 만일 이 해시맵에 해당하는 키가 없다면 `None`을 반환한다.

해시맵 내의 키/값 쌍에 대한 반복 작업을 수행할 수 있다:

```rust
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }
```

## 해시맵과 소유권

`i32` 처럼 `Copy` 트레이트를 구현한 타입의 값은 해시맵 안으로 복사된다. `String` 처럼 소유권이 있는 값의 경우, 아래의 예제 처럼 값들이 이동되어 해시맵이 그 값의 소유자가 된다:

```rust
    use std::collections::HashMap;

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name과 field_value는 이 시점부터 유효하지 않습니다.
    // 사용을 시도해보고 무슨 컴파일러 에러가 발생하는 알아보세요!
```

## 해시맵 업데이트 하기

해시맵의 데이터를 변경하고 싶을 때는 키에 이미 값이 할당되어 있을 경우에 대한 처리 방법을 결정해야 된다.

### 값을 덮어쓰기

```rust
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);
```

25가 출련된다.

### 키가 없을 때만 키와 값 추가하기

해시맵은 특정 키에 값이 있는지 검사하는 `entry` 라는 특별한 API가 있다.

`entry` 함수의 반환 값은 열거형 `Entry` 인데, 해당 키가 있는지 혹은 없는지 나타낸다.

```rust
    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);
```

`Entry` 의 `or_insert` 메서드는 해당 키가 존재할 경우 `Entry` 키에 대한 연관된 값을 반환하도록 정의되어 있고, 그렇지 않을 경우 매개변수로 제공된 값을 해당 키에 대한 새 값으로 삽입하고 수정된 `Entry`에 대한 값을 반환한다.

## 예전 값에 기초하여 값을 업데이트 하기

```rust
    use std::collections::HashMap;

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
```

`split_whitespace` 메서드는 `text` 값을 공백문자로 나눈 서브 슬라이스에 대한 반복자를 반환한다.
`or_insert` 메서드는 실제로는 해당 키에 대한 값의 가변 참조자(`&mut V`)를 반환합니다. 여기서는 `count` 변수에 가변 참조자를 저장하였고, 여기에 값을 할당하기 위해 먼저 애스터리스크(`*`)를 사용하여 `count`를 역참조해야 합니다. 가변 참조자는 `for` 루프의 끝에서 스코프 밖으로 벗어나고, 따라서 모든 값의 변경은 안전하며 대여 규칙에 위배되지 않습니다.
