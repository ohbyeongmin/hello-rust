# Common Collections

러스트의 표준 라이브러리에는 컬렉션 (collection) 이라 불리는 매우 유용한 데이터 구조들이 포함되어 있다. 내장된 배열이나 튜플 타입과는 달리 이 컬렉션들이 가리키고 있는 데이터들은 힙에 저장된다. 이는 데이터의 양이 컴파일 타임에 결정되지 않아도 되며 프로그램 실행 중에 늘어나거나 줄어들 수 있다는 의미이다.

- 벡터(vector)는 여러 개의 값을 서로 붙어 있게 저장할 수 있도록 해준다.
- 문자열(string)은 문자(character)의 모음이다 `String` 타입은 전에도 다루었지만, 이번 장에 더 깊이 다룬다.
- 해시맵(hash map)은 어떤 값을 특정한 키와 연관지어 주도록 한다.
