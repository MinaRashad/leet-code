use std::vec;

fn main() {
    let mut nums = vec![1,1,2];

    remove_duplicates(&mut nums);
}

fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut last = 0;


    for i in 1..nums.len(){

        if nums[i] != nums[i-1]{
            last +=1;

            nums[last] = nums[i];

        }
        
    }


    println!("{nums:?} , {last}");

    (last+1) as i32
}