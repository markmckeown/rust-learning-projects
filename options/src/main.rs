fn main() {
    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    let sum: i8;
    let sum2 :i8;

    match y {
       Some(i) => {
           sum = x + i;
       }
       None => {
           sum = 0;
       }
    }

    if let Some(i) = y {
       sum2 = x + i;
    } else {
       sum2 = 0;       
    }

    println!("Sum: {}", sum);
    println!("Sum: {}", sum2);
}
