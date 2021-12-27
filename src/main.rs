extern crate crypto;

use self::crypto::digest::Digest;
use self::crypto::sha3::Sha3;

use std::thread;

fn main() {
    let mut hasher = Sha3::keccak256();

    let mut line = String::new();
    // println!("Enter the target function:");
    // let _b1 = std::io::stdin().read_line(&mut line).unwrap();

    // line = line.trim().to_string();
    line = "withdraw(uint256)".to_string();

    println!("{}", line);

    // hasher.reset();
    hasher.input_str(&line);

    let hex = hasher.result_str();
    let signature: String = hex.chars().take(8).collect();

    println!("Signature: {}", signature);

    let limit = u32::pow(2, 24);

    let worker_count = 8;
    let step = limit as usize / worker_count;

    let steps = [
        0,
        step,
        step * 2,
        step * 3,
        step * 4,
        step * 5,
        step * 6,
        step * 7,
        limit as usize,
    ];

    let mut handles = Vec::new();

    for i in 1..=worker_count {
        let start = steps[i - 1];
        let end = steps[i];

        let sig = signature.clone();

        let handle = thread::spawn(move || {
            for i in start..end {
                let mut hasher = Sha3::keccak256();
                hasher.input_str("OwnerTransferV");
                hasher.input_str(format!("{:X}", i).as_str());
                hasher.input_str("(uint256)");

                let hex = hasher.result_str();
                let hash: String = hex.chars().take(8).collect();

                if hash == sig {
                    println!("Found: {}", i);
                    break;
                }
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

// asdf: 4c8f1858