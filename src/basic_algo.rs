//return -1 if not match
pub fn no_kmp(source: &str, target: &str) -> (i32, i32) {
    let s_bytes = source.as_bytes();
    let t_bytes = target.as_bytes();
    println!("source len:{}, target len:{}", s_bytes.len(), t_bytes.len());
    let mut i:usize = 0;
    let mut j:usize = 0;
    let mut match_times: i32 = 0;
    while i < s_bytes.len() && j < t_bytes.len() {
        if s_bytes[i] == t_bytes[j] {
            i += 1; j += 1;
        } else {
            i = i - j + 1; j = 0;
        }
        match_times += 1;
    }
    if j == t_bytes.len() {
        return ((i - j) as i32, match_times);
    }
    return (-1, match_times);
}
pub fn kmp(source: &str, target: &str) -> (i32, i32) {
    let s_bytes = source.as_bytes();
    let t_bytes = target.as_bytes();
    println!("source len:{}, target len:{}", s_bytes.len(), t_bytes.len());
    let mut i:usize = 0;
    let mut j:i32 = 0;
    let mut match_times: i32 = 0;
    while i < s_bytes.len() && j < t_bytes.len() as i32 {
        if s_bytes[i] == t_bytes[j as usize] {
            i += 1; j += 1;
        } else {
            j = kmp_nextj(t_bytes, j);
            if j == -1 {
                i += 1; j = 0;
            }
        }
        match_times += 1;
    }
    if j == t_bytes.len() as i32 {
        return ((i as i32 - j) as i32, match_times);
    }
    return (-1, match_times);
}
fn kmp_nextj(target: &[u8], j:i32) -> i32 {
    let mut next = vec![-1; target.len()];
    let mut i = 0; let mut k: i32 = -1;
    while i < (target.len() - 1) {
        if k == -1 || target[i] == target[k as usize] {
            i += 1; k += 1;
            if target[i] != target[k as usize] {
                next[i] = k;
            } else {
                next[i] = next[k as usize];
            }
        } else {
            k = next[k as usize];
        }
    }
    return next[j as usize];
}

