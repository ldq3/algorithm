use std::mem::swap;

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

/**
# Analysis
插入排序算法共由 n 步迭代组成，故其运行时间应取决于各步迭代中所执行的查找、删除及插入操作的效率

列表
插入操作和删除操作均只需 O(1) 时间

查找操作所需时间可在 O(1) 至 O(n) 之间浮动，当输入序列已经有序时，该算法中的每次查找操作均仅需 O(1)  时间，总体运行时间为 O(n)，但反过来, 若输出序列完全逆序, 则各次查找操作所需时间将线性递增, 累计共需  $O\left (n^{2}\right)$  时间，在等概率条件下，平均仍需要 $O\left (n^{2}\right.  )$ 时间

*/
pub fn insert(vector: &mut Vec<i32>) {
    for sorted_len in 1..vector.len() {
        let sorted_vec = &vector[0..sorted_len];

        let hot = sorted_len;
        match sorted_vec.binary_search(& vector[hot]) {
            Ok(offset) | Err(offset) => {
                for i in (offset..hot).rev() {
                    vector.swap(i, i+1);
                }
            }
        }
    }
}

/**
# Analysis
选择排序亦由 n 步迭代组成，故其运行时间取决于各步迭代中查找及插入操作的效率

insertB () 和 remove () 均只需  O (1)  时间

SelectMax () 每次必须遍历整个无序前叒, 耗时应线性正比于前缀长度; 全程累计耗时 $O\left (n^{2}\right)$

实际上进一步地仔细观察之后不难发现, 无论输入序列中各元素的大小次序如何, 以上 n 次 selectMax () 调用的累计耗时总是  $\Theta\left (n^{2}\right)$，其最好和最坏情况下的渐进效率相同

选择排序属于 CBA 式算法, 故相对于 $\Omega (n \log n)$ 下界，$\Theta\left (n^{2}\right)$ 的效率应有很大的改进空间

借助堆, 可以令单次 selectMax ()操作的复杂度降至  O (\log n) , 从而使选择排序的整体效率提高至 $O (n \log n)$

在每一步迭代中，交换 M 和 X 只需常数时间，对 x 的下滤调整不超过 $O (\log n)$ 时间

因此, 全部 n 步迭代累计耗时不超过 $O (n \log n)$

即便使用蛮力算法而不是 Floyd 算法来完成  H  的初始化，整个算法的运行时间也不超过 $O (n \log n)$

纵览算法的整个过程, 除了用于支持词条交换的一个辅助单元, 几乎不需要更多的辅助空间, 故的确属于就地算法

得益于向量结构的简洁性，几乎所有以上操作都可便捷地实现，因此该算法不仅可简明地编码，其实际运行效率也因此往往要高于其它 $O (n \log n)$ 的算法
*/
pub fn select(vector: &mut Vec<i32>) {
    // initialize heap
    let heap = ;

    for heap_size in (1..(vector.len() + 1)).rev() {
        // delete        
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
        let mut vector = vec![8, 5, 6, 7, 4, 3, 2, 1];
        super::insert(&mut vector);
        assert_eq!(vector, [1, 2, 3, 4, 5, 6, 7, 8]);
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