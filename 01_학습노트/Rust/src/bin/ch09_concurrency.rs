//! 챕터 9: 동시성(Concurrency) — 스레드, 채널, 상태 공유
//!
//! 실행: cargo run --bin ch09_concurrency
//!
//! Rust는 "무서움 없는 동시성(Fearless Concurrency)"을 지향합니다.
//! 소유권과 타입 시스템 덕분에 컴파일 타임에 데이터 레이스(Data Race)를 잡아냅니다.

use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    // ── 1. 스레드 생성 (thread::spawn) ───────────────────────────
    let handle = thread::spawn(|| {
        for i in 1..=3 {
            println!("스레드: {i}");
            thread::sleep(Duration::from_millis(10));
        }
    });

    for i in 1..=2 {
        println!("메인: {i}");
        thread::sleep(Duration::from_millis(10));
    }

    // handle.join()으로 스레드가 끝날 때까지 대기
    handle.join().unwrap();

    // ── 2. move 클로저로 스레드에 소유권 전달 ─────────────────────
    let v = vec![1, 2, 3];
    let handle_move = thread::spawn(move || {
        println!("스레드로 이동된 벡터: {v:?}");
    });
    handle_move.join().unwrap();

    // ── 3. 채널(Channel)을 통한 메시지 전달 (mpsc) ─────────────────
    // mpsc: Multiple Producer, Single Consumer (복수 생산자, 단일 소비자)
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone(); // 송신자 복제 가능

    thread::spawn(move || {
        let msg = String::from("tx1에서 보낸 메시지");
        tx1.send(msg).unwrap(); // send는 소유권을 이동시킴
    });

    thread::spawn(move || {
        let msg = String::from("tx에서 보낸 메시지");
        tx.send(msg).unwrap();
    });

    // 수신 (메시지가 도착할 때마다 꺼냄)
    for received in rx {
        println!("수신완료: {received}");
    }

    // ── 4. Arc<Mutex<T>>: 여러 스레드 간 상태 공유 ────────────────
    // Mutex: 상호 배제(Mutual Exclusion)를 통해 한 번에 한 스레드만 데이터 접근
    // Arc: Atomic Reference Counting (스레드 안전한 Rc)
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter_clone.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for h in handles {
        h.join().unwrap();
    }

    println!("10개 스레드 카운팅 결과: {}", *counter.lock().unwrap());
}
