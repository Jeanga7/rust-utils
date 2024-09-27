impl Solution {
    pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
        let mut n = n;
        let mut flowerbed = flowerbed;
        
        for i in 0..flowerbed.len() {
            if flowerbed[i] == 0 &&
               (i == 0 || flowerbed[i - 1] == 0) &&
               (i == flowerbed.len() - 1 || flowerbed[i + 1] == 0) {
                flowerbed[i] = 1;
                n -= 1;
            }
            if n <= 0 {
                return true;
            }
        }
        
        false
    }
}


