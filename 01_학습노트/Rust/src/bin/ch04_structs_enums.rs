//! 챕터 4: 구조체와 열거형 — 데이터를 표현하는 방법
//!
//! 실행: cargo run --bin ch04_structs_enums

use std::fmt;

// ── 1. 구조체(struct): 관련 데이터를 묶는 타입 ───────────────────
#[derive(Debug)] // Debug 트레이트를 파생(derive)시켜 {:?}로 출력 가능하게 함
struct User {
    name: String,
    age: u32,
    email: Option<String>, // 값이 있거나 없거나 — null 대신 Option!
}

// 필드 이름 없이 타입만 갖는 튜플 구조체(Tuple Struct)
#[derive(Debug)]
struct Color(i32, i32, i32);

#[derive(Debug)]
struct Point(i32, i32, i32);

// 필드가 전혀 없는 유사 단위 구조체(Unit-like Struct)
// 상태는 없지만 트레이트를 구현할 때 유용합니다.
#[derive(Debug)]
struct AlwaysEqual;

impl User {
    // 연관 함수(정적 메서드): Self = 이 타입. 관용적으로 new라고 부릅니다.
    fn new(name: &str, age: u32) -> Self {
        Self {
            name: name.to_string(), // &str → String 변환
            age,
            email: None, // 초기엔 이메일이 없음
        }
    }

    // 메서드: self를 "빌려서(&)" 읽기만 함
    fn is_adult(&self) -> bool {
        self.age >= 18
    }

    // 가변 메서드: &mut self로 원본을 수정
    fn set_email(&mut self, email: &str) {
        self.email = Some(email.to_string());
    }
}

// ── 2. 열거형(enum): "이 중 반드시 하나"의 값 ────────────────────
enum Shape {
    Circle(f64),              // 각 변형(variant)마다
    Rectangle(f64, f64),      // 서로 다른 데이터를
    Triangle { a: f64, b: f64, c: f64 }, // 다른 형태로 가질 수 있음
}

impl Shape {
    fn area(&self) -> f64 {
        // enum을 다룰 때는 match가 필수! 컴파일러가 모든 경우를 체크합니다.
        match self {
            Shape::Circle(r) => std::f64::consts::PI * r * r,
            Shape::Rectangle(w, h) => w * h,
            Shape::Triangle { a, b, c } => {
                // 헤론의 공식
                let s = (a + b + c) / 2.0;
                (s * (s - a) * (s - b) * (s - c)).sqrt()
            }
        }
    }
}

// ── 3. 트레이트 구현으로 출력 형식 지정 (6장에서 자세히!) ────────
impl fmt::Display for Shape {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Shape::Circle(_) => write!(f, "원"),
            Shape::Rectangle(_, _) => write!(f, "직사각형"),
            Shape::Triangle { .. } => write!(f, "삼각형"),
        }
    }
}

fn main() {
    // ── struct 사용 ──────────────────────────────────────────────
    let mut user = User::new("김러스트", 25);
    println!("{user:?}"); // Debug 출력

    println!("성인인가요? {}", user.is_adult());
    user.set_email("rust@example.com");
    println!("이메일 설정 후: {user:?}");

    // 튜플 구조체 및 단위 구조체 사용
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    let _subject = AlwaysEqual;
    println!("검정색: ({}, {}, {}), 원점: ({}, {}, {})", black.0, black.1, black.2, origin.0, origin.1, origin.2);

    // 구조체 갱신 문법(struct update syntax): 일부만 바꿔 새 구조체 생성
    let user2 = User {
        name: String::from("박코딩"),
        ..user // 나머지 필드는 user에서 가져옴
    };
    // 주의: user의 name(String)이 이동되어 user는 더 이상 완전하지 않을 수 있습니다.
    println!("user2 = {user2:?}");

    // ── enum 사용 ────────────────────────────────────────────────
    let shapes = [
        Shape::Circle(2.0),
        Shape::Rectangle(3.0, 4.0),
        Shape::Triangle { a: 3.0, b: 4.0, c: 5.0 },
    ];

    for shape in &shapes {
        println!("{shape} 넓이: {:.2}", shape.area());
    }

    // ── Option<T>: null이 없는 Rust의 해법 ───────────────────────
    // 값이 있으면 Some(값), 없으면 None. 컴파일러가 None 체크를 강제합니다.
    let email: Option<&String> = user2.email.as_ref();
    match email {
        Some(e) => println!("user2의 이메일: {e}"),
        None => println!("user2는 이메일이 없습니다"),
    }

    // if let: "이 경우에만" 처리할 때 간결합니다.
    if let Some(e) = &user2.email {
        println!("(if let) 이메일: {e}");
    }

    // ── 연습 과제 ────────────────────────────────────────────────
    // 1) Book { title, author, pages } 구조체를 만들고
    //    is_long(&self) -> bool 메서드를 추가해 보세요 (500쪽 이상).
    // 2) TrafficLight enum(빨강/노랑/초록)을 만들고
    //    seconds(&self) -> u32 로 신호 시간을 반환해 보세요.
    // 3) User에 introduce(&self) -> String 을 추가해 "안녕하세요, OOO님"을
    //    반환해 보세요. (email이 있으면 이메일도 포함)
}
