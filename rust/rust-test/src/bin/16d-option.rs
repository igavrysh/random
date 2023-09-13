struct Survey {
    q1: Option<i32>,
    q2: Option<bool>,
    q3: Option<String>,
}

fn main() {
    let response = Survey {
        q1: Some(12),
        // q2: Some(true),
        q2: None,
        q3: Some("A".to_owned()),
    };

    match response.q1 {
        Some(ans) => println!("q1: {:?}", ans),
        None => println!("q1: no response"),
    }

    match response.q2 {
        Some(ans) => println!("q2: {:?}", ans),
        None => println!("q1: no response"),
    }

    match response.q3 {
        Some(ans) => println!("q3: {:?}", ans),
        None => println!("q1: no response"),
    }
}

