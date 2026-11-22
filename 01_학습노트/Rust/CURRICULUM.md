# Rust 학습 커리큘럼

`std`만 사용하는 예제로 구성되어 있어, 추가 의존성(크레이트) 설치 없이 바로 실행할 수 있습니다.

## 커리큘럼 구성

| 챕터 | 주제 | 핵심 개념 | 바이너리 |
|------|------|-----------|----------|
| 1 | 시작하기 | 변수, 가변성, 상수(`const`), 섀도잉, 기본 타입, 함수 | `ch01_hello` |
| 2 | 제어 흐름 | `if`/`match`, `if let`, `while let`, 루프, 레이블 | `ch02_control_flow` |
| 3 | 소유권 | 소유권 이동, `Copy`/`Clone`, 참조와 borrow, 슬라이스 | `ch03_ownership` |
| 4 | 구조체와 열거형 | struct, 튜플 구조체, enum, 패턴 매칭, `Option`/`Result` | `ch04_structs_enums` |
| 5 | 컬렉션과 에러 처리 | `Vec`, `HashMap`, `String`, `?` 연산자 | `ch05_collections_errors` |
| 6 | 제네릭과 트레이트 | 제네릭, 트레이트, trait bound, 라이프타임 입문 | `ch06_generics_traits` |
| 7 | 클로저와 이터레이터 | 클로저, iterator adapter, `map`/`filter`/`fold` | `ch07_closures_iterators` |
| 8 | 스마트 포인터 | `Box<T>`, `Rc<T>`, `RefCell<T>`, 내부 가변성 | `ch08_smart_pointers` |
| 9 | 동시성 | 스레드(`spawn`), 채널(`mpsc`), `Arc<Mutex<T>>` | `ch09_concurrency` |
| 10 | 모듈과 테스트 | 모듈 시스템, 가시성, 유닛 테스트 | `cargo test` |
| 11 | 미니 프로젝트: 할 일 CLI | 지금까지 배운 것 종합 | `todo` |

## 학습 순서 및 예상 소요 시간

- 총 11개 섹션, 하루 1~2챕터 기준 약 1~2주
- 3장(소유권)과 슬라이스가 첫 번째 벽입니다. 여기서 시간을 들여 천천히 보세요.
- 6장(라이프타임)과 8·9장(스마트 포인터, 동시성)에서 심화 개념을 다룹니다.

## 실행 방법

Rust 설치 후([설치 가이드](#rust-설치)) 프로젝트 루트에서:

```bash
# 챕터별 예제 실행
cargo run --bin ch01_hello
cargo run --bin ch02_control_flow
cargo run --bin ch03_ownership
cargo run --bin ch04_structs_enums
cargo run --bin ch05_collections_errors
cargo run --bin ch06_generics_traits
cargo run --bin ch07_closures_iterators
cargo run --bin ch08_smart_pointers
cargo run --bin ch09_concurrency

# 테스트 실행 (10장)
cargo test

# 미니 프로젝트 실행 (11장)
cargo run --bin todo -- add "Rust 공부하기"
cargo run --bin todo -- list
cargo run --bin todo -- done 1
```

## 권장 학습 방법

1. **먼저 실행**: 챕터 예제를 그대로 실행해서 출력을 확인합니다.
2. **코드 읽기**: 소스 파일의 주석을 따라 읽습니다. 각 섹션이 하나의 개념입니다.
3. **일부러 고장내기**: 주석 처리된 코드의 주석을 풀고 컴파일 에러를 확인합니다.
   소유권/빌림 에러를 직접 겪는 것이 가장 빠른 학습법입니다.
4. **과제**: 각 챕터 끝의 "연습 과제"를 직접 풀어봅니다.

## Rust 설치

Windows: https://rustup.rs 안내대로 `rustup-init.exe`를 받아 실행하거나,

```powershell
winget install Rustlang.Rustup
```

설치 후 새 터미널에서 `rustc --version`으로 확인합니다. VS Code를 쓰고 있다면
`rust-analyzer` 확장을 함께 설치하는 것을 권장합니다.
