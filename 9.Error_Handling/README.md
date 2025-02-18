# Error Handling

러스트는 에러를 복구 가능한(recoverable) 에러와 복구 불가능한(unrecoverable) 에러 두 가지 범주로 묶는다.
러스트에는 예외 처리와 같은 메커니즘이 없다.

대신 복구 가능한 에러를 위한 `Result<T, E>` 타입과 복구 불가능한 에러가 발생했을 때 프로그램을 종료하는 `panic!` 매크로가 있다.
