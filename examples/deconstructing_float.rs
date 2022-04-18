use rust_in_action::visualizing_float;


fn main()  {
    let n: f32 = 42.42;

    let (sign, exp, frac) = visualizing_float::to_parts(n);
    let (sign_, exp_, mantissa) = visualizing_float::decode(sign, exp, frac);
    let n_ = visualizing_float::from_parts(sign_, exp_, mantissa);

    println!("{} -> {}", n, n_);
    println!("field    | as bits | as real number");
    println!("sign     | {sign:01b} | {sign_:?}");
    println!("exponent | {exp:08b} | {exp_}");
    println!("mantissa | {frac:023b} | {mantissa}");
}
