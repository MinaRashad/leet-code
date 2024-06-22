fn main() {
    let mut nums = vec![];
    let val = 0;

    remove_element(&mut nums, val);
}


// Wow this beats 100% 
// awesome!

fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let mut count: i32 = 0;
    let mut start: i32 = 0;
    let mut end:i32 = nums.len() as i32 -1;


    while start <= end {
        println!("{start}");

        let current_num = nums[start as usize];
        if current_num != val{
            count += 1
        }

        else{

            // equal to val then loop from the back until we get to a non equal
            let mut replacement = val;
            while replacement == val {
                replacement = nums[end as usize];

                end -= 1;

                // in case all remaining are val, then we should break
                if start >= end {
                    break
                }     
            }

            // re-check if replacement is not equal
            if replacement != val {
                println!("{nums:?}, {start}, {replacement}");

                nums[start as usize] = replacement;
                println!("{:?}",nums);

                count += 1; // since we found another non equal number
            }
            
        }

        start += 1;

    }

    println!("{:?}",nums);
    println!("{}",count);

    return count;
}