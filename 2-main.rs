fn main() {
    let v= vec![0,1,2,5];

    println!("sum is {:?}",sum(&v));
}

fn sum(Vec:&[u32]) -> Option<u32>{
    let mut sum: Option<u32> = Some(0);
    
    for &i in Vec
    {
        sum = sum.unwrap().checked_add(i);
    }

    sum
} 