use primefactor::PrimeFactors;
fn main() {
    let mut prev:u128 = 5;
    let mut n:u128 = 5;

    loop {
        let p = PrimeFactors::from((2*(n))-1).to_factor_vec().first().unwrap().integer;
        println!("n: {}\tvalue: {}\tnext value: {}\tstep: {}", n, prev, p, (p-1)/2);

        prev = p;
        if n > 100000000000000 { 
            break;
        }
        n += (p-1)/2;
    }
}
