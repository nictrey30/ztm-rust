// survey program, not all answers to the questions will have an answer, so we use Option<T>

struct Survey {
    q1: Option<i32>,
    q2: Option<bool>,
    q3: Option<String>,
}

fn main() {
    let response = Survey {
        q1: Some(12),
        q2: Some(true),
        q3: Some("A".to_owned()),
    };
    match response.q1 {
        Some(answ) => println!("q1: {:?}", answ),
        None => println!("q1: no response"),
    }
}
