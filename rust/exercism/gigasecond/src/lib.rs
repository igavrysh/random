
use time::PrimitiveDateTime as DateTime;
use time::ext::NumericalDuration;

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    let gigasec = 1_000_000_000;
    let d = gigasec.seconds();
    let t = start.checked_add(d).unwrap();
    t
}

