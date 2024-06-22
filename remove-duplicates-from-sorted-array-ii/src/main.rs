fn main() {
    let mut nums = vec![1,2,3];

    remove_duplicates(&mut nums);
}

fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut last = 1;

    if nums.len() <=2{
        return nums.len() as i32;
    }


    for i in 2..nums.len(){

        if nums[i] != nums[last] || (nums[last] != nums[last-1]){
            last +=1;

            nums[last] = nums[i];

        }
        
    }


    println!("{nums:?} , {last}");

    (last+1) as i32
}