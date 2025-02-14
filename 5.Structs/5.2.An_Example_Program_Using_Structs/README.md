# An Example Program Using Structs

## 트레이트 파생으로 유용한 기능 추가

`println!` 같은 함수는 기본 타입들에 대한 `Display`가 기본적으로 구현되어 있다. 하지만 구조체와 같이 애매한 경우 `Display`가 제공되지 않는다.

`{:?}` 는 `println!` 에 `Debug` 라는 출력 형식을 사용. 개발자가 디버깅하는 동안 값을 보게 해주는 트레이트.

러스트는 디버깅 정보를 출력하는 기능을 자체적으로 가지고 있다. 하지만 우리가 만든 구조체에서 해당 기능을 적용하려면 명시적인 동의가 필요하므로, `#[derive(Debug)]` 외부 속성을 작성해 줘야 한다.

필드가 많을 경우 읽기 쉽게 `{:#?}` 을 사용할 수 있다.

그 외 `Debug` 포맥을 사용하여 값을 출력하는 방법으로는 `dbg!` 매크로를 사용하는 것이다.
이는 표현식의 소유권을 가져와서, (참조자를 사용하는 `println!` 과는 다름) 코드에서 `dbg!` 매크로를 호출한 파일 및 라인 번호를 결과값으로 함께 출력하고 다시 소유권을 반환한다.

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);
}
```
