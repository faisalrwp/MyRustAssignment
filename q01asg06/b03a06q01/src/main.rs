use std::io;

#[derive(Debug)]
// Q1 Part 1 - define a custom datatype using Struct 
// with your favorite name. Which contains 3 fields, 
// and 1 should be String Type.
struct student
{ name:String, batch:u8, rollno:u32,quiz1:u8,quiz2:u8,quiz3:u8}

fn main() {
    myinfo();
    
    // Q1 Part 2A - Create an instance of the above defined struct
    let st01 = getStudents();
    mydl();
    // Q1 Part 2B - Print complete instance
    mysl();
    println!("Student Data {:?}",st01);
    mysl();
    // Q1 Part 2C - Print instance by calling its fields.
    println!("Batch {} Stduent {} Having Roll No. PIAIC - {} Results Are As Under", st01.batch, st01.name, st01.rollno);
    println!("Quiz 1 - {}/100 ; Quiz 2 - {}/100 ; Quiz 3 - {}/100", st01.quiz1, st01.quiz2, st01.quiz3);
    mysl();
    // Q1 Part 2D - create another instance by using fields of first instance
    let st02 = student
    {
        name:String::from("Shahzad"),batch:st01.batch-1,
        rollno:st01.rollno/2,quiz1:st01.quiz1/2,quiz2:st01.quiz2/2,quiz3:st01.quiz3/2
    };
    mydl();
    mysl();
    println!("Student Data {:?}",st02);
    printStudent(&st02);
    mydl();

    // Q1 Part 4 - in main function call user defined function, 
    // Print instance returned by the user defined function.
    let st03=addStudent(String::from("Masood"),3,54321,50, 80, 40);
    mysl();
    println!("Student Data {:?}",st03);
    printStudent(&st03);
    
    // Extra Work Call - Showing Graph of the marks Obtained
    println!("Display Graphs Of Students Marks (y/n) ? ");
    let option : char =gettext().trim().to_uppercase().parse().expect("Failed Conversion");
    if  (option == 'Y') 
    {
        graphStudent(&st01);
        graphStudent(&st02);
        graphStudent(&st03);
    }
}

// Q1 Part 3 - define user defined function, 
// User defined function should receive some arguments and 
// return an instance of your above defined struct.
fn addStudent(name:String,batch:u8,rollno:u32,quiz1:u8,quiz2:u8,quiz3:u8) -> student
{
    let myst=student{name,batch,rollno,quiz1,quiz2,quiz3};
    return myst;
}
fn printStudent(st:&student)
{
    mysl();
    println!("Batch {} Stduent {} Having Roll No. PIAIC - {} Results Are As Under", st.batch, st.name, st.rollno);
    println!("Quiz 1 - {}/100 ; Quiz 2 - {}/100 ; Quiz 3 - {}/100", st.quiz1, st.quiz2, st.quiz3);
    mysl();
}

fn getStudents() -> student
{
    println!("Enter Student's Name : ");
    let name : String = gettext().trim().parse().expect("Failed Conversion");
    println!("Enter Student's Batch : ");
    let batch : u8 = gettext().trim().parse().expect("Failed Conversion");
    println!("Enter Student's Roll # : ");
    let rollno : u32 = gettext().trim().parse().expect("Failed Conversion");
    println!("Enter Quiz 01 Marks: ");
    let quiz1 : u8 = gettext().trim().parse().expect("Failed Conversion");
    println!("Enter Quiz 02 Marks: ");
    let quiz2 : u8 = gettext().trim().parse().expect("Failed Conversion");
    println!("Enter Quiz 03 Marks: ");
    let quiz3 : u8 = gettext().trim().parse().expect("Failed Conversion");
    let st = student{name,batch,rollno,quiz1,quiz2,quiz3};
    return st;
}

fn gettext()-> String
{
    let mut text = String::new();
        io::stdin()
            .read_line(&mut text)
            .expect("Failed In Reading Input");
    return text;
}


// Extra Work Functions - Showing Graph of the marks Obtained

fn PrintHGrid()
{
    println!("       ------------------------------");
    println!("         1  2  3  4  5  6  7  8  9 1");
    println!("         0  0  0  0  0  0  0  0  0 00");

}
fn PrintBar(quiz:u8,marks:u8)
{
    print!("Quiz {}| {} ",quiz, GetEmoji(marks));
    for i in 1..(marks/10)
    {    print!("{} ",GetEmoji(marks)); }
    println!(" ({})", marks);
}

fn printlegends()
{
    println!("------------------------------------------------------------");
    print!(" | {} 90+",  GetEmoji(99));
    print!(" | {} 80+",  GetEmoji(89));
    print!(" | {} 70+",  GetEmoji(79));
    print!(" | {} 60+",  GetEmoji(69));
    print!(" | {} 50 +",  GetEmoji(59));
    print!(" | {} 00-50 |",  GetEmoji(49));
    println!("");
    println!("------------------------------------------------------------");

}

fn GetEmoji(marks:u8) -> char
{
    if marks >= 90
    { return '\u{1F44C}'}
    else if marks >= 80
    { return '\u{1F44D}'}
    else if marks >= 70
    { return '\u{1F44F}'}
    else if marks >= 60
    { return '\u{1F97A}'}
    else if marks >= 50
    { return '\u{1F535}'}
    else 
    { return '\u{1F534}'}
    
}

fn graphStudent(st:&student)
{
    println!("");
    mydl();
    println!("Graphical Result of {}",st.name);
    mysl();
    println!("Batch : {} <> Roll Number : {}",st.batch,st.rollno); 
    mysl();
    printlegends();
    PrintBar(1,st.quiz1);
    PrintBar(2,st.quiz2);
    PrintBar(3, st.quiz3);
    PrintHGrid();
    mydl();
    println!("");

}

fn mydl() {
    println!("=====================");
}
fn mysl() {
    println!("---------------------");
}
fn myinfo() {
    mydl();
    println!("IOT Batch 3 Assignment 6");
    println!("Question No. 01");
    println!("Faisal Shahzad");
    println!("PIAIC49775");
    mydl();
}