/// Trova un duplicato in `a` (n+1 valori in {0, ..., n-1}) mandando ogni valore v
/// nella cella di indice v. O(n) tempo, O(1) spazio extra, modifica `a`.
pub fn my_sol(a: &mut [usize]) -> Option<usize> {
    for i in 0..a.len() {
        while a[i] != i {
            let v = a[i];
            if a[v] == v {
                return Some(v);
            }
            a.swap(i, v);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_duplicate_in_lesson_example() {
        let mut a = vec![1, 3, 5, 6, 5, 2, 0, 4, 7];
        assert_eq!(my_sol(&mut a), Some(5));
    }

    #[test]
    fn finds_duplicate_when_it_sits_in_last_cell() {
        // la cella n non può mai sistemarsi: il duplicato va trovato comunque
        let mut a = vec![0, 1, 2, 3, 2];
        assert_eq!(my_sol(&mut a), Some(2));
    }

    #[test]
    fn returns_a_value_that_appears_twice_with_many_duplicates() {
        let orig = vec![0, 0, 0, 1, 1];
        let d = my_sol(&mut orig.clone()).expect("un duplicato esiste sempre");
        assert!(orig.iter().filter(|&&x| x == d).count() >= 2);
    }

    #[test]
    fn returns_none_without_duplicates() {
        let mut a = vec![2, 0, 1];
        assert_eq!(my_sol(&mut a), None);
    }
}

pub mod claude;
