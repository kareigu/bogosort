use rand::thread_rng;
use rand::seq::SliceRandom;
use stopwatch::{Stopwatch};

fn main() {
    println!("Input array length: ");
    let mut buf: String = "".to_string();

    std::io::stdin().read_line(&mut buf).expect("Did not enter a correct string");
    let sortlength = buf.trim().parse::<i32>();

    let mut sorted: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    match sortlength {
        Ok(ok) => if ok > 0 { sorted = (1 .. (ok + 1)).map(i32::from).collect() },
        Err(e) => println!("{}", e),
    }



    let mut sortable: Vec<i32> = sorted.to_vec();
    let mut iterations: i128 = 0;

    let sw = Stopwatch::start_new();

    sortable.shuffle(&mut thread_rng());

    print!("Original: ");
    for i in &sortable {
        print!("{} ", i);
    }
    bogosort(&mut iterations, &sorted, &mut sortable);

    print!("\nResult: ");
    for i in &sortable {
        print!("{} ", i);
    }

    println!("\nIterations: {}", iterations);

    let secondspassed: f64 = sw.elapsed_ms() as f64 / 1000 as f64;
    println!("Seconds: {}", secondspassed);


    std::io::stdin().read_line(&mut buf).expect("Did not enter a correct string");
}

fn bogosort(iterations: &mut i128, sorted: &Vec<i32>, unsorted: &mut Vec<i32>) {
    let mut issorted = false;

    while !issorted {
        if *unsorted == *sorted{
            issorted = true;
        }
        else {
            (*unsorted).shuffle(&mut thread_rng());
        }
        *iterations += 1;
    }
}
