use crate::marge_sort::merge_sort;

mod marge_sort;

fn main() {
    println!("sorted: {:?}",merge_sort(vec![0, 42, 3, 12].into()));
}

