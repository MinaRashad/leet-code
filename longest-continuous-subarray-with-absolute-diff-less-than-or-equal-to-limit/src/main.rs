fn main() {
    let nums = vec![1,1,1,1,1,2,13,11,10];
    let limit = 10;
    longest_subarray(nums, limit);
}


fn longest_subarray(nums: Vec<i32>, limit: i32) -> i32 {
    // we know that any sub-array of 1 will always have a
    // max diff of zero

    let mut start =0;
    let mut end = 0;
    let mut max_size = 0;    
    let mut max = nums[0];
    let mut min = nums[0];


    while end != nums.len(){
        let mut sub_array_size = 0;

        println!("{start}->{end}, {max}-{min}");
        // getting max and min in the first sub array
        if max - min <= limit{
            sub_array_size = end - start +1;
            if sub_array_size > max_size {
                max_size = sub_array_size;
            }

            end +=1;

            // update max and min, since we are not removing
            // we can just check if the new number is max or min
            if end != nums.len(){
                if nums[end] > max{
                    max = nums[end]
                }
                if nums[end] < min{
                    min = nums[end]
                }
            }
        }
        else{
            start +=1;

            // if we remove a number we should check the number
            // removed is a max or a min, if so
            // we would need to loop through the list to find the
            // new min

            let removed = nums[start-1];

            // loop through the remaining nums
            // no need to do anything if:
            // there is another copy of removed
            // or if removed is not equal to max or min

            // since we are removing, if there is another copy of

            if removed == max || removed == min {
                let mut temp_max = -1;
                let mut temp_min = i32::MAX;
                let mut duplicated = false;
                for i in nums[start..=end].iter(){
                    if *i == removed{
                        duplicated = true;
                        break;
                    }
                    if *i > temp_max{
                        temp_max = *i;
                    }

                    if *i < temp_min{
                        temp_min = *i;
                    }
                }

                if !duplicated{
                    max = temp_max;
                    min = temp_min;
                }

                
            }
        }

    }
    
    println!("{max_size}");
    max_size as i32
}