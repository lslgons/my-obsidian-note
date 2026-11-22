//! 챕터 9: 미니 프로젝트 — 할 일 CLI 앱
//!
//! 지금까지 배운 것을 모두 씁니다:
//! 소유권(3장), 구조체와 enum(4장), 에러 처리(5장),
//! 클로저/이터레이터(7장), 모듈(8장), 파일 입출력.
//!
//! 실행 예시:
//!   cargo run --bin todo -- add "Rust 공부하기"
//!   cargo run --bin todo -- list
//!   cargo run --bin todo -- done 1
//!   cargo run --bin todo -- rm 1

use std::env;
use std::fs;
use std::path::PathBuf;

use rust_tutorial::todo::Task;

const SAVE_FILE: &str = "todo.txt";

fn main() {
    // ── 명령줄 인수 파싱 ─────────────────────────────────────────
    // env::args()의 첫 번째 값은 프로그램 이름이므로 건너뜁니다.
    let args: Vec<String> = env::args().skip(1).collect();

    let command = match args.first() {
        Some(cmd) => cmd.as_str(),
        None => {
            eprintln!("사용법: todo <add|list|done|rm> [인수]");
            eprintln!("  todo add \"할 일\"  — 추가");
            eprintln!("  todo list          — 목록 보기");
            eprintln!("  todo done <번호>   — 완료 처리");
            eprintln!("  todo rm <번호>     — 삭제");
            std::process::exit(1);
        }
    };

    // ── 저장 파일에서 기존 할 일 불러오기 ─────────────────────────
    let mut tasks = load_tasks();

    match command {
        "add" => {
            let title = args.get(1).expect("추가할 내용을 입력하세요");
            tasks.push(Task::new(title));
            save_tasks(&tasks);
            println!("추가했습니다: {title}");
        }

        "list" => {
            if tasks.is_empty() {
                println!("할 일이 없습니다. todo add 로 추가해 보세요!");
                return;
            }
            for (i, task) in tasks.iter().enumerate() {
                println!("{}. {task}", i + 1); // Display 트레이트 활용
            }
            let done_count = tasks.iter().filter(|t| t.done).count();
            println!("— {}개 중 {}개 완료 —", tasks.len(), done_count);
        }

        "done" => {
            let index = parse_index(args.get(1), &tasks);
            tasks[index].finish();
            save_tasks(&tasks);
            println!("완료했습니다: {}", tasks[index].title);
        }

        "rm" => {
            let index = parse_index(args.get(1), &tasks);
            let removed = tasks.remove(index);
            save_tasks(&tasks);
            println!("삭제했습니다: {}", removed.title);
        }

        other => {
            eprintln!("알 수 없는 명령: {other}");
            std::process::exit(1);
        }
    }
}

/// "todo done 2" 처럼 받은 1-based 번호를 유효한 0-based 인덱스로 바꿉니다.
/// 잘못된 입력은 사용자에게 친절한 에러와 함께 종료합니다.
fn parse_index(arg: Option<&String>, tasks: &[Task]) -> usize {
    let n: usize = arg
        .and_then(|s| s.trim().parse().ok())
        .unwrap_or_else(|| {
            eprintln!("번호를 입력하세요 (현재 할 일: {}개)", tasks.len());
            std::process::exit(1);
        });

    if n == 0 || n > tasks.len() {
        eprintln!("번호는 1부터 {} 사이여야 합니다", tasks.len());
        std::process::exit(1);
    }
    n - 1
}

/// 파일 형식: 한 줄에 한 항목, "done,제목" (done은 0 또는 1)
fn load_tasks() -> Vec<Task> {
    let path = save_path();

    // 파일이 없으면 첫 실행 — 빈 목록으로 시작합니다.
    let content = match fs::read_to_string(&path) {
        Ok(text) => text,
        Err(_) => return Vec::new(),
    };

    content
        .lines() // 줄 단위 순회 (7장 이터레이터!)
        .filter(|line| !line.is_empty())
        .map(|line| {
            // "1,Rust 공부하기" → done=true, title="Rust 공부하기"
            let (done_str, title) = line
                .split_once(',') // 타입이 다른 두 조각으로 분해 (1장 튜플!)
                .unwrap_or(("0", line));
            let mut task = Task::new(title);
            task.done = done_str == "1";
            task
        })
        .collect()
}

/// 파일 형식 그대로 저장합니다.
fn save_tasks(tasks: &[Task]) {
    let content: String = tasks
        .iter()
        .map(|t| format!("{},{}", t.done as u8, t.title))
        .collect::<Vec<_>>()
        .join("\n");

    fs::write(save_path(), content).expect("todo.txt 저장 실패");
}

/// 저장 파일 위치: 실행한 디렉터리의 todo.txt
fn save_path() -> PathBuf {
    PathBuf::from(SAVE_FILE)
}
