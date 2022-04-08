use std::cmp::Ordering;

pub fn insertion_sort<T, F>(list: &mut Vec<T>, compare: F) -> &mut Vec<T>
    where
        T: Copy,
        F: Fn(&T, &T) -> Ordering
{
    if list.len() <= 1 {
        return list;
    }

    for i in 1..list.len() {
        let key = list[i];
        let mut j = i;
        loop {
            j -= 1;
            if compare(&list[j], &key) == Ordering::Greater {
                list[j + 1] = list[j];
            }
            if j == 0 {
                break;
            }
        }
        list[j] = key;
    }
    list
}


#[test]
fn test() {
    let mut arr = vec![6, 5, 4, 3, 2, 1];
    insertion_sort(&mut arr, |a, b| a.cmp(b));
    assert_eq!(arr, vec![1, 2, 3, 4, 5, 6]);
    insertion_sort(&mut arr, |a, b| b.cmp(a));
    assert_eq!(arr, vec![6, 5, 4, 3, 2, 1]);

    let mut arr = vec![1];
    insertion_sort(&mut arr, |a, b| a.cmp(b));
    assert_eq!(arr, vec![1])
}
