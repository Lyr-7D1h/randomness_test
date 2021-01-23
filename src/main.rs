use std::io::Write;

use rand::Rng;

fn print_count(count: &Vec<usize>, rev: usize) {
    std::io::stdout().write(b"\x1Bc").unwrap();
    let max = *(count.iter().max().unwrap()) as f32;
    let percentages: Vec<f32> = count
        .iter()
        .map(|x| {
            if *x == 0 {
                return 0 as f32;
            } else {
                return (*x as f32) / max;
            }
        })
        .collect();

    let mut output = String::new();
    for cp in (0..10).rev() {
        for p in percentages.iter() {
            if p * 10 as f32 >= cp as f32 {
                output += "X";
            } else {
                output += " ";
            }
        }
        output += "\n";
    }
    print!("{}", output);
}

fn main() {
    let range = 100;
    let rev = 100000;

    let mut rng = rand::thread_rng();
    let mut count: Vec<usize> = vec![0; range];

    for r in 0..rev {
        let rand = rng.gen_range(0, range);
        count[rand] += 1;
        print_count(&count, r)
    }
}
