use std::io;

fn main() {
    myinfo();
    println!("Enter Student's Name : ");
    let name : String = gettext().trim().parse().expect("Failed Conversion");
    println!("Enter Subject 01 Marks: ");
    let marks01 : u8 = getmarks();
    println!("Enter Subject 02 Marks: ");
    let marks02 : u8 = getmarks();
    let percent = getpercentage(name.clone(),marks01,marks02);
    if percent >= 70.0
    {
        println!("{} Passed The Exam Scoring {} %", name, percent);
    }
    else
    {
        println!("{} Failed The Exam Scoring {} %", name, percent);
    }
    mydl();
}

fn gettext()-> String
{
    let mut text = String::new();
        io::stdin()
            .read_line(&mut text)
            .expect("Failed In Reading Input");
    return text;
}

fn getmarks() -> u8
{
    let text = gettext();
    return text.trim().parse().expect("Failed Conversion");
}

fn getpercentage(myname:String,mymarks01:u8,mymarks02:u8) -> f32
{
    mydl();
    println!("Student Name {}", myname);
    mysl();
    println!("Marks Obtained");
    mysl();
    println!("Subject 01 : {} / 100", mymarks01);
    println!("Subject 02 : {} / 100", mymarks02);
    let mytotalmarks : f32 = (mymarks01+mymarks02).into();
    let mypercentage : f32 = mytotalmarks/200.0*100.0;
    println!("Percentage : {} %", mypercentage);
    mydl();
    return mypercentage;

}

fn mydl() {
    println!("=====================");
}
fn mysl() {
    println!("---------------------");
}
fn myinfo() {
    mydl();
    println!("IOT Batch 3 Assignment 5");
    println!("Question No. 01");
    println!("Faisal Shahzad");
    println!("PIAIC49775");
    mydl();
}