pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut i: usize = 0;
    let last_index = nums.len() - 1;
    loop {
        if i == last_index {
            break;
        }
        let mut j = i + 1;
        'inner: loop {
            println!("i:{i}, j:{j}, n1:{n1}, n2:{n2}", n1 = nums[i], n2 = nums[j]);
            if nums[i] + nums[j] == target {
                return vec![i as i32, j as i32];
            }
            if j == last_index {
                break 'inner;
            }
            j += 1;
        }
        i += 1;
    }
    vec![0, 1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0, 1])
    }

    #[test]
    fn case2() {
        assert_eq!(two_sum(vec![3, 2, 4], 6), vec![1, 2])
    }

    #[test]
    fn case3() {
        assert_eq!(two_sum(vec![3, 3], 6), vec![0, 1])
    }

    #[test]
    fn case4() {
        assert_eq!(two_sum(vec![3, 2, 3], 6), vec![0, 2])
    }
}
