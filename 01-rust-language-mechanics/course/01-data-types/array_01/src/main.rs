fn main() {
    //packet data
    let mut packet_data = [100, 200, 300];
    packet_data[1] = 250;
    let number_packet = packet_data.len();
    println!("{number_packet}");

    //buffer
    let mut _buffer: [u8; 64] = [0; 64];

    //cpu cores
    let cpu_cores = [2.4, 3.2, 2.8, 3.5];
    let number_len = cpu_cores.len();
    let last_cores = cpu_cores[3];
    println!("{last_cores}");
    println!("{number_len}");

    //display cpu_cores
    println!("{:?}", cpu_cores);

    //debug
    let cpu_architectures = ["x86_64", "ARM64", "RISC-V", "i386"];
    println!("{cpu_architectures:#?}",);
}
