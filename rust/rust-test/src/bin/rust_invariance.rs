fn safe_swap<'a>(mut_long: &mut &'a str, mut_short: &mut &'a str) {
    // This is completely safe because both sides are guaranteed to live
    // at least as long as 'a
    std::mem::swap(mut_long, mut_short);
}

fn main() {
    let mut long_lived= "I live forever";
    // but this doesn't
    // let long_lived: &'static str = "I live forever";
    {
        // this works
        let short_lived = String::from("temporary");

        // At this call site, static_string's reference type is implicitly
        // coerced from `&'static str` to `&'a str` to match short_lived.
        safe_swap(&mut long_lived, &mut short_lived.as_str());

        // Inside this block, static_string now points to "temporary".
        // This is safe because "temporary" is still alive here.
        println!("Inside block: {}", long_lived);
    }

    // The borrow checker will catch you here!
    // You cannot read `static_string` here because its lifetime was coerced
    // down to the inner block's scope, and that scope has ended.
    //println!("{}", static_string); // <- Un-commenting this will cause a compile error
}