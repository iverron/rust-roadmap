#![allow(unused_variables)]

fn main() {
    //1
    let raw_input = "    Hello\tmy name is VeRRon\n \"Always learning\"";
    let clean_input = raw_input.trim();
    println!("input: {clean_input}");

    let system_path = r"C:\Users\Verron\Documents\Projects";
    println!("{system_path}");

    let status_icon = '🦀';
    let is_alpha = status_icon.is_alphabetic();
    println!("icon is alphabetic {status_icon}? {is_alpha}");

    let character = 'A';
    let is_upper = character.is_uppercase();
    let is_lower = character.is_lowercase();
    println!("character:{character}\nis uppercase? {is_upper}\nis lowercase? {is_lower}");

    //2
    let base_metric: i32 = -5_2;
    let result = base_metric.abs().pow(3);
    println!("result: {result}");

    let sys_index: isize = 64;
    let counter: usize = 32;

    let exact_factor: f64 = 3.1415926535;
    println!(
        "pi:{}\npi ceil:{}\npi floor:{}\n pi round:{}",
        exact_factor,
        exact_factor.ceil(),
        exact_factor.floor(),
        exact_factor.round()
    );

    let exact_cast = exact_factor as f32 as i16;
    println!("{:.2}\n{:.4}", exact_factor, exact_factor);

    //3
    let mut audit_scores: [i8; 4] = [8, 16, 32, 64];
    let array_len = audit_scores.len();
    audit_scores[2] = 20;
    let master_record = (exact_factor, exact_cast, audit_scores, sys_index);
    let first = master_record.0;
    let second = master_record.1;
    let third = master_record.2;
    let fourth = master_record.3;
    let (a, b, c, d) = master_record;

    //4

    let has_clearance = true;
    let is_verified = true;
    let audit_approved = is_verified && has_clearance || !is_verified;
    let num: i32 = -5;
    let positive = num.is_positive();
    let negative = num.is_negative();
    println!("Number is positive?:{positive}\nNumber is negative?{negative}");

    //ranges
    let num_range: std::ops::Range<i8> = 1..50;
    let char_range: std::ops::RangeInclusive<char> = 'a'..='z';
    println!("Range:{:#?}", char_range);

    //for
}
