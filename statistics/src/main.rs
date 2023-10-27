fn median(v: &Vec<i32>) -> i32 {
    let mut v = v.clone();
    v.sort();
    let mid = v.len() / 2;
    v[mid]
}

fn mode(v: &Vec<i32>) -> i32 {
    let mut counts = std::collections::HashMap::new();
    for &i in v {
        let count = counts.entry(i).or_insert(0);
        *count += 1;
    }
    let mut mode = 0;
    let mut max_count = 0;
    for (k, v) in counts {
        if v > max_count {
            max_count = v;
            mode = k;
        }
    }
    mode
}

fn average(v: &Vec<i32>) -> f64 {
    let mut sum = 0;
    for &i in v {
        sum += i;
    }
    sum as f64 / v.len() as f64
}

fn main() {
    println!("Start");
    let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 8, 8];
    println!("Median: {}", median(&v));
    println!("Mode: {}", mode(&v));
    println!("Average: {}", average(&v));

    println!("Hello, world!");
}
