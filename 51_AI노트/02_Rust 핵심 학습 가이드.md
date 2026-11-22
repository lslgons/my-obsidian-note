---
created: 2026-11-22
modified: 2026-11-22
---

# Rust 핵심 학습 가이드

공식 문서 [The Rust Programming Language (한국어 번역본)](https://doc.rust-kr.org/) 기반의 핵심 요약과 `01_학습노트/Rust` 예제 코드 매핑 가이드입니다.

---

## 1. 커리큘럼 및 예제 코드 매핑

| 공식 문서 장 (doc.rust-kr.org) | 핵심 개념 요약 | 대응 예제 바이너리 |
|---|---|---|
| **3장. 일반 프로그래밍 개념** | 불변/가변 변수(`mut`), 상수(`const`), 섀도잉(Shadowing), 스칼라/복합 타입, 함수 | `src/bin/ch01_hello.rs` |
| **3장 & 6장. 제어 흐름 및 패턴 매칭** | 식(Expression)으로서의 `if`, `match`, `if let`, `while let`, 루프(`loop`/`while`/`for`), 루프 레이블 | `src/bin/ch02_control_flow.rs` |
| **4장. 소유권 이해하기** | 소유권 3대 규칙, 이동(Move), 복사(`Copy`/`Clone`), 참조와 대여(`&`, `&mut`), NLL, 슬라이스(`&str`, `&[T]`) | `src/bin/ch03_ownership.rs` |
| **5장 & 6장. 구조체와 열거형** | struct 정의 및 메서드(`impl`), 튜플 구조체, 유사 단위 구조체, enum, `Option<T>` | `src/bin/ch04_structs_enums.rs` |
| **8장 & 9장. 컬렉션과 에러 처리** | `Vec<T>`, `HashMap<K, V>`, `String`, `panic!`, `Result<T, E>`, 에러 전파(`?` 연산자) | `src/bin/ch05_collections_errors.rs` |
| **10장. 제네릭, 트레이트, 수명** | 제네릭 함수/구조체, 트레이트 정의 및 기본 구현, Trait Bound, 정적 디스패치 vs 동적 디스패치(`&dyn Trait`) | `src/bin/ch06_generics_traits.rs` |
| **13장. 반복자와 클로저** | 클로저 환경 캡처(`Fn`, `FnMut`, `FnOnce`), `move` 키워드, 반복자 어댑터(`map`, `filter`) 및 소비자(`sum`, `fold`) | `src/bin/ch07_closures_iterators.rs` |
| **15장. 스마트 포인터** | `Box<T>`(힙 할당, 재귀적 타입), `Rc<T>`(참조 카운팅 다중 소유), `RefCell<T>`(런타임 대여 검사, 내부 가변성) | `src/bin/ch08_smart_pointers.rs` |
| **16장. 무서움 없는 동시성** | `thread::spawn`, `move` 클로저, `mpsc::channel` 메시지 전달, `Arc<Mutex<T>>` 상태 공유 | `src/bin/ch09_concurrency.rs` |
| **7장 & 11장. 모듈과 테스트** | 모듈 트리(`mod`), 가시성(`pub`), 유닛 테스트(`#[test]`, `assert_eq!`, `#[should_panic]`) | `src/lib.rs` (`cargo test`) |
| **종합 실습** | CLI 파싱, 파일 입출력, 에러 처리, 트레이트 종합 실습 | `src/bin/todo.rs` |

---

## 2. 주요 개념별 핵심 포인트

### 2.1 변수 가리기 (Shadowing) vs 가변성 (`mut`)
* `mut` 변수는 동일한 타입 내에서 값만 변경할 수 있습니다.
* `let`을 사용한 **섀도잉(Shadowing)**은 새로운 변수를 선언하므로 **타입을 변경**할 수 있으며, 변환 후 불변성을 유지할 수 있습니다.
  ```rust
  let spaces = "   "; // &str
  let spaces = spaces.len(); // usize
  ```

### 2.2 간결한 제어 흐름 (`if let` & `while let`)
* `match`는 모든 경우(exhaustiveness)를 처리해야 하지만, 관심 있는 한 가지 경우만 처리하고 나머지는 무시하고자 할 때는 `if let`이 유용합니다.
* `while let`은 조건 패턴이 매칭되는 동안 반복 수행합니다 (예: 스택에서 요소를 계속 꺼낼 때).

### 2.3 소유권과 슬라이스 (Slice)
* 슬라이스는 컬렉션의 연속된 일부분을 참조하는 **소유권 없는 참조자**입니다.
* 문자열 리터럴(`"hello"`)은 바이너리 특정 지점을 가리키는 `&str`(문자열 슬라이스)입니다.
* 슬라이스를 사용하면 원본 데이터가 변경되는 것을 컴파일 타임에 방지할 수 있습니다.

### 2.4 스마트 포인터 (Smart Pointers)
* **`Box<T>`**: 크기가 컴파일 타임에 정해지지 않는 재귀적 데이터 구조(예: Cons 리스트)나 힙 메모리 할당에 사용.
* **`Rc<T>`**: 단일 스레드에서 하나의 데이터에 다중 소유권이 필요할 때 참조 횟수를 세어 마지막 소유자가 사라질 때 해제.
* **`RefCell<T>`**: 내부 가변성 패턴을 통해 불변 참조자임에도 내부 값을 수정 가능하게 함 (대여 규칙을 런타임에 검사).

### 2.5 동시성 (Fearless Concurrency)
* **`mpsc::channel`**: "메모리를 공유하여 통신하지 말고, 통신을 통해 메모리를 공유하라"는 철학을 따르는 채널 통신.
* **`Arc<Mutex<T>>`**: 멀티스레드 환경에서 여러 스레드가 데이터에 안전하게 접근하기 위한 원자적 참조 카운팅(`Arc`)과 상호 배제 잠금(`Mutex`).

---

## 3. 실행 방법

프로젝트 디렉터리(`d:\my-obsidian-note\01_학습노트\Rust`)에서 실행:

```bash
# 전체 테스트 실행
cargo test

# 특정 예제 바이너리 실행
cargo run --bin ch02_control_flow
cargo run --bin ch08_smart_pointers
cargo run --bin ch09_concurrency
```
