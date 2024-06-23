fn main() {
    let nums = vec![2,2,2,1,2,2,1,2,2,2]    ;
    let k = 3;

    let nice = number_of_subarrays(nums, k);

    println!("{nice}");
}

fn number_of_subarrays(nums: Vec<i32>, k: i32) -> i32 {
    let mut count = 0;
    
    let mut odd_index: Vec<usize> = vec![];
    let mut odds_len: usize = 0;
    for i in 0..nums.len(){
        if nums[i]%2 == 1 {
            odd_index.push(i);
            odds_len +=1;
        }
    }
    let max = nums.len() - 1;

    // now for each index i to i+k we have a desired sub-array
    // from that sub-array, we can find other sub-arrays
    // by muliplying the (difference between i and i-1) + 1
    // with (difference between i+k and i+k+1) +1

    if odds_len < k as usize {return 0;}

    println!("{:?}",odd_index);
    for (i,_odd_i) in odd_index[..=odds_len - k as usize].iter().enumerate(){
        let start: usize = i;
        let end: usize = i + (k as usize) -1;
        let pre_diff: usize;
        let next_diff:usize;

        println!("{start} {end}");
        if start > 0{
            pre_diff = odd_index[start] - odd_index[start-1];
        }
        else{
            // if its the first element, everything before the element counts
            pre_diff = odd_index[start]+1
        }
        if end < odds_len-1{
            next_diff = odd_index[end+1] - odd_index[end];
        }
        else{
            // at the end of the array, everything after counts
            next_diff = (max - odd_index[end]) +1;
        }
        println!("i:{i}, {pre_diff}*{next_diff}");
        count += pre_diff * next_diff;

    }



    count as i32
}