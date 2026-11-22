//! 챕터 2: 제어 흐름 — if, match, 루프
//!
//! 실행: cargo run --bin ch02_control_flow

fn grade(score: u32) -> char {
    // ── if는 "식(expression)"입니다 ──────────────────────────────
    // 문(statement)이 아니라 식이기 때문에 변수에 대입할 수 있습니다.
    // 이 때문에 Rust에는 삼항연산자(?:)가 없습니다 — if 식으로 충분하니까요.
    if score >= 90 {
        'A'
    } else if score >= 80 {
        'B'
    } else {
        'C'
    }
}

fn describe_number(n: i32) -> &'static str {
    // ── match: 강력한 패턴 매칭 ──────────────────────────────────
    // 모든 경우를 반드시 다뤄야 합니다(철저함, exhaustiveness).
    // `_`는 "나머지 전부" 와일드카드입니다.
    match n {
        0 => "영",
        1 | 2 | 3 => "작은 수", // 여러 값 OR 매칭
        4..=9 => "한 자리 수",  // 범위 매칭
        n if n < 0 => "음수",   // 가드(guard) 조건
        _ => "그 외",
    }
}

fn main() {
    // ── 1. if 식 ─────────────────────────────────────────────────
    let score = 85;
    println!("점수 {score} → 등급 {}", grade(score));

    // if를 식으로 사용해 변수에 대입
    let x = 10;
    let parity = if x % 2 == 0 { "짝수" } else { "홀수" };
    println!("{x}은(는) {parity}");
    // 주의: if 블록의 각 분기 반환 타입은 서로 같아야 합니다.

    // ── 2. match ─────────────────────────────────────────────────
    for n in [-5, 1, 7, 100] {
        println!("{n}: {}", describe_number(n));
    }

    // ── 3. 루프의 세 가지 ────────────────────────────────────────
    // (1) loop: 무한 루프. `break 값` 으로 반환값을 줄 수 있습니다.
    let mut count = 0;
    let result = loop {
        count += 2;
        if count >= 10 {
            break count * 2; // loop의 반환값
        }
    };
    println!("loop 결과: {result}");

    // (2) while: 조건 반복
    let mut n = 3;
    while n > 0 {
        print!("{n} ");
        n -= 1;
    }
    println!("발사!");

    // (3) for: 범위 순회 — 실무에서 가장 많이 씁니다
    for i in 0..5 {
        // 0,1,2,3,4 (5는 제외)
        print!("{i} ");
    }
    println!();
    for i in (1..=3).rev() {
        print!("{i} ");
    }
    println!("시작!");

    // ── 4. 루프 레이블: 중첩 루프 한 번에 탈출하기 ───────────────
    'outer: for i in 1..=3 {
        for j in 1..=3 {
            if i * j == 6 {
                println!("2 x 3 = 6 발견! 전체 루프 종료");
                break 'outer; // 레이블이 붙은 바깥 루프를 탈출
            }
        }
    }

    // ── 5. 간결한 제어 흐름: if let 및 while let ────────────────
    // match로 한 가지 경우만 처리하고 나머지는 무시하고 싶을 때 보일러플레이트를 줄여줍니다.
    let config_max: Option<u8> = Some(3u8);
    if let Some(max) = config_max {
        println!("최댓값이 {max}로 설정되었습니다.");
    } else {
        println!("최댓값 설정이 없습니다.");
    }

    // while let: 패턴 매칭이 성공하는 동안 계속 반복
    let mut stack = vec![1, 2, 3];
    print!("스택 pop 순서: ");
    while let Some(top) = stack.pop() {
        print!("{top} ");
    }
    println!();

    // ── 연습 과제 ────────────────────────────────────────────────
    // 1) 1부터 100까지의 3의 배수 합을 for 루프로 구해 보세요.
    // 2) 피즈버즈(FizzBuzz): 1~15까지, 3의 배수면 "Fizz", 5의 배수면 "Buzz",
    //    둘 다면 "FizzBuzz", 아니면 숫자를 출력하세요. (match로도 풀어 보세요)
    // 3) `loop`로 1~20 사이 소수의 개수를 구해 보세요.
}
