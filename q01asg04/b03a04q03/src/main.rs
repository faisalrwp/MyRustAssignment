fn main() {
    let num:i32 = 22;
    myinfo();
    println!("The Number To Be Squared Is {}", num);
    mysl();
    let mytuple=mysquare(num);
    println!("The Square Is {:?}", mytuple);
    mydl()
}

fn mysquare(mynum:i32) -> (i32,i64)
{
    (mynum, (mynum*mynum).into())
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
    println!("Question No. 03");
    println!("Faisal Shahzad");
    println!("PIAIC49775");
    mydl();
}
