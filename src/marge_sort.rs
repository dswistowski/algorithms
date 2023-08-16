use std::collections::VecDeque;


pub fn merge_sort<T: PartialOrd>(array: VecDeque<T>) -> VecDeque<T> {
    match array.len() {
        1 => array,
        _ => {
            let (l, r) = split(array);
            merge(merge_sort(l), merge_sort(r))
        }
    }
}

fn split<T>(mut array: VecDeque<T>) -> (VecDeque<T>, VecDeque<T>) {
    let right = array.split_off(array.len() / 2);
    return (array, right)
}

fn merge<T: PartialOrd>(mut l: VecDeque<T>, mut r: VecDeque<T>) -> VecDeque<T> {
    let mut ret: VecDeque<T> = VecDeque::new();
    while l.len() > 0 && r.len() > 0 {
        ret.push_back(
            match l[0] < r[0] {
                true => {
                    l.pop_front().unwrap()
                },
                false => {
                    r.pop_front().unwrap()
                }
            }
        )
    }

    for e in l {
        ret.push_back(e)
    }
    for e in r {
        ret.push_back(e)
    }
    ret
}



#[cfg(test)]
mod tests {
    use super::*;
    use rand::thread_rng;
    use rand::seq::SliceRandom;
    
    #[test]
    fn can_sort() {
        assert_eq!(merge_sort(vec![3,4,3,1,0].into()), vec![0,1,3,3,4])
    }

    #[test]
    fn can_sort_random() {
        let mut rng = thread_rng();
        for _ in 0..100 {
            let mut vec: Vec<u32> = (0..1000).collect();
            let before = vec.clone();
            vec.shuffle(&mut rng);
            // can be false positive but c\mo let play lotto
            assert_ne!(vec, before);
            assert_eq!(merge_sort(vec.into()), before)
        }
    }
}