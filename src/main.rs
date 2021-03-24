use std::env;

mod p01;
mod p02;
mod p03;
mod p04;
mod p05;
mod p06;
mod p07;
mod p08;
mod p09;
mod p10;
mod p11;
mod p12;
mod p13;
mod p14;
mod p15;
mod p16;
mod p17;
mod p18;
mod p19;
mod p20;
mod p21;
mod p22;
mod p23;
mod p24;
mod p25;

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        return Err("Usage: aoc2019 {problem_number}".into());
    }

    let out = match args[1].parse::<u32>() {
        Ok(1) => p01::run(),
        Ok(2) => p02::run(),
        Ok(3) => p03::run(),
        Ok(4) => p04::run(),
        Ok(5) => p05::run(),
        Ok(6) => p06::run(),
        Ok(7) => p07::run(),
        Ok(8) => p08::run(),
        Ok(9) => p09::run(),
        Ok(10) => p10::run(),
        Ok(11) => p11::run(),
        Ok(12) => p12::run(),
        Ok(13) => p13::run(),
        Ok(14) => p14::run(),
        Ok(15) => p15::run(),
        Ok(16) => p16::run(),
        Ok(17) => p17::run(),
        Ok(18) => p18::run(),
        Ok(19) => p19::run(),
        Ok(20) => p20::run(),
        Ok(21) => p21::run(),
        Ok(22) => p22::run(),
        Ok(23) => p23::run(),
        Ok(24) => p24::run(),
        Ok(25) => p25::run(),
        _ => Err("Invalid problem number".into()),
    }?;
    println!("{}", out);
    Ok(())
}
