impl Solution {
    pub fn min_available_duration(slots1: Vec<Vec<i32>>,
                                  slots2: Vec<Vec<i32>>,
                                  duration: i32) -> Vec<i32> {
        let mut time: Vec<(i32, i32)> = Vec::new();
        for slot in slots1.iter() {
            time.push((slot[0], 1));
            time.push((slot[1], -1));
        }
        for slot in slots2.iter() {
            time.push((slot[0], 1));
            time.push((slot[1], -1));
        }
        time.sort_unstable_by(
            |a, b| if a.0 == b.0 { b.1.cmp(&a.1) }
                   else { a.0.cmp(&b.0) });
        let mut count = 0;
        let mut maybe_start = None;
        for pt in time.iter() {
            count += pt.1;
            if count == 2 {
                maybe_start = Some(pt.0);
                continue;
            }
            if let Some(start) = maybe_start {
                if pt.0 - start >= duration {
                    return vec![start, start + duration];
                }
            }
            maybe_start = None;
        }
        Vec::new()
    }
}