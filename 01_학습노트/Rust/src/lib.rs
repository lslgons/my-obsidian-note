//! 챕터 8: 모듈과 테스트 — 코드를 구조화하고 검증하기
//!
//! 실행: cargo test
//!
//! lib.rs는 "라이브러리 크레이트"입니다. src/bin/ 아래의 실행 예제들과 달리
//! 다른 코드에서 `use rust_tutorial::...` 로 가져다 쓸 수 있습니다.
//! 이 파일에서는 모듈 구성, 가시성(pub), 유닛 테스트를 배웁니다.

// ── 1. 모듈: 코드를 논리적으로 묶는 상자 ─────────────────────────
pub mod math {
    /// 두 수의 합 (doc comment: `///`는 문서를 만들 때 사용됩니다)
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    // pub이 없으므로 모듈 밖에서는 접근할 수 없습니다 (비공개).
    fn secret() -> i32 {
        42
    }

    /// 나눗셈 — 0으로 나누면 None (2장의 Option 재등장!)
    pub fn divide(a: f64, b: f64) -> Option<f64> {
        if b == 0.0 {
            None
        } else {
            Some(a / b)
        }
    }

    // 중첩 모듈도 가능합니다.
    pub mod stats {
        pub fn mean(values: &[f64]) -> Option<f64> {
            if values.is_empty() {
                None
            } else {
                Some(values.iter().sum::<f64>() / values.len() as f64)
            }
        }
    }
}

// ── 2. 별개의 모듈: 할 일 항목 (9장에서 확장됩니다) ──────────────
pub mod todo {
    use std::fmt;

    // #[derive]: 컴파일러가 표준 트레이트 구현을 자동 생성해 줍니다.
    #[derive(Debug, Clone, PartialEq)]
    pub struct Task {
        pub title: String,
        pub done: bool,
    }

    impl Task {
        pub fn new(title: &str) -> Self {
            Task {
                title: title.to_string(),
                done: false,
            }
        }

        pub fn finish(&mut self) {
            self.done = true;
        }
    }

    // Display 트레이트 구현: { } 로 출력할 때의 모양을 결정합니다.
    impl fmt::Display for Task {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let mark = if self.done { "✓" } else { " " };
            write!(f, "[{mark}] {}", self.title)
        }
    }
}

// ── 3. 유닛 테스트 ────────────────────────────────────────────────
// `cargo test`를 실행할 때만 컴파일됩니다.
// 각 함수가 독립적으로 실행되고, panic 하거나 assert! 를 통과하지 못하면 실패합니다.
#[cfg(test)]
mod tests {
    use super::math;
    use super::todo::Task;

    #[test]
    fn add_works() {
        assert_eq!(math::add(2, 3), 5);
        assert_eq!(math::add(-1, 1), 0);
    }

    #[test]
    fn divide_by_zero_is_none() {
        assert_eq!(math::divide(10.0, 2.0), Some(5.0));
        assert_eq!(math::divide(10.0, 0.0), None);
    }

    #[test]
    fn mean_of_empty_is_none() {
        assert_eq!(math::stats::mean(&[]), None);
        assert_eq!(math::stats::mean(&[1.0, 2.0, 3.0]), Some(2.0));
    }

    #[test]
    fn task_lifecycle() {
        let mut task = Task::new("Rust 공부하기");
        assert_eq!(task.to_string(), "[ ] Rust 공부하기");

        task.finish();
        assert!(task.done);
        assert_eq!(task.to_string(), "[✓] Rust 공부하기");
    }

    // 패닉(실패)을 테스트하는 방법도 있습니다.
    #[test]
    #[should_panic]
    fn assert_failures() {
        assert_eq!(1, 2); // 이 테스트는 일부러 실패시킵니다
    }
}

// lib.rs 자체는 실행 파일이 아니므로 main이 없습니다.
// 실행 예제는 src/bin/ 아래에 있습니다.
