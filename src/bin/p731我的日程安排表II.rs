struct MyCalendarTwo {
    booked: Vec<(i32, i32)>,
    booked_twice: Vec<(i32, i32)>,
}

impl MyCalendarTwo {
    fn new() -> Self {
        MyCalendarTwo {
            booked: Vec::new(),
            booked_twice: Vec::new(),
        }
    }

    fn book(&mut self, start_time: i32, end_time: i32) -> bool {
        for &(l, r) in &self.booked_twice {
            if !(start_time >= r || end_time <= l) {
                return false;
            }
        }

        for &(l, r) in &self.booked {
            if !(start_time >= r || end_time <= l) {
                self.booked_twice.push((start_time.max(l), end_time.min(r)));
            }
        }

        self.booked.push((start_time, end_time));
        true
    }
}
fn main() {}
