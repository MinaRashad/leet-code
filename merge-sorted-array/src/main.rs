fn main() {
    println!("Hello, world!");
    let mut nums1 = vec![1,2,4,5,6,0];
    let mut nums2 = vec![3];
    let m = 5;
    let n = 1;
    merge(&mut nums1, m, &mut nums2, n);

}

fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {

    // we should not use additional space. 
    // num1 length is m+n, so we will use the end
    let mut i = m-1; // this point to the last element in num 1
    let mut j = n-1; // this point to the last element in num 2
    let mut k = m+n-1;// this points to the last space in num1

    while j >= 0 {
        let num1 = if i>=0 {nums1[i as usize]} else{i32::MIN};
        let num2 = nums2[j as usize];

        if num1 >= num2{
            nums1[k as usize] = nums1[i as usize];
            i -= 1;
        }
        else if num1 < num2{
            nums1[k as usize] = nums2[j as usize];
            j -= 1;
        }
        k -= 1;
    }

    println!("{:?}",nums1);
    // now there are two cases, if num2 is finished first
    // we have the correct answer
    // if num1 finished first, 
}
