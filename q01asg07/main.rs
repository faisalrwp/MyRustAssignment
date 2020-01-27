mod lib;
fn main()
{
    let myname=lib::input::getstring("Enter Your Name :".to_string());
    println!("{}",myname);
}