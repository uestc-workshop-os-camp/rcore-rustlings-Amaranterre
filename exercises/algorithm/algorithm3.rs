/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

fn sort<T : PartialOrd + Clone>(array: &mut [T]){
    let len = array.len();

	let mut ranges : Vec<(usize, usize)> = Vec::new();
    ranges.push((0, len - 1));

    while let Some((range_left, range_right)) = ranges.pop() {
        let mut left = range_left;
        let mut right = range_right;
        let mid = (left + right) / 2;

        if left >= right {
            continue;
        }

        // get median as the mid number
        let mid_number = if (array[left] > array[right]) == (array[left] < array[mid]) {
            array[left].clone()
        } else if (array[mid] > array[left]) == (array[mid] < array[right]) {
            array[mid].clone()
        } else {
            array[right].clone()
        };

        while left < right {
            while array[left] < mid_number && left < right {
                left+=1;
            }
            while array[right] > mid_number && left < right {
                right-=1;
            }
            // std::mem::swap(&mut array[left], &mut array[right]);
            array.swap(left, right);
        }

        if array[left] < mid_number {
            left+=1;
        }

        ranges.push((range_left, left - 1));
        ranges.push((right + 1, range_right));


    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
	#[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
	#[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}