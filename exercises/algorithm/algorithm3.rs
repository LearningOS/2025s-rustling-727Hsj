/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

fn sort<T: std::cmp::PartialOrd + Clone>(array: &mut [T], left: usize, right: usize){
	//TODO 快速排序
    if left < right{
        let mid = partition(array, left, right);
        if mid > 0{
            sort(array, left, mid-1);
        }
        sort(array, mid+1, right);
    }
}

fn partition<T: std::cmp::PartialOrd + Clone>(array: &mut [T], left: usize, right: usize) -> usize{
    let mut l = left+1;
    let mut r = right;
    let pivot = array[left].clone();

    let mid = loop {
        while l <= r && &array[l] <= &pivot {
            l += 1;
        }
        while l <= r && &array[r] >= &pivot {
            r -= 1;
        }


        if l >= r{
            break r
        }
        
        array.swap(l, r);
    };

   array[left] = array[mid].clone();
   array[mid] = pivot.clone();

   mid

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        let len = vec.len();
        sort(&mut vec, 0 as usize, len-1);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
	#[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        let len = vec.len();
        sort(&mut vec, 0 as usize, len-1);
        assert_eq!(vec, vec![1]);
    }
	#[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        let len = vec.len();
        sort(&mut vec, 0 as usize, len-1);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}