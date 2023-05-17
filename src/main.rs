extern "C" {

    fn guarded_rshift(x: u64, r: u32) -> u64;

}

fn main() {
    let mut args = std::env::args().skip(1);
    let x = args.next().unwrap().parse().unwrap();
    let r = args.next().unwrap().parse().unwrap();
    println!("Result = {}", unsafe { guarded_rshift(x, r) });
}
