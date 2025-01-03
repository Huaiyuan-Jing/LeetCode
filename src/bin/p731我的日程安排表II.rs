struct MyCalendarTwo {
    bit: Vec<i32>,
    a: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCalendarTwo {
    fn new() -> Self {
        MyCalendarTwo {
            bit: vec![0; 1e9 as usize + 10],
            a: vec![0; 1e9 as usize + 10],
        }
    }

    fn update(&mut self, index: i32, val: i32) {
        let mut index = index;
        self.a[index as usize] += val;
        while (index as usize) < self.bit.len() {
            self.bit[index as usize] += val;
            index += index & (-index);
        }
    }

    fn query(&self, index: i32) -> i32 {
        let mut ans = 0;
        let mut index = index;
        while index > 0 {
            ans += self.bit[index as usize];
            index -= index & (-index);
        }
        ans
    }

    fn book(&mut self, start_time: i32, end_time: i32) -> bool {
        let start_time = start_time + 1;
        let end_time = end_time + 1;
        if self.query(end_time - 1) - self.query(start_time - 1) >= 2 || self.query(start_time) >= 2
        {
            return false;
        }
        let mut cnt = self.query(start_time - 1);
        for i in start_time..end_time {
            cnt += self.a[i as usize];
            if cnt >= 2 {
                return false;
            }
        }
        self.update(start_time, 1);
        self.update(end_time, -1);
        true
    }
}

/**
 * Your MyCalendarTwo object will be instantiated and called as such:
 * let obj = MyCalendarTwo::new();
 * let ret_1: bool = obj.book(startTime, endTime);
 */
fn main() {
    let mut obj = MyCalendarTwo::new();
    let ret_1 = obj.book(10, 20);
    let ret_2 = obj.book(50, 60);
    let ret_3 = obj.book(10, 40);
    let ret_4 = obj.book(5, 15);
    let ret_5 = obj.book(5, 10);
    let ret_6 = obj.book(25, 55);

    println!(
        "{} {} {} {} {} {}",
        ret_1, ret_2, ret_3, ret_4, ret_5, ret_6
    );
}
