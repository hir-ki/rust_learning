/*
問題4: 可変参照と借用

double_first という関数を作成し、Vec<i32> の可変参照を受け取り、最初の要素を2倍にしたものを返します。

*/

fn double_first(num: &mut Vec<i32>) {
    if num.is_empty() {
        panic!("Vector is empty");
    }
    num[0] *= 2;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_double_first_with_multiple_elements() {
        let mut numbers = vec![1, 2, 3];
        double_first(&mut numbers);
        assert_eq!(numbers, vec![2, 2, 3]);
    }

    #[test]
    #[should_panic(expected = "Vector is empty")]
    fn test_double_first_with_empty_vector() {
        let mut numbers: Vec<i32> = Vec::new();
        double_first(&mut numbers);
    }

    #[test]
    fn test_double_first_with_single_element() {
        let mut numbers = vec![5];
        double_first(&mut numbers);
        assert_eq!(numbers, vec![10]);
    }

    #[test]
    fn test_double_first_with_same_elements() {
        let mut numbers = vec![3, 3, 3];
        double_first(&mut numbers);
        assert_eq!(numbers, vec![6, 3, 3]);
    }
}
