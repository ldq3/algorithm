use std::collections::{BinaryHeap, HashMap};

// The record which will be stored in heap.
#[derive(Copy, Clone, Eq, PartialEq)]
struct HeapRecord {
    key: usize,
    cost: usize,
}

#[derive(Debug, Clone)]
struct TableRecord {
    cost: usize,
    forward: Option<usize>,
}

// BinaryHeap 默认是最大堆，通过定义一个反向的排序规则来实现最小堆 
impl Ord for HeapRecord {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for HeapRecord {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn dijkstra(adj_list: &HashMap<usize, Vec<(usize, usize)>>, start: usize) -> HashMap<usize, TableRecord> {
    let mut distances: HashMap<usize, TableRecord> = HashMap::new();
    let mut fringe: HashMap<usize, TableRecord> = HashMap::new();

    fringe.insert(start, TableRecord{ cost: 0, forward: None });

    while !fringe.is_empty() {
        println!("fringe: {:?}", fringe);
        let position_ref = fringe.iter().min_by(|a, b| a.1.cost.cmp(&b.1.cost)).unwrap().0;
        let position = *position_ref;

        let table_record = fringe.remove(&position).unwrap();
        distances.insert(position, table_record.clone());

        for &(neighbor_key, neighbor_cost) in adj_list.get(&position).unwrap() {
            if distances.contains_key(&neighbor_key) {
                continue;
            }

            if fringe.contains_key(&neighbor_key) {
                if fringe[&neighbor_key].cost > table_record.cost + neighbor_cost {
                    fringe.insert(neighbor_key, TableRecord{ cost: table_record.cost + neighbor_cost, forward: Some(position) });
                }
            } else {
                fringe.insert(neighbor_key, TableRecord{ cost: table_record.cost + neighbor_cost, forward: Some(position) });
            }
        }
    }
    distances
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dijkstra_1() {
        let mut graph = HashMap::new();
        graph.insert(0, vec![(1, 10), (2, 1)]);
        graph.insert(1, vec![(3, 2)]);
        graph.insert(2, vec![(1, 3), (3, 9), (4, 2)]);
        graph.insert(3, vec![]);
        graph.insert(4, vec![(0, 7), (3, 6)]);

        let distances = dijkstra(&graph, 0);

        for (node, distance) in distances.iter() {
            println!("Distance from 0 to {}: {:?}", node, distance);
        }
    }
}
