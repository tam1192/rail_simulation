pub struct BinarySearchState {
    pub data: Vec<i32>,
    pub target: i32,
    pub low: usize,
    pub high: usize,
    pub mid: usize,
    pub step: usize,
    pub is_found: bool,
    pub is_finished: bool,
}

impl BinarySearchState {
    pub fn new(array_size: usize, target: i32) -> Self {
        let data: Vec<i32> = (1..=array_size).map(|i| (i * 5) as i32).collect();
        let high = data.len().saturating_sub(1);
        Self {
            data,
            target,
            low: 0,
            high,
            mid: 0,
            step: 0,
            is_found: false,
            is_finished: false,
        }
    }

    pub fn reset(&mut self, array_size: usize, target: i32) {
        *self = Self::new(array_size, target);
    }

    pub fn step(&mut self) -> crate::domain::formula::FormulaTrace {
        if self.is_finished || self.data.is_empty() || self.low > self.high {
            self.is_finished = true;
            return vec!["探索終了 (l > h または完了済み)".to_string()];
        }

        let low = self.low;
        let high = self.high;
        self.step += 1;
        self.mid = self.low + (self.high - self.low) / 2;
        let mid_val = self.data[self.mid];
        let mut trace = vec![
            "m = l + ⌊(h − l) / 2⌋".to_string(),
            format!("  = {low} + ⌊({high} − {low}) / 2⌋ = {}", self.mid),
            format!("a[m] = a[{}] = {}", self.mid, mid_val),
            format!("t = {}", self.target),
        ];

        if mid_val == self.target {
            self.is_found = true;
            self.is_finished = true;
            trace.push(format!("{mid_val} = t  ⇒  発見 (index = {})", self.mid));
        } else if mid_val < self.target {
            trace.push(format!("{mid_val} < t"));
            if self.mid >= self.high {
                self.is_finished = true;
                trace.push("m ≥ h  ⇒  未発見".to_string());
            } else {
                self.low = self.mid + 1;
                trace.push(format!("⇒ l = m + 1 = {}", self.low));
            }
        } else {
            trace.push(format!("{mid_val} > t"));
            if self.mid == 0 || self.mid - 1 < self.low {
                self.is_finished = true;
                trace.push("m = 0 または m − 1 < l  ⇒  未発見".to_string());
            } else {
                self.high = self.mid - 1;
                trace.push(format!("⇒ h = m − 1 = {}", self.high));
            }
        }

        trace
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_existing_value() {
        let mut search = BinarySearchState::new(10, 25);
        while !search.is_finished {
            search.step();
        }
        assert!(search.is_found);
        assert_eq!(search.data[search.mid], 25);
    }

    #[test]
    fn reports_missing_value() {
        let mut search = BinarySearchState::new(10, 7);
        while !search.is_finished {
            search.step();
        }
        assert!(!search.is_found);
    }
}
