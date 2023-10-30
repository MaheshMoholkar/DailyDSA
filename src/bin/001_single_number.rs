// Given a non-empty array of integers, every element appears twice except for one. Find that single one.

fn find(arr: Vec<i32>){
    let mut result = 0;
    for num in arr{
        result ^= num;
    }
    println!("{}",result);
}

fn main(){
    let arr = vec![4, 1, 2, 1, 2, 4, 3];
    find(arr);
}