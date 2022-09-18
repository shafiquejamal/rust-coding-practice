// --- Directions
// Given an array and chunk size, divide the array into many subarrays
// where each subarray is of length size
// --- Examples
// chunk([1, 2, 3, 4], 2) --> [[ 1, 2], [3, 4]]
// chunk([1, 2, 3, 4, 5], 2) --> [[ 1, 2], [3, 4], [5]]
// chunk([1, 2, 3, 4, 5, 6, 7, 8], 3) --> [[ 1, 2, 3], [4, 5, 6], [7, 8]]
// chunk([1, 2, 3, 4, 5], 4) --> [[ 1, 2, 3, 4], [5]]
// chunk([1, 2, 3, 4, 5], 10) --> [[ 1, 2, 3, 4, 5]]

pub fn chunk(s: Vec<i32>, size: usize) -> Vec<Vec<i32>> {
    let mut chunks: Vec<Vec<i32>> = vec![];
    let mut current_chunk: Vec<i32> = vec![];

    for &e in s.iter() {
        if current_chunk.len() < size {
            current_chunk.push(e)
        } else {
            chunks.push(current_chunk);
            current_chunk = vec![e];
        }
    }
    chunks.push(current_chunk);
    chunks
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(chunk(vec![1, 2, 3, 4], 2), vec![vec![1, 2], vec![3, 4]]);
    }

    #[test]
    fn case2() {
        assert_eq!(
            chunk(vec![1, 2, 3, 4, 5], 2),
            vec![vec![1, 2], vec![3, 4], vec![5]]
        );
    }

    #[test]
    fn case3() {
        assert_eq!(
            chunk(vec![1, 2, 3, 4, 5, 6, 7, 8], 3),
            vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8]]
        );
    }

    #[test]
    fn case4() {
        assert_eq!(
            chunk(vec![1, 2, 3, 4, 5], 4),
            vec![vec![1, 2, 3, 4], vec![5]]
        );
    }

    #[test]
    fn case5() {
        assert_eq!(chunk(vec![1, 2, 3, 4, 5], 10), vec![vec![1, 2, 3, 4, 5]]);
    }
}
