struct ATM {
    res: Vec<i32>,
    cash_type: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl ATM {
    fn new() -> Self {
        ATM {
            res: vec![0; 5],
            cash_type: vec![20, 50, 100, 200, 500],
        }
    }

    fn deposit(&mut self, banknotes_count: Vec<i32>) {
        (0..5).for_each(|i| self.res[i] += banknotes_count[i]);
    }

    fn withdraw(&mut self, amount: i32) -> Vec<i32> {
        let mut tmp = amount;
        let mut res = vec![0; 5];
        for i in (0..5).rev() {
            let cnt = (tmp / self.cash_type[i]).min(self.res[i]);
            tmp -= cnt * self.cash_type[i];
            res[i] = cnt;
        }
        if tmp > 0 {
            return vec![-1];
        }
        (0..5).for_each(|i| self.res[i] -= res[i]);
        return res;
    }
}

/**
 * Your ATM object will be instantiated and called as such:
 * let obj = ATM::new();
 * obj.deposit(banknotesCount);
 * let ret_2: Vec<i32> = obj.withdraw(amount);
 */
fn main() {}
