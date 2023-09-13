use std::vec;

/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let result = code
        .chars()
        .map(|c|
            if c.is_alphanumeric() || c.is_whitespace() { Result::<char, String>::Ok(c) } 
            else { Result::<char, String>::Err(format!("not supported symbol {}", c)) }
        )
        .filter(|c| {
            match c {
                Ok(ch) => !ch.is_whitespace(),
                Err(_) => true,
            }
        })
        .map(|r| {
            match r {
                Ok(c) => match c.to_string().parse::<u8>() {
                    Ok(d) =>  Result::<u8, String>::Ok(d),
                    Err(e) => Result::<u8, String>::Err(e.to_string())
                },
                Err(s) => Err(s),
            }
        })
        .fold(Result::<Vec<u8>, Vec<String>>::Ok(vec![]), |acc_r, r| {
            match r {
                Ok(d) => {
                    match acc_r {
                        Ok(mut ar) => { ar.push(d); Ok(ar)},
                        Err(e) => { Err(e) },
                    }
                }
                Err(s) => {
                    match acc_r {
                        Ok(_) => { Err(vec![s])},
                        Err(mut e) => { e.push(s); Err(e) },
                    }
                }
            }
        })
        .and_then(|v| {
            match v.len() {
                0..=1 => Err(vec![format!("number of digits, should be > 1")]),
                _ => Ok(v),
            }
        });
    
    let v: Vec<u8>;
    match result {
        Ok(vec) => v = vec,
        Err(_) => return false,
    }

    v.iter()
        .rev()
        .enumerate()
        .map(|(i, d)| if i%2==1 { 2*d } else { *d } )
        .map(|d| if d > 9 { d-9 } else { d } )
        .fold(0 as u32, |acc, d| { acc + d as u32 })
        % 10 == 0
}
