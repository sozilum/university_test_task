use std::io;

pub fn dragon(num: i64){
    let mut num: i64 = num;
    let mut total: i64 = 0;

    if num % 3 > 0{
        total += 3;
        num -= 3;

        loop{
            total *= 3;
            num -= 3;
            if num == 4{
                total *= 4;
                break   
            }
            if num == 2{
                total *= 2;
                break
            }
        }

    }else{
        total += 3;
        num -= 3;
        loop{
            total *= 3;
            num -= 3;

            if num == 0{
                break
            }
            
        }
    }


    println!("{}", total);
}



fn main() {
    println!("Введите число для расчёта максимальной мощности: ");
    
    let mut aux_num = String::new();
    io::stdin().read_line(&mut aux_num).expect("Что-то пошло не так при вводе данных");
    
    match aux_num.trim().parse::<i64>(){
        Ok(num) => dragon(num),
        Err(_) => println!("Что-то пошло не так при попытке спарсить число"),
    }
}
