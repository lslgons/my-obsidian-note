//! 챕터 5: 컬렉션과 에러 처리 — Vec, HashMap, String, ?
//!
//! 실행: cargo run --bin ch05_collections_errors

use std::collections::HashMap;
use std::io;

fn main() {
    // ── 1. Vec<T>: 가변 길이 배열 — 가장 많이 쓰는 컬렉션 ────────
    let mut fruits = vec!["사과", "바나나"];
    fruits.push("오렌지");
    println!("과일: {fruits:?}");

    // 인덱스 접근은 두 가지 방법이 있습니다.
    let second = fruits[1]; // 잘못된 인덱스면 panic!
    println!("두 번째: {second}");

    // get은 Option을 돌려줍니다 — 패닉 대신 안전하게 처리.
    if let Some(f) = fruits.get(10) {
        println!("10번째: {f}");
    } else {
        println!("10번째 과일은 없습니다 (패닉 대신 None)");
    }

    // 순회: 소유권 이동 버전과 참조 버전
    for f in &fruits {
        print!("{f} ");
    }
    println!();

    // ── 2. HashMap<K, V>: 키-값 저장소 ───────────────────────────
    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert(String::from("러스트"), 95);
    scores.insert(String::from("코딩"), 80);
    scores.entry(String::from("러스트")).or_insert(0); // 있으면 무시됨

    for (name, score) in &scores {
        println!("{name}: {score}점");
    }

    // ── 3. String: 소유하는 UTF-8 문자열 ─────────────────────────
    let mut s = String::from("안녕");
    s.push_str("하세요");
    s.push('!');
    println!("{s}");

    // String과 &str의 차이: String은 소유, &str은 참조(빌림).
    let borrowed: &str = &s;
    println!("빌린 문자열의 길이(바이트): {}", borrowed.len());
    // 주의: len()은 바이트 수. 한글은 한 글자에 3바이트!

    // ── 4. 에러 처리: panic! vs Result ───────────────────────────
    // panic!: 복구 불가능한 버그 (프로그램 종료)
    // Result<T, E>: 복구 가능한 에러 (호출자가 처리)

    // 파일을 열 때 처럼 실패할 수 있는 작업의 시늉을 만들어 봅니다.
    match parse_number("42") {
        Ok(n) => println!("파싱 성공: {n}"),
        Err(e) => println!("파싱 실패: {e}"),
    }

    match parse_number("안녕") {
        Ok(n) => println!("파싱 성공: {n}"),
        Err(e) => println!("파싱 실패: {e}"),
    }

    // ? 연산자: 에러를 만나면 즉시 호출자에게 반환 — 아래 함수 참고.

    // ? 연산자를 main에서도 쓸 수 있습니다 (main이 Result를 반환하도록).
    let result = read_and_sum();
    println!("read_and_sum 결과: {result:?}");

    // ── 연습 과제 ────────────────────────────────────────────────
    // 1) 단어 목록에서 각 단어의 등장 횟수를 HashMap으로 세어 보세요.
    // 2) Vec<i32>의 최댓값을 Option<i32>로 반환하는 함수를 만들어 보세요
    //    (빈 벡터라면 None).
    // 3) parse_number를 16진수도 지원하도록 확장해 보세요 ("0x2A" → 42).
}

// Result<T, E>: 성공이면 Ok(T), 실패면 Err(E)
fn parse_number(s: &str) -> Result<i32, String> {
    s.parse::<i32>()
        .map_err(|_| format!("'{s}'는 정수가 아닙니다"))
}

// `?` 연산자: 에러가 나면 그 즉시 반환, 성공이면 값을 꺼냄.
// parse_number가 Result<i32, String>을 돌려주므로
// 이 함수도 같은 에러 타입으로 Result를 반환하면 ?를 그대로 쓸 수 있습니다.
fn read_two_numbers() -> Result<(i32, i32), String> {
    let a = parse_number("10")?; // 실패 시 read_two_numbers가 Err를 반환
    let b = parse_number("abc")?; // 여기서 Err("'abc'는 정수가 아닙니다") 반환!
    Ok((a, b))
}

// io::Result<i32>: 표준 라이브러리의 Result도 같은 원리입니다.
fn read_and_sum() -> io::Result<i32> {
    // 시늉: 실제 입력 읽기 대신 고정값. io::Error를 만들어 에러 흐름을 보여줍니다.
    let _fake: io::Result<()> = Err(io::Error::new(io::ErrorKind::NotFound, "파일 없음"));
    let _fake2: io::Result<()> = Err(io::Error::new(io::ErrorKind::NotFound, "파일 없음"));
    Ok(42)
}
