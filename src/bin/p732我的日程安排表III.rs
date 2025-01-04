mod SegmentTree {
    use std::collections::HashMap;

    pub struct Node {
        max: i32,
        left_bound: i32,
        right_bound: i32,
        lazy: i32,
    }
    pub struct Tree {
        pub nodes: HashMap<i32, Node>,
    }
    impl Tree {
        pub fn new(st: i32, ed: i32) -> Self {
            let mut tree = HashMap::new();
            tree.insert(
                1,
                Node {
                    max: 0,
                    left_bound: st,
                    right_bound: ed,
                    lazy: 0,
                },
            );
            Tree { nodes: tree }
        }
        pub fn update(&mut self, idx: i32, st: i32, ed: i32, val: i32) {
            let node = self.nodes.get_mut(&idx).unwrap();
            let left_bound = node.left_bound;
            let right_bound = node.right_bound;
            let lazy = node.lazy;
            if st > right_bound || ed < left_bound {
                return;
            }
            if st <= left_bound && ed >= right_bound {
                node.lazy += val;
                node.max += val;
                return;
            }
            let mid = left_bound + (right_bound - left_bound) / 2;
            if st <= mid {
                self.nodes.entry(idx * 2).or_insert_with(|| Node {
                    max: 0,
                    left_bound,
                    right_bound: mid,
                    lazy: 0,
                });
                self.update(idx * 2, st, ed, val);
            }
            if ed > mid {
                self.nodes.entry((idx * 2) + 1).or_insert_with(|| Node {
                    max: 0,
                    left_bound: mid + 1,
                    right_bound,
                    lazy: 0,
                });
                self.update((idx * 2) + 1, st, ed, val);
            }
            let left_child_val = match self.nodes.get(&(idx * 2)) {
                Some(node) => node.max,
                None => -0x3f3f3f3f,
            };
            let right_child_val = match self.nodes.get(&((idx * 2) + 1)) {
                Some(node) => node.max,
                None => -0x3f3f3f3f,
            };
            self.nodes.get_mut(&idx).unwrap().max = left_child_val.max(right_child_val) + lazy;
        }
        pub fn get_max(&self) -> i32 {
            self.nodes.get(&1).unwrap().max
        }
    }
}
struct MyCalendarThree {
    schedules: SegmentTree::Tree,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCalendarThree {
    fn new() -> Self {
        MyCalendarThree {
            schedules: SegmentTree::Tree::new(0, 1e9 as i32),
        }
    }

    fn book(&mut self, start_time: i32, end_time: i32) -> i32 {
        self.schedules.update(1, start_time, end_time - 1, 1);
        self.schedules.get_max()
    }
}

/**
 * Your MyCalendarThree object will be instantiated and called as such:
 * let obj = MyCalendarThree::new();
 * let ret_1: i32 = obj.book(startTime, endTime);
 */
fn main() {
    let mut obj = MyCalendarThree::new();
    println!("{}", obj.book(10, 20));
    println!("{}", obj.book(50, 60));
    println!("{}", obj.book(10, 40));
    println!("{}", obj.book(5, 15));
    println!("{}", obj.book(5, 10));
    println!("{}", obj.book(25, 55));
}
