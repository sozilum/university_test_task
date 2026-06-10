pub fn find_index(list: Vec<i32>, num: i32) -> std::io::Result<i32>{
    let mut target_index: i32 = 0;
    match list.iter().position(|&target| target == num){
        Some(index) => {
            target_index = index as i32
        },
        None => println!("Что-то пошло не так"),
    }
    Ok(target_index)
}

pub fn range_two_sum(list: Vec<i32>) -> std::io::Result<i32>{
    let mut min_num: i32 = list[0];
    let mut max_num: i32 = list[0];
    let mut index: Vec<i32> = Vec::new();
    let mut total: i32 = 0;

    for num in &list{

        if num < &min_num{
            min_num = *num;
        };
        if num > &max_num{
            max_num = *num;
       };
    }

    index.push(find_index(list.clone(), min_num)?);
    index.push(find_index(list.clone(), max_num)?);
    
    index.sort();
    
    for num in &list[index[0] as usize..index[1] as usize]{
        if *num < 0{
            total += num
        }
    }
    
    Ok(total)
    }
    
    


fn main() {
    let test_vec: Vec<i32> = vec![10,-6,1,2,9,3,4,5,6,-6,-7,-4,2];
    let total = range_two_sum(test_vec);

    println!("Сумма чисел между минимальным и максимальным значением равна: {}", total.unwrap());
}
