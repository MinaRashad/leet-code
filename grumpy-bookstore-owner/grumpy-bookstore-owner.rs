fn main(){
    println!("Hello world");

    let _ = max_satisfied(vec![10,1,7], vec![0,0,0], 3);
}


fn max_satisfied(customers: Vec<i32>, grumpy: Vec<i32>, minutes: i32) -> i32 {
    let total_minutes = customers.len() as usize;
    let minutes = minutes as usize;

    let mut sum = 0;

    for i in 0..minutes as usize{
        sum += customers[i] * (1-grumpy[i]);
    }

    let mut max = sum;


    for i in minutes..total_minutes as usize{
        // first remove the first element from the sum

        let elem_remove = customers[i-minutes] * (1-grumpy[i-minutes]);
        let elem_add = customers[i] * (1-grumpy[i]);

        sum += elem_add - elem_remove;

        if sum <= 0{
            sum =0;
        }        

        if sum > max {
            max = sum;
        }

    }

    // now we know that max is the maximum value the shopkeeper can please
    // with the trick now we just see how many people will be pleased 
    // without tricks
    let mut sum = 0;

    for i in 0..total_minutes as usize{
        sum += customers[i] * grumpy[i];
    }


    return sum + max;
}