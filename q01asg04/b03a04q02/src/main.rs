fn main() {
    myinfo();
    let rollno:u8=10;
    let percentage:f32=88.8;
    let grade:char='A';
    printer(rollno,percentage,grade);
    mydl();
}

fn printer(rno:u8,per:f32,gr:char)
{
    println!("  Student Data Sheet");
    mysl();
    println!("   Roll Number : {}", rno);
    println!("    Percentage : {}", per);
    println!("Grade Obtained : {}", gr);
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
    println!("Question No. 02");
    println!("Faisal Shahzad");
    println!("PIAIC49775");
    mydl();
}