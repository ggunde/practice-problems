pub struct Solution {
    dp: Vec<i32>,
}
impl Solution {
    pub fn climb_stairs(&mut self, n: i32) -> i32 {
        if n == 0 {
            return 1;
        }
        if n < 0 {
            return 0;
        }

        if self.dp[n as usize] > 0 {
            return self.dp[n as usize];
        }

        let ways = self.climb_stairs(n - 1) + self.climb_stairs(n - 2);
        self.dp[n as usize] = ways;

        return ways;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut solution = Solution { dp: vec![0; 46] };
        assert_eq!(solution.climb_stairs(2), 2);
        assert_eq!(solution.climb_stairs(3), 3);
        assert_eq!(solution.climb_stairs(4), 5);
        assert_eq!(solution.climb_stairs(5), 8);
        assert_eq!(solution.climb_stairs(6), 13);
        assert_eq!(solution.climb_stairs(7), 21);
        assert_eq!(solution.climb_stairs(8), 34);
        assert_eq!(solution.climb_stairs(9), 55);
        assert_eq!(solution.climb_stairs(45), 1836311903);
    }
}
