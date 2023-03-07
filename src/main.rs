fn sieve_of_eratosthenes(n:usize) -> Vec<usize>{
    let mut v:Vec<bool> = vec![true; n];
    v[0] = false;
    v[1] = false;
    for i in 2..((n as f64).sqrt().floor()) as usize {
        if v[i] {
            for j in ((i*i)..n).step_by(i) {
                v[j] = false;
            }
        }
    }
    let mut answ:Vec<usize> = vec![];
    for (i, b) in v.iter().enumerate() {
        if *b {
            answ.push(i);
        }
    }
    answ
}
fn main() {
    println!("{:?}", sieve_of_eratosthenes(std::usize::MAX/(8196*8196*64)).last().unwrap());
}
