use std::thread;

fn main() {
    let limit = u32::pow(2, 20);

    let workerCount = 8;

    let collected = (0..limit).collect::<Vec<u32>>();

    let chunks = collected.chunks((limit as usize) / workerCount);

    for chunk in chunks {
        println!("{:?}", chunk.len());
    }
}
