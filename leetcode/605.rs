impl Solution {
    pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
        let mut i = -2;
        let mut j = 0;
        let mut count = 0;
        for &num in flowerbed.iter() {
            if num == 0 {
                if j - i > 1 {
                    count += 1;
                    i = j;
                }
            } else {
                if j - i <= 1 {
                    count -= 1;
                }
                i = j;
            }
            j += 1;
        }
        count >= n
    }
}