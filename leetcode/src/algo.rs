struct Algo;

impl Algo {
    //规定数组可能包含重复元素，其余不变。
    //给定一个长度为的有序数组 nums 和一个元素 target,数组不存在重复元素。
    //现将 target 插入数组 nums 中，并保持其有序性。
    //若数组中已存在元素 target ，则插入到其左方。
    //请返回插入后 target 在数组中的索引。

    pub fn binary_search<T>(ordered_seq: &[T], target: T) -> Result<usize, String>
    where
        T: PartialOrd,
    {
        if ordered_seq.is_empty() {
            return Err("Not found".to_string());
        }
        let mut start = 0;
        let mut end = ordered_seq.len().saturating_sub(1);

        while start < end {
            let mid_num = start + (end - start).saturating_div(2);
            match ordered_seq[mid_num].partial_cmp(&target) {
                Some(std::cmp::Ordering::Equal) => end = mid_num,
                Some(std::cmp::Ordering::Less) => start = mid_num + 1,
                Some(std::cmp::Ordering::Greater) => end = mid_num,
                None => return Err("Comparison failed".to_string()),
            }
        }
        Ok(start)
    }

    pub fn binary_search_left_edge<T>(ordered_seq: &[T], target: T) -> i32
    where
        T: PartialOrd + Clone,
    {
        if let Ok(i) = Self::binary_search(ordered_seq, target.clone()) {
            if i < ordered_seq.len() && ordered_seq[i] == target {
                return i as i32;
            }
        }
        -1
    }

    pub fn selection_sort<T>(disordered_seq: &mut [T])
    where
        T: PartialOrd,
    {
        if disordered_seq.is_empty() || disordered_seq.len() == 1 {
            return;
        }
        let n = disordered_seq.len();
        for i in 0..n - 1 {
            let mut k = i;
            for j in i + 1..n {
                if disordered_seq[k] > disordered_seq[j] {
                    k = j;
                }
            }
            disordered_seq.swap(i, k);
        }
    }

    pub fn bubble_sort<T>(disordered_seq: &mut [T])
    where
        T: PartialOrd,
    {
        let n = disordered_seq.len();
        for i in 0..n {
            let mut flag = false;
            for j in 0..n-i-1 {
                if disordered_seq[j] > disordered_seq[j + 1] {
                    disordered_seq.swap(j, j + 1);
                    flag = true;
                }
            }
            if !flag {
                break; // 如果没有交换，说明已经有序
            }
        }
    }

    
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_binary_search() {
        let ordered_seq = vec![1, 2, 3, 4, 5];
        assert_eq!(Algo::binary_search(&ordered_seq, 3), Ok(2));
        assert_eq!(Algo::binary_search(&ordered_seq, 6), Ok(4));
        assert_eq!(Algo::binary_search(&ordered_seq, 0), Ok(0));
    }

    #[test]
    fn test_binary_search_left_edge() {
        let ordered_seq = vec![1, 2, 2, 3, 4, 5];
        assert_eq!(Algo::binary_search_left_edge(&ordered_seq, 2), 1);
        assert_eq!(Algo::binary_search_left_edge(&ordered_seq, 3), 3);
        assert_eq!(Algo::binary_search_left_edge(&ordered_seq, 6), -1);
        assert_eq!(Algo::binary_search_left_edge(&ordered_seq, 0), -1);
    }

    #[test]
    fn test_selection_sort() {
        let mut disordered_seq = vec![5, 3, 1, 4, 2];
        Algo::selection_sort(&mut disordered_seq);
        assert_eq!(disordered_seq, vec![1, 2, 3, 4, 5]);

        let mut single_element = vec![42];
        Algo::selection_sort(&mut single_element);
        assert_eq!(single_element, vec![42]);

        let mut empty_seq: Vec<i32> = Vec::new();
        Algo::selection_sort(&mut empty_seq);
        assert!(empty_seq.is_empty());
    }

    #[test]
    fn test_bubble_sort() {
        let mut disordered_seq = vec![5, 3, 1, 4, 2];
        Algo::bubble_sort(&mut disordered_seq);
        assert_eq!(disordered_seq, vec![1, 2, 3, 4, 5]);

        let mut single_element = vec![42];
        Algo::bubble_sort(&mut single_element);
        assert_eq!(single_element, vec![42]);

        let mut empty_seq: Vec<i32> = Vec::new();
        Algo::bubble_sort(&mut empty_seq);
        assert!(empty_seq.is_empty());

        let mut char_seq = vec!['e', 'd', 'c', 'b', 'a'];
        Algo::bubble_sort(&mut char_seq);
        assert_eq!(char_seq, vec!['a', 'b', 'c', 'd', 'e']);

        let mut float_seq = vec![3.1, 2.2, 5.5, 4.4, 1.0];
        Algo::bubble_sort(&mut float_seq);
        assert_eq!(float_seq, vec![1.0, 2.2, 3.1, 4.4, 5.5]);

        let mut string_seq = vec!["apple", "orange", "banana", "grape"];
        Algo::bubble_sort(&mut string_seq);
        assert_eq!(string_seq, vec!["apple", "banana", "grape", "orange"]);
    }
}
