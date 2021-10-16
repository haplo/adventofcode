use md5;

#[derive(Debug)]
struct Solution {
    nonce: u64,
    attempt: String,
    digest: md5::Digest,
}

fn find_solution(secret: String) -> Solution {
    let mut n: u64 = 1;
    loop {
        let attempt = secret.clone() + &n.to_string();
        let digest = md5::compute(attempt.clone().into_bytes());
        if (digest.0[0] & 0xff == 0) && (digest.0[1] & 0xff == 0) && (digest.0[2] & 0xff == 0) {
            return Solution {
                nonce: n,
                attempt: attempt,
                digest: digest,
            };
        }
        if n % 1000000 == 0 {
            println!("{} hashes attempted", n);
        }
        n += 1;
    }
}

fn main() {
    let secret = "ckczppom".to_string();
    let solution = find_solution(secret);
    println!("Found solution: {:?}", solution);
}
