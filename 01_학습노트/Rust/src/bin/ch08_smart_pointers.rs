//! 챕터 8: 스마트 포인터 — Box, Rc, RefCell
//!
//! 실행: cargo run --bin ch08_smart_pointers
//!
//! 보통 참조자(&)는 데이터를 대여만 하지만, 스마트 포인터는
//! 메타데이터와 부가 기능을 갖추고 데이터의 소유권을 가집니다.
//! (Deref 트레이트로 역참조 가능, Drop 트레이트로 자동 정리)

use std::cell::RefCell;
use std::rc::Rc;

// 재귀적 타입 정의: Box<T>를 이용해 컴파일 타임에 크기를 알 수 있게 함
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

// 다중 소유권을 위한 Rc List
#[derive(Debug)]
enum RcList {
    Cons(i32, Rc<RcList>),
    Nil,
}

fn main() {
    // ── 1. Box<T>: 힙(Heap) 메모리에 데이터 저장 ───────────────────
    // 스택 대신 힙에 데이터를 할당하고, 크기가 고정되지 않는 재귀적 타입을 만들 때 필수적입니다.
    let b = Box::new(5);
    println!("힙에 저장된 값: {b}");

    use List::{Cons, Nil};
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("재귀적 Box 연결 리스트: {list:?}");

    // ── 2. Rc<T>: 참조 카운팅(Reference Counting) ────────────────
    // 단일 스레드 환경에서 하나의 데이터에 "여러 소유자"가 필요할 때 사용합니다.
    use RcList::{Cons as RcCons, Nil as RcNil};
    let shared_tail = Rc::new(RcCons(10, Rc::new(RcNil)));
    println!("shared_tail 참조 카운트: {}", Rc::strong_count(&shared_tail)); // 카운트 1

    let list_a = RcCons(3, Rc::clone(&shared_tail));
    println!("list_a 생성 후 카운트: {}", Rc::strong_count(&shared_tail)); // 카운트 2

    {
        let _list_b = RcCons(4, Rc::clone(&shared_tail));
        println!("블록 안 list_b 생성 후 카운트: {}", Rc::strong_count(&shared_tail)); // 카운트 3
    }
    // _list_b가 스코프를 벗어나 드롭됨
    println!("블록 종료 후 카운트: {}", Rc::strong_count(&shared_tail)); // 카운트 2

    // ── 3. RefCell<T>: 내부 가변성 패턴(Interior Mutability) ─────
    // 불변 참조자를 통해서도 내부 데이터를 변경할 수 있게 합니다.
    // 빌림 규칙 검사를 컴파일 타임이 아니라 "런타임"에 수행합니다.
    let val = RefCell::new(42);

    {
        let mut mut_borrow = val.borrow_mut();
        *mut_borrow += 10;
    } // mut_borrow가 여기서 스코프를 벗어남

    println!("RefCell 수정된 값: {}", *val.borrow());

    // ── 4. Rc<RefCell<T>> 조합: 가변 데이터를 다중 소유하기 ─────────
    let shared_number = Rc::new(RefCell::new(100));

    let clone1 = Rc::clone(&shared_number);
    let clone2 = Rc::clone(&shared_number);

    *clone1.borrow_mut() += 50;
    *clone2.borrow_mut() += 20;

    println!("최종 공유 값 (100 + 50 + 20): {}", *shared_number.borrow());
}
