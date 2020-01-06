fn main() {
    myinfo();
    let myarray: [i8; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mut myindex: usize = 0;
    while myindex < 10 {
        println!("The Number Is {}", myarray[myindex]);
        if myarray[myindex] == 3 || myarray[myindex] == 7 || myarray[myindex] == 10 {
            mysl();
            println!("Special Security Check");
            mysl();
        }
        myindex += 1;
    }
    mydl();
}
fn mydl() {
    println!("=====================");
}
fn mysl() {
    println!("---------------------");
}
fn myinfo() {
    mydl();
    println!("IOT Batch 3 Assignment 3");
    println!("Question No. 02");
    println!("Faisal Shahzad");
    println!("PIAIC49775");
    mydl();
}
