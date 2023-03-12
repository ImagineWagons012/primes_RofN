use primefactor::PrimeFactors;
fn main() {
    let mut prev:u128 = 5;
    let mut n:u128 = 5;

    loop {
        let p = PrimeFactors::from(((2*(n))-1) as u128).to_factor_vec().first().unwrap().integer;
        
        //decimal representation
        println!("n: {}\tvalue: {}\tnext value: {}\tstep: {}", 
        n, 
        prev,
        p, 
        (p-1)/2);

        // hex representation
        /* 
        println!("n: 0x{}\tvalue: 0x{}\tnext value: 0x{}\tstep: 0x{}", 
        hex::encode_upper(n.to_be_bytes()).replace('0', ""), 
        hex::encode_upper(prev.to_be_bytes()).replace('0', ""), 
        hex::encode_upper(p.to_be_bytes()).replace('0', ""),
        hex::encode_upper(((p-1)/2).to_be_bytes()).replace('0', "")); 
        */

        prev = p;
        if n > std::u64::MAX as u128 { 
            break;
        }
        n += (p-1)/2;
    }
}
