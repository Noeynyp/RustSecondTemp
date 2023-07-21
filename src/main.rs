fn main() {
    let args: Vec<String> = std::env::args().collect();
    let c_arg = if args.len() < 2 { "" } else { &args[1] };
    let c: f32 = c_arg.parse().unwrap_or(0.0);
    println!("Temperature in degree fahrenheit: {}", (9. / 5.) * c + 32.);
}