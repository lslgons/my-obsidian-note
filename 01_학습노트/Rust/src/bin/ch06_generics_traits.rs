//! 챕터 6: 제네릭과 트레이트 — 재사용 가능한 코드 만들기
//!
//! 실행: cargo run --bin ch06_generics_traits

// ── 1. 제네릭: 타입을 매개변수로 받는 코드 ───────────────────────
// 컴파일러가 사용하는 타입별로 실제 코드를 생성합니다(단형성화, monomorphization).
// PartialOrd bound: "크게 비교할 수 있는 타입"만 허용합니다.
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// ── 2. 트레이트: 타입이 할 수 있는 일을 정의하는 "계약" ──────────
// 다른 언어의 인터페이스와 비슷하지만, 기존 타입에 나중에 붙일 수도 있습니다.
trait Greet {
    fn name(&self) -> String;

    // 기본 구현: 오버라이드하지 않으면 이 구현이 쓰입니다.
    fn greet(&self) -> String {
        format!("안녕하세요, {}님!", self.name())
    }
}

struct Korean;
struct Robot(String);

impl Greet for Korean {
    fn name(&self) -> String {
        "김러스트".to_string()
    }
}

impl Greet for Robot {
    fn name(&self) -> String {
        self.0.clone()
    }

    // 기본 구현을 오버라이드
    fn greet(&self) -> String {
        "삐빅. 안녕하세요, 인간.".to_string()
    }
}

// ── 3. 트레이트 bound: 제네릭에 제약 걸기 ────────────────────────
// (a) 제네릭 + T: Trait — 컴파일 타임 다형성("static dispatch")
fn announce<T: Greet>(who: &T) {
    println!("{}", who.greet());
}

// (b) 트레이트 객체 &dyn Trait — 런타임 다형성("dynamic dispatch")
// 서로 다른 타입을 한 컬렉션에 담을 때 유용합니다.
fn announce_all(who: &[&dyn Greet]) {
    for w in who {
        println!("{}", w.greet());
    }
}

// ── 4. 표준 트레이트 몇 가지 ─────────────────────────────────────
// Debug: {:?} 출력 / Clone: .clone() / PartialEq: == / Default: 기본값
#[derive(Debug, Clone, PartialEq, Default)]
struct Point {
    x: i32,
    y: i32,
}

// From 트레이트: 다른 타입에서의 "변환 생성" — 표준 라이브러리의 관용 문법
impl From<(i32, i32)> for Point {
    fn from((x, y): (i32, i32)) -> Self {
        Point { x, y }
    }
}

// ── 5. 라이프타임: 참조가 유효한 범위를 컴파일러에게 알려주기 ────
// `'a`는 "반환되는 참조는 두 입력 참조 중 더 짧은 쪽만큼 유효하다"는 표시.
// 이 표시가 없으면 컴파일러는 반환 참조가 무엇에 의존하는지 알 수 없습니다.
fn longer<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    // ── 제네릭 ───────────────────────────────────────────────────
    let numbers = [3, 7, 1, 9, 4];
    let words = ["배", "사과", "감"];

    println!("가장 큰 수: {}", largest(&numbers));
    println!("가장 긴 단어: {}", largest(&words));

    // ── 트레이트 ─────────────────────────────────────────────────
    let korean = Korean;
    let robot = Robot("로보티".to_string());

    announce(&korean); // 기본 구현 사용
    announce(&robot); // 오버라이드한 구현 사용

    let crowd: Vec<&dyn Greet> = vec![&korean, &robot];
    announce_all(&crowd);

    // ── 표준 트레이트 ────────────────────────────────────────────
    let p1 = Point::default(); // Default
    let p2 = Point::from((3, 4)); // From
    let p3 = p2.clone(); // Clone

    println!("{p1:?} {p2:?} {p3:?}"); // Debug
    println!("p2 == p3? {}", p2 == p3); // PartialEq

    // From이 있으면 into()도 쓸 수 있습니다 (자동 제공).
    let p4: Point = (5, 6).into();
    println!("튜플 → Point: {p4:?}");

    // ── 라이프타임 ───────────────────────────────────────────────
    let s1 = String::from("긴 문자열입니다");
    {
        let s2 = String::from("짧음");
        let longer_word = longer(&s1, &s2);
        println!("더 긴 문자열: {longer_word}");
    } // s2가 여기서 drop됩니다 — 이후 longer_word는 사용 불가!

    // 아래의 주석을 풀면 에러: "s2 does not live long enough"
    // println!("{}", longer(&s1, "블록 밖에서는 이렇게 쓰면 안전")); // 이건 OK: 'static 문자열 리터럴

    // ── 연습 과제 ────────────────────────────────────────────────
    // 1) 제네릭 함수 second_element<T>(list: &[T]) -> Option<&T>를 만들어 보세요.
    // 2) Animal 트레이트(sound(&self) -> String)를 만들고 Dog, Cat을 구현한 뒤
    //    Vec<&dyn Animal>로 전원의 울음소리를 출력해 보세요.
    // 3) summarize<'a>(a: &'a str, b: &'a str) -> String 을 만들어
    //    "a / b" 형식으로 이어 붙여 보세요.
}
