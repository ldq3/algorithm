use log::{
    debug,
    LevelFilter
};

use log4rs::append::file::FileAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Appender, Config, Root};

pub fn bubble(arr: &mut [i32]) {
    let mut swapped = true;
    while swapped {
        swapped = false;
        for i in 0..arr.len() - 1 {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
                swapped = true;
            }
        }
    }
}

pub fn merge(arr: &mut [i32]) {
    env_logger::init();

    let exponent = 3;

    let mut merged = arr.to_vec();
    debug!("input: {:?}", merged);
    debug!("length of input: {}", merged.len());

    let mut base = 1;
    for count in 0..exponent {
        println!("[round {}]", count);
        let mut temp = Vec::new();

        for j in (0..arr.len()).step_by(base * 2) {
            println!("merging from index {} to {}", j, j + base * 2 - 1);

            let mut index1 = 0;
            let mut index2 = 0;
            for _ in 0..(2 * base) {
                println!("1: {}, 2: {}", index1, index2);
                if index1 == base {
                    temp.push(merged[j+base+index2].clone());
                    index2 += 1;
                    continue;
                }
                if index2 == base {
                    temp.push(merged[j+index1].clone());
                    index1 += 1;
                    continue;
                }
                if merged[j+index1] < merged[j+base+index2] {
                    temp.push(merged[j+index1].clone());
                    index1 += 1;
                } else {
                    temp.push(merged[j+base+index2].clone());
                    index2 += 1;
                }
            }
        } 

        merged = temp;
        base *= 2;
    }

    for (i, val) in merged.into_iter().enumerate() {
        arr[i] = val;
    }
}

pub fn insert(arr: &mut [i32]) {
    for i in 1..arr.len() {
        let mut j = i;
        while j > 0 && arr[j - 1] > arr[j] {
            arr.swap(j, j - 1);
            j -= 1;
        }
    }
}

pub fn select(arr: &mut [i32]) {
    for i in 0..arr.len() - 1 {
        let mut min = i;
        for j in i + 1..arr.len() {
            if arr[j] < arr[min] {
                min = j;
            }
        }
        arr.swap(i, min);
    }
}

pub fn quick(arr: &mut [i32]) {
    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{l} - {m}\n")))
        .build("log/output.log").unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(Root::builder().appender("logfile").build(log::LevelFilter::Debug)).unwrap();

    log4rs::init_config(config).unwrap();

    let mut stack = Vec::new();
    stack.push((0, arr.len() - 1));

    while let Some((start, end)) = stack.pop() {
        let len = end - start + 1;
        if len == 1 {
            continue;
        } else {
            let pivot_index = start + partition(&mut arr[start..(end + 1)]);
            debug!("pivot index: {}", pivot_index);
            debug!("start: {}", start);
            debug!("end: {}", end);
            debug!("arr: {:?}", arr);
            
            if start < pivot_index {
                stack.push((start,  pivot_index - 1));
            }
            if end > pivot_index {
                stack.push((pivot_index + 1, end));
            }
        }
        debug!("stack: {:?}", stack);
    }
}

fn partition(arr: &mut [i32]) -> usize {
    let mut low = 0;
    let mut high = arr.len() - 1;

    let pivot = arr[low];
    
    while low < high {
        while low < high && arr[high] >= pivot {
            high -= 1;
        }
        arr[low] = arr[high];
        while low < high && arr[low] <= pivot {
            low += 1;
        }
        arr[high] = arr[low];
    }
    arr[low] = pivot;

    low
}

#[cfg(test)]
mod tests {
    #[test]
    fn bubble() {
        let mut arr = [5, 4, 3, 2, 1];
        super::bubble(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn merge() {
        let mut arr = [8, 5, 6, 7, 4, 3, 2, 1];
        super::merge(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5, 6, 7, 8]);
    }

    #[test]
    fn insert() {
        let mut arr = [8, 5, 6, 7, 4, 3, 2, 1];
        super::insert(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5, 6, 7, 8]);
    }

    #[test]
    fn select() {
        let mut arr = [8, 5, 6, 7, 4, 3, 2, 1];
        super::select(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5, 6, 7, 8]);
    }

    #[test]
    fn quick() {
        let mut arr = [8, 5, 6, 7, 4, 3, 2, 1];
        super::quick(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5, 6, 7, 8]);
    }

    #[test]
    fn partition() {
        let mut arr = [8, 5, 6, 7, 4, 3, 2, 1];
        let pivot_index = super::partition(&mut arr[0..8]);
        assert_eq!(pivot_index, 7);
    }
}