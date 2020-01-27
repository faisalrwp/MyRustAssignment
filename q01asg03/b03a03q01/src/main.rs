use std::io;

fn main() {
    let mut myweight = String::new();
    let mut myheight = String::new();
    myinfo();
    println!("Enter Your Weight (KG) ");
    io::stdin()
        .read_line(&mut myweight)
        .expect("Failed In Reading Input");
    println!("Enter Your Height (cm) ");
    io::stdin()
        .read_line(&mut myheight)
        .expect("Failed In Reading Input");
    mysl();
    let myweight: f64 = myfloat(myweight);
    let myheight: f64 = myfloat(myheight);
    let mybmi: f64 = mybmi(myheight, myweight);

    println!(
        "Your Height : {} \nYour Weight : {}\nYour BMI : {}",
        myheight, myweight, mybmi
    );
    mydl();
    mybmiquality(mybmi);
    mydl();
}

fn mybmi(myh: f64, myw: f64) -> f64 {
    let myh = mycmtom(myh);
    let myreturn: f64 = myw / (myh * myh);
    return myreturn;
}
fn mybmiquality(mybmi: f64) {
    if mybmi > 25.0 {
        println!("OverWeight ...");
    } else if mybmi > 18.5 {
        println!("Normal Weight ...");
    } else {
        println!("UnderWeight ...");
    }
}
// Convert String To Float
fn myfloat(mystr: String) -> f64 {
    let myreturn: f64 = mystr
        .trim()
        .parse()
        .expect("ERROR : Only Pass Number Based String");
    return myreturn;
}
// Convert centimeter To meter
fn mycmtom(mycm: f64) -> f64 {
    let myreturn: f64 = mycm / 100.0;
    return myreturn;
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
    println!("Question No. 01");
    println!("Faisal Shahzad");
    println!("PIAIC49775");
    mydl();
}
