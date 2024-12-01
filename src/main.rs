fn partition<T>(slice: &mut [T]) -> usize
where
    T: PartialOrd + Clone,
{
    let len = slice.len();
    let pivot_index = len - 1;
    let pivot = &slice[pivot_index].clone();
    let mut i = 0;

    for j in 0..pivot_index {
        if slice[j] <= *pivot {
            slice.swap(i, j);
            i += 1;
        }
    }

    slice.swap(i, pivot_index);
    i
}

fn quick_sort<T>(slice: &mut [T])
where
    T: PartialOrd + Clone,
{
    if !slice.is_empty() {
        let partition_index = partition(slice);
        let len = slice.len();

        quick_sort(&mut slice[0..partition_index]);
        quick_sort(&mut slice[partition_index + 1..len]);
    }
}

fn main() {
    let mut v = vec![3, 5, 6, 8, 1, 2];

    quick_sort(&mut v);
    println!("{v:?}");
}

#[cfg(test)]
mod test {
    use super::quick_sort;

    #[test]
    fn int_test() {
        let mut v = vec![3, 5, 6, 8, 1, 2];
        let v_sol = vec![1, 2, 3, 5, 6, 8];

        quick_sort(&mut v);
        assert_eq!(v, v_sol);
    }

    #[test]
    fn char_test() {
        let mut v = vec!['u', 'b', 'i', 'J', '8'];
        let v_sol = vec!['8', 'J', 'b', 'i', 'u'];

        quick_sort(&mut v);
        assert_eq!(v, v_sol);
    }

    #[test]
    fn float_test() {
        let mut v = vec![0.556, 1.2, 1.78, 0.331, 0.451];
        let v_sol = vec![0.331, 0.451, 0.556, 1.2, 1.78];

        quick_sort(&mut v);
        assert_eq!(v, v_sol);
    }

    #[test]
    fn str_test() {
        let mut v = vec![
            "hello",
            "it's me",
            "I begane to wonder",
            "If After all these years",
        ];
        let v_sol = vec![
            "I begane to wonder",
            "If After all these years",
            "hello",
            "it's me",
        ];

        quick_sort(&mut v);
        assert_eq!(v, v_sol);
    }
}
