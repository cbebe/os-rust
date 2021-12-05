use std::{process::exit, time::Duration, time::Instant};

pub fn get_int() -> u64 {
    let mut input = String::new();
    if let Ok(_size) = std::io::stdin().read_line(&mut input) {}
    input.trim().parse().unwrap()
}

fn bubble_sort(vec: &mut Vec<i32>, max_seconds: u64) {
    let start = Instant::now();
    let duration = Duration::from_secs(max_seconds);
    for i in 0..vec.len() - 1 {
        for j in 0..(vec.len() - i - 1) {
            if start.elapsed() > duration {
                exit(0);
            }

            if vec[j] > vec[j + 1] {
                vec.swap(j, j + 1);
            }
        }
    }
}

fn main() {
    let max_seconds = get_int();
    let array_size = get_int();

    let mut vec: Vec<i32> = Vec::with_capacity(array_size as usize);
    for _ in 0..vec.capacity() {
        vec.push(rand::random());
    }

    bubble_sort(&mut vec, max_seconds);
    println!("Sorted array:\n{:?}", vec);
}

#[cfg(test)]
mod test {
    #[test]
    fn test_bubble_sort() {
        let mut vec = vec![0, 3, 2, 5, 10, 8];
        let mut sorted_vec = vec.clone();
        sorted_vec.sort();
        super::bubble_sort(&mut vec, 5);
        assert_eq!(vec, sorted_vec);
    }
}
