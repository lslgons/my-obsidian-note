//! 챕터 3: 소유권 — Rust의 가장 중요한 개념
//!
//! 실행: cargo run --bin ch03_ownership
//!
//! Rust는 GC(쓰레기 수거기) 대신 "소유권 규칙"으로 메모리를 관리합니다.
//! 규칙은 두 개뿐입니다:
//!   1. 모든 값은 정확히 하나의 소유자(owner)를 가진다.
//!   2. 소유자가 스코프를 벗어나면 값은 해제(drop)된다.

fn main() {
    // ── 1. 이동(move): 소유권이 넘어간다 ─────────────────────────
    let s1 = String::from("hello");
    let s2 = s1; // s1의 소유권이 s2로 "이동"했습니다.

    // println!("{s1}"); // ← 주석을 풀면 에러! "value borrowed after move"
    // 스택에 있는 값(i32 등)은 자동으로 복사되므로 문제없습니다. 아래에서 다룹니다.
    println!("s2 = {s2}");

    // 함수에 값을 전달하는 것도 이동입니다.
    takes_ownership(s2);
    // println!("{s2}"); // ← 이것도 에러! s2는 함수로 이동했습니다.

    // ── 2. Copy 타입 vs Clone ────────────────────────────────────
    let a = 5;
    let b = a; // i32 같은 "작은 값"은 Copy 트레이트를 갖고 있어 복사됩니다.
    println!("a = {a}, b = {b}"); // a를 여전히 쓸 수 있음

    let s3 = String::from("clone me");
    let s4 = s3.clone(); // 복사(깊은 복사)하고 싶으면 명시적으로 .clone()
    println!("s3 = {s3}, s4 = {s4}"); // 둘 다 유효

    // ── 3. 참조(&): 소유권 없이 빌린다 ───────────────────────────
    // `&`를 붙여 "참조"를 넘기면 소유권이 이동하지 않습니다. 이걸 "빌림"이라 합니다.
    let s5 = String::from("hello");
    let len = string_length(&s5); // &s5: 빌려만 줌
    println!("'{s5}'의 길이는 {len}"); // s5를 여전히 사용 가능!

    // ── 4. 가변 참조: 빌려서 수정한다 ────────────────────────────
    let mut s6 = String::from("hello");
    append_world(&mut s6);
    println!("s6 = {s6}");

    // ── 5. 빌림 규칙: 컴파일러가 지켜주는 데이터 경쟁 방지 ───────
    let mut s7 = String::from("rules");
    let r1 = &s7; // 불변 참조는 여러 개 OK
    let r2 = &s7;
    println!("{r1} {r2}");

    // 아래의 주석을 풀면 "cannot borrow `s7` as mutable because it is
    // also borrowed as immutable" 에러가 납니다:
    // let r3 = &mut s7;       // 동시에 가변 참조는 불가!
    // println!("{r1} {r3}");

    // NLL(Non-Lexical Lifetimes) 덕분에 r1, r2를 마지막으로 쓴 후에는
    // 가변 참조를 만들 수 있습니다:
    let r3 = &mut s7;
    r3.push('!');
    println!("s7 = {s7}");

    // ── 6. 매달린 참조(dangling reference) 방지 ──────────────────
    // 아래 dangle()의 주석을 풀면 에러: "returns a reference to a dropped value"
    // 참조는 항상 유효한 값만 가리켜야 합니다.

    // ── 7. 슬라이스(Slice) 타입: 소유권 없는 부분 참조 ───────────
    // 컬렉션 전체가 아닌 연속된 일련의 요소를 참조합니다.
    let full_text = String::from("hello world");
    let hello_slice: &str = &full_text[0..5];
    let world_slice: &str = &full_text[6..11];
    println!("슬라이스 분리: '{hello_slice}', '{world_slice}'");

    // 문자열 리터럴 자체가 실은 바이너리에 고정된 문자열 슬라이스(&str)입니다.
    let literal: &str = "hello";
    println!("문자열 리터럴은 &str: {literal}");

    // 배열 슬라이스: &[T]
    let arr = [10, 20, 30, 40, 50];
    let arr_slice: &[i32] = &arr[1..4]; // [20, 30, 40]
    println!("배열 슬라이스: {arr_slice:?}");

    // ── 연습 과제 ────────────────────────────────────────────────
    // 1) takes_ownership에서 값을 돌려받으려면 어떻게 해야 할까요?
    //    (힌트: 값도 같이 반환하거나, 참조를 사용)
    // 2) 가변 참조를 받는 emphasize(&mut String) 함수를 만들어
    //    문자열 끝에 "!?!" 를 붙여 보세요.
    // 3) main의 s1 이동 코드 주석을 풀고, 컴파일러가 알려주는 해결책을 읽어 보세요.
}

fn takes_ownership(s: String) {
    println!("함수가 소유권을 가짐: {s}");
    // 함수가 끝나면 s는 drop됩니다. (메모리 해제)
}

fn string_length(s: &String) -> usize {
    s.len() // s는 빌린 것이라 소유하지 않음 — 함수가 끝나도 해제하지 않음
}

fn append_world(s: &mut String) {
    s.push_str(", world"); // 빌려서 원본을 수정
}

// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s // ← s는 함수가 끝나며 해제되므로, 이 참조는 유효하지 않습니다.
// }
