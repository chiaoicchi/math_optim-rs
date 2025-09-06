// This function has time complexity of O(|s|).
// l <=> (
// r <=> )
pub fn vec_is_parenthesis(s: &[u8], l: u8, r: u8) -> bool {
    let mut t = 0u32;
    s.iter().all(|&c| {
        if c == l {
            t += 1;
            true
        } else if c == r {
            t != 0 && {
                t -= 1;
                true
            }
        } else {
            false
        }
    }) && t == 0
}

// This function has time complexity of O(n).
// The length of bits is n.
// S_i = ( <=> (b >> i) & 1 == 0
// S_i = ) <=> (b >> i) & 1 == 1
pub fn num_is_parenthesis(n: usize, b: usize) -> bool {
    let mut cnt = 0i32;
    for i in (0..n).rev() {
        if (b >> i) & 1 == 0 {
            cnt += 1;
        } else {
            cnt -= 1;
        }
        if cnt < 0 {
            return false;
        }
    }
    cnt == 0
}
