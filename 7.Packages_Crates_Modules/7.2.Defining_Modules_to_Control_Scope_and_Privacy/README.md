# Defining Modules to Control Scope and Privacy

이번 장에서 다룰 것
: Modules, Paths, `use`, `pub`, `as`, 외부 패키지, `glob` 연산자

## 모듈 치트 시트

- 크레이트 루트부터 시작: 크레이트를 컴파일할 때 컴파일러는 먼저 크레이트 루트 파일을 봅니다 (보통은 라이브러리 크레이트의 경우 src/lib.rs 혹은 바이너리 크레이트의 경우 src/main.rs입니다).

- 모듈 선언: 크레이트 루트 파일에는 새로운 모듈을 선언할 수 있습니다; `mod garden;`이라는 코드로 ‘garden’ 모듈을 선언할 수 있습니다. 컴파일러는 아래의 장소에서 이 모듈의 코드가 있는지 살펴볼 것입니다:

  - `mod garden` 뒤에 세미콜론 대신 중괄호를 써서 안쪽에 코드를 적은 인라인
  - src/garden.rs 파일 안
  - src/garden/mod.rs 파일 안

- 서브모듈 선언: 크레이트 루트가 아닌 다른 파일에서는 서브모듈 (submodule) 을 선언할 수 있습니다. 예를 들면 src/garden.rs 안에 `mod vegetables;`를 선언할 수도 있습니다. 컴파일러는 부모 모듈 이름의 디렉터리 안쪽에 위치한 아래의 장소들에서 이 서브모듈의 코드가 있는지 살펴볼 것입니다:

  - `mod vegetables` 뒤에 세미콜론 대신 중괄호를 써서 안쪽에 코드를 적은 인라인
  - src/garden/vegetables.rs 파일 안
  - src/garden/vegetables/mod.rs 파일 안

- 모듈 내 코드로의 경로: 일단 모듈이 크레이트의 일부로서 구성되면, 공개 규칙이 허용하는 한도 내에서라면 해당 코드의 경로를 사용하여 동일한 크레이트의 어디에서든 이 모듈의 코드를 참조할 수 있게 됩니다. 예를 들면, garden vegetables 모듈 안에 있는 `Asparagus` 타입은 `crate::garden::vegetables::Asparagus`로 찾아 쓸 수 있습니다.

- 비공개 vs 공개: 모듈 내의 코드는 기본적으로 부모 모듈에게 비공개 (private) 입니다. 모듈을 공개 (public) 로 만들려면, `mod` 대신 `pub mod`를 써서 선언하세요. 공개 모듈의 아이템들을 공개하려면 마찬가지로 그 선언 앞에 `pub`을 붙이세요.

- `use` 키워드: 어떤 스코프 내에서 `use` 키워드는 긴 경로의 반복을 줄이기 위한 어떤 아이템으로의 단축경로를 만들어 줍니다. `crate::garden::vegetables::Asparagus`를 참조할 수 있는 모든 스코프에서 `use crate::garden::vegetables::Asparagus;`로 단축경로를 만들 수 있으며, 그 이후부터는 스코프에서 이 타입을 사용하려면 `Asparagus`만 작성해주면 됩니다.
