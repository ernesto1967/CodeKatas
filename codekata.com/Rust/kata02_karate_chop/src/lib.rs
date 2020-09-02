mod iterative {
    /// The function 'chop' shall search the value in the array and return its index.
    /// If the value is not found in the array -1 shall be returned.
    #[allow(dead_code)]
    pub fn chop(value: i32, array: &Vec<i32>) -> isize {
        if array.len() > 0 {
            let mut min_index = 0usize;
            let mut max_index = array.len()-1;

//            println!("chop({}, {:?}) min:{} max:{}", value, array, min_index, max_index);

            loop {
                let test_index = (min_index + max_index) / 2;
                let test_value = array[test_index];
//                println!("chop({}, {:?}) min:{} max:{} test:{} -> {}", value, array, min_index, max_index, test_index, test_value);

                if value == test_value {
	            return test_index as isize;
                }
                else if value < test_value {
                    if test_index > min_index {
                        max_index = test_index -1;
                    } else {
                        break;
                    }
                }
                else {
                    if test_index < max_index {
                        min_index = test_index +1;
                    } else {
                        break;
                    }
                } 
            }

//            println!("chop: value not found");
        } else {
//            println!("chop({}, {:?}) empty array", value, array);
        }
        -1
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn test_chop_iterative() {
        use super::iterative::chop as chop;
        assert_eq!(-1, chop(3, &vec![]));
        assert_eq!(-1, chop(3, &Vec::from(vec![1])));
        assert_eq!(0,  chop(1, &Vec::from(vec![1])));
        
        assert_eq!(0,  chop(1, &Vec::from(vec![1, 3, 5])));
        assert_eq!(1,  chop(3, &Vec::from(vec![1, 3, 5])));
        assert_eq!(2,  chop(5, &Vec::from(vec![1, 3, 5])));
        assert_eq!(-1, chop(0, &Vec::from(vec![1, 3, 5])));
        assert_eq!(-1, chop(2, &Vec::from(vec![1, 3, 5])));
        assert_eq!(-1, chop(4, &Vec::from(vec![1, 3, 5])));
        assert_eq!(-1, chop(6, &Vec::from(vec![1, 3, 5])));

        assert_eq!(0,  chop(1, &Vec::from(vec![1, 3, 5, 7])));
        assert_eq!(1,  chop(3, &Vec::from(vec![1, 3, 5, 7])));
        assert_eq!(2,  chop(5, &Vec::from(vec![1, 3, 5, 7])));
        assert_eq!(3,  chop(7, &Vec::from(vec![1, 3, 5, 7])));
        assert_eq!(-1, chop(0, &Vec::from(vec![1, 3, 5, 7])));
        assert_eq!(-1, chop(2, &Vec::from(vec![1, 3, 5, 7])));
        assert_eq!(-1, chop(4, &Vec::from(vec![1, 3, 5, 7])));
        assert_eq!(-1, chop(6, &Vec::from(vec![1, 3, 5, 7])));
        assert_eq!(-1, chop(8, &Vec::from(vec![1, 3, 5, 7])));

    }
}
