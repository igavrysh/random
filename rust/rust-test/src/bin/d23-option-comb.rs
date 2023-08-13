fn main() {
    let a: Option<i32> = Some(1);
    dbg!(a);

    let a_is_some = a.is_some();
    dbg!(a_is_some);

    let a_is_none = a.is_none();
    dbg!(a_is_none);

    let a_mapped = a.map(|num| num + 1);
    dbg!(a_mapped);

    let a_filtered = a.filter(|num| num == &1);
    dbg!(a_filtered);

    let a_or_else = a.or_else(|| Some(5));
    dbg!(a_or_else);

    let unwrapped = a.unwrap_or_else(|| 0);
    dbg!(unwrapped);
}