fn main() {
    let cpu_info: (&str, u8, f64) = ("i5 13400", 10, 4.60);
    // let frequency = cpu_info.2;
    // println!("CPU Frequency:{frequency}");
    // println!("{cpu_info:#?}");

    let (name, cores, frequency) = cpu_info;
    println!("Model:{name}\ncores:{cores}\nFrequency:{frequency}",);
}
