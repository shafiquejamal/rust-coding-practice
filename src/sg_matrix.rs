fn get_next_direction(
    cur_dir: (i32, i32),
    cur_pos: (i32, i32),
    top: &mut i32,
    bottom: &mut i32,
    left: &mut i32,
    right: &mut i32,
) -> (i32, i32) {
    let proposed_next_dir = (cur_pos.0 + cur_dir.0, cur_pos.1 + cur_dir.1);
    if proposed_next_dir.1 > *right {
        *top += 1;
        (1, 0)
    } else if proposed_next_dir.1 < *left {
        *bottom -= 1;
        (-1, 0)
    } else if proposed_next_dir.0 > *bottom {
        *right -= 1;
        (0, -1)
    } else if proposed_next_dir.0 < *top {
        *left += 1;
        (0, 1)
    } else {
        cur_dir
    }
}

pub fn matrix(n: i32) -> Vec<Vec<i32>> {
    let mut matrix: Vec<Vec<i32>> = Vec::new();
    for _ in 1..=n {
        let mut v = vec![];
        for _ in 1..=n {
            v.push(0);
        }
        matrix.push(v)
    }

    let mut cur_dir = (0, 1);
    let mut top: i32 = 0;
    let mut bottom: i32 = n - 1;
    let mut left: i32 = 0;
    let mut right: i32 = n - 1;
    let mut cur_pos = (0, 0);

    for i in 1..=(n * n) {
        // println!("+++ cur_dir:{cur_dir:?}, cur_pos:{cur_pos:?}, top:{top:?}, bottom:{bottom:?}, left:{left:?}, right:{right:?},");
        matrix[cur_pos.0 as usize][cur_pos.1 as usize] = i;
        cur_dir = get_next_direction(
            cur_dir,
            cur_pos,
            &mut top,
            &mut bottom,
            &mut left,
            &mut right,
        );
        cur_pos = (cur_pos.0 + cur_dir.0, cur_pos.1 + cur_dir.1)
    }

    matrix
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            matrix(4),
            vec![
                vec![1, 2, 3, 4],
                vec![12, 13, 14, 5],
                vec![11, 16, 15, 6],
                vec![10, 9, 8, 7]
            ]
        );
    }
}
