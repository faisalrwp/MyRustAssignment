fn main() 
{
    myinfo();
    let temp : i8 = -10;
    println!("Current Temperature Is {}", temp);
    mysl();
    tester(temp);
    mydl();
}

fn tester(num:i8)
{
    if num < 0
    { println!("Negetive Temperature - Freezing");}
    else
    if num == 0
    {println!("Zero Temperature - Get Ready For Rough Weather");}
    else
    {println!("Positive Temperature - Its Summer Time Ahead");}
}

fn mydl() {
    println!("=====================");
}
fn mysl() {
    println!("---------------------");
}
fn myinfo() {
    mydl();
    println!("IOT Batch 3 Assignment 4");
    println!("Question No. 01");
    println!("Faisal Shahzad");
    println!("PIAIC49775");
    mydl();
}
