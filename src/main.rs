use compress::Delta;

fn main() {
    //Generate a string that is 10_000 bytes long and create a copy with small changes at random positions
    let src = "a".repeat(100_000).into_bytes();
    let mut dst = src.clone();
    let n = 1000;
    for _ in 0..n {
        let i = rand::random::<usize>() % src.len();
        dst[i] = if dst[i] == 'a' as u8 {
            'b' as u8
        } else {
            'a' as u8
        };
    }
    //Create a delta from the original and the modified string
    let delta = Delta::create(&src, &dst).unwrap();
    println!("delta.len() = {}", delta.len());
    println!(
        "compression ratio = {}",
        delta.len() as f64 / src.len() as f64
    );
    //Apply the delta to the modified string to get the original string back
    //The result should be the same as the original string
    let result = Delta::apply(&src, &delta).unwrap();
    // Print lengths for sanity check
    println!("src.len() = {}", src.len());
    println!("result.len() = {}", result.len());
    assert_eq!(dst, result);
}
