//! 챕터 7: 클로저와 이터레이터 — 함수형 스타일의 Rust
//!
//! 실행: cargo run --bin ch07_closures_iterators

fn main() {
    // ── 1. 클로저(closure): 주변 환경을 "잡아둘 수 있는" 익명 함수 ─
    let add = |a: i32, b: i32| a + b; // 타입을 적을 수도 있고
    let square = |x| x * x; // 컴파일러가 추론하게 할 수도 있습니다

    println!("3 + 4 = {}", add(3, 4));
    println!("5² = {}", square(5));

    // ── 2. 클로저는 환경을 캡처합니다 ────────────────────────────
    let name = String::from("러스터");
    // 불변으로 캡처 — name을 계속 쓸 수 있습니다.
    let hello = || println!("안녕, {name}!");
    hello();
    println!("이름은 여전히 {name}");

    // 가변으로 캡처 — `mut` 필요
    let mut count = 0;
    let mut increment = || {
        count += 1;
        count
    };
    increment();
    increment();
    println!("count는 이제 {count}");

    // move 키워드: 환경의 소유권을 클로저에 넘깁니다.
    // 스레드나 다른 수명으로 클로저를 보낼 때 필요합니다(13장 스레드에서 재등장).
    let data = vec![1, 2, 3];
    let own_data = move || println!("{data:?}");
    own_data();
    // println!("{data:?}"); // ← 주석을 풀면 에러! 소유권이 클로저로 이동함

    // ── 3. 이터레이터: 게으른(lazy) 시퀀스 ───────────────────────
    // iter()는 그냥 만들어만 두고, 소비(consume)할 때까지 아무 일도 하지 않습니다.
    let numbers = vec![1, 2, 3, 4, 5];

    // map: 각 원소를 변환 (게으름 — 아직 실행 안 됨)
    let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
    println!("2배: {doubled:?}");

    // filter: 조건을 통과하는 원소만
    let evens: Vec<i32> = numbers.iter().filter(|x| **x % 2 == 0).copied().collect();
    println!("짝수: {evens:?}");

    // find: 첫 번째로 조건에 맞는 원소 (Option 반환)
    let first_over_2 = numbers.iter().find(|x| **x > 2);
    println!("2보다 큰 첫 수: {first_over_2:?}");

    // ── 4. 소비자(consumer) 메서드들 ─────────────────────────────
    // sum: 합계
    println!("합계: {}", numbers.iter().sum::<i32>());

    // fold: 누적 계산 — 합계, 최댓값, 문자열 결합 등 무엇이든
    let sum = numbers.iter().fold(0, |acc, x| acc + x);
    println!("fold 합계: {sum}");

    // max / min
    println!("최댓값: {:?}", numbers.iter().max());

    // any: 하나라도 조건을 만족하는지
    println!("3의 배수가 있나? {}", numbers.iter().any(|x| x % 3 == 0));

    // ── 5. 체이닝: 파이프라인처럼 엮기 ───────────────────────────
    let result: Vec<i32> = (1..=10)
        .filter(|x| x % 2 == 1) // 홀수만
        .map(|x| x * x) // 제곱
        .take(3) // 3개만
        .collect();
    println!("1~10 홀수 제곱 중 3개: {result:?}"); // [1, 9, 25]

    // 문자열에도 적용할 수 있습니다.
    let words = ["사과", "바나나", "체리", "수박"];
    let long_names: Vec<String> = words
        .iter()
        .filter(|w| w.chars().count() > 2)
        .map(|w| format!("[{w}]"))
        .collect();
    println!("3글자보다 긴 과일: {long_names:?}");

    // ── 6. 이터레이터 직접 만들기 ────────────────────────────────
    // 1, 2, 3, ... 무한 피보나치
    let fib = Fibonacci { a: 0, b: 1 };
    let first10: Vec<u64> = fib.take(10).collect();
    println!("피보나치 10개: {first10:?}");

    // ── 연습 과제 ────────────────────────────────────────────────
    // 1) 클로저 두 개를 받아 합성한 함수를 돌려주는 함수를 만들어 보세요. (고차 함수)
    // 2) (1..=100)에서 3 또는 5의 배수의 합을 fold 없이 sum으로 구해 보세요.
    // 3) 문자열 슬라이스에서 각 단어의 첫 글자만 대문자로 바꿔 이어 붙여 보세요.
    //    (map + collect, chars 사용)
}

/// 무한 피보나치 이터레이터: Iterator 트레이트만 구현하면 됩니다.
struct Fibonacci {
    a: u64,
    b: u64,
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        // u64 오버플로우에 대비해 checked_add 사용
        let next = self.a.checked_add(self.b)?;
        self.a = self.b;
        self.b = next;
        Some(self.a)
    }
}
