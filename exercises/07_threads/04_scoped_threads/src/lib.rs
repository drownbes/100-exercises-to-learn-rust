// TODO: Given a vector of integers, split it in two halves
//  and compute the sum of each half in a separate thread.
//  Don't perform any heap allocation. Don't leak any memory.

pub fn sum(v: Vec<i32>) -> i32 {
    let slice = v.leak();
    let split_point = slice.len() / 2;
    let v0 = &slice[..split_point];
    let v1 = &slice[split_point..];
    std::thread::scope(|scope| {
        let p0 = scope.spawn(move || {
            v0.iter().sum::<i32>()
        });
        let p1 = scope.spawn(move || {
            v1.iter().sum::<i32>()
        });
        p0.join().unwrap() + p1.join().unwrap()
    })
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(sum(vec![]), 0);
    }

    #[test]
    fn one() {
        assert_eq!(sum(vec![1]), 1);
    }

    #[test]
    fn five() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5]), 15);
    }

    #[test]
    fn nine() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]), 45);
    }

    #[test]
    fn ten() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 55);
    }
}
