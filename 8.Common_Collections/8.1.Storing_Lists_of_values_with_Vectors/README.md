# Storing Lists of Values with Vectors

`Vec<T>`

- 메모리에서 모든 값을 서로 이웃하도록 배치하는 단일 데이터 구조에 하나 이상의 값을 저장
- 같은 타입의 값만 저장
- 파일 내의 텍스트 라인들이나 장바구니의 품목 가격 같은 아이템 목록을 저장하는 상황일 때 유용

## 새 벡터 만들기

```rust
    let v: Vec<i32> = Vec::new();
```

대부분의 경우 초기값과 함께 제공하므로 타입 명시가 필요하지 않다. 편의를 위해 `vec!` 매크로를 제공.

```rust
    let v = vec![1, 2, 3];
```

## 벡터 업데이트하기

벡터는 `push` 메서드를 통해 요소를 추가할 수 있다.

```rust
    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
```

## 벡터 요소 읽기

벡터에 저장된 값을 참조하는 방법은 인덱싱과 `get` 메서드 두 가지가 있다.

```rust
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }
```

`&`와 `[]`를 사용하면 인덱스 값에 위치한 요소의 참조자를 얻게 된다.

`get` 함수에 인덱스를 매개변수로 넘기면, `match` 를 통해 처리할 수 있는 `Option<&T>` 를 얻게 된다.

동작을 어떻게 할 것인가에 따라 둘 중 하나를 선택해서 사용한다. 만약 벡터의 인덱스를 넘는 범위를 가지게 될 경우 인덱스 방식은 에러를 발생시켜 프로그램을 종료시키고, `get` 메서드를 사용하면 `None`에 대해 처리할 수 있다.

프로그램에 유효한 참조자가 있다면, 대여 검사기 (borrow checker)가 소유권 및 대여 규칙을 집행하여 이 참조자와 벡터의 내용물로부터 얻은 다른 참조자들이 계속 유효하게 남아있도록 보장한다.

같은 스코프내에서는 가변 참조자와 불변 참조라를 동시에 가질 수 없다. 아래 예제는 동작하지 않는다.

```rust
    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0];

    v.push(6);

    println!("The first element is: {first}");
```

벡터는 모든 요소가 서로 붙어서 메모리에 저장된다. 그리고 새로운 요소를 벡터 끝에 추가할 경우, 현재 벡터 메모리 위치에 새로운 요소를 추가할 공간이 없다면, 다른 넉넉한 곳에 메모리를 새로 할당하고 기존 요소를 새로 할당한 공간에 복사한다. 이 경우, 기존 요소의 참조자는 해제된 메모리를 가리키게 되기 때문에 이러한 상황을 대여 규칙으로 막아둔 것이다.

## 벡터 값에 대해 반복하기

벡터는 인덱스를 사용하여 한 번에 하나의 값에 접근하기보다는 모든 요소에 대한 반복 처리를 한다.

```rust
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }
```

```rust
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
```

## 열거형을 이용해 여러 타입 저장하기

벡터 내에 다른 타입의 값들을 저장할 필요가 있다면 열거형을 정의하여 사용할 수 있다.

```rust
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
```

## 벡터가 버려지면 벡터의 요소도 버려진다.

`struct`와 마찬가지로, 벡터는 스코프를 벗어날 때 해제 된다.

```rust
    {
        let v = vec![1, 2, 3, 4];

        // v를 가지고 작업하기
    } // <- 여기서 v가 스코프 밖으로 벗어나고 해제됩니다
```
