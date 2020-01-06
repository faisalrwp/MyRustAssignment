fn main() {
    println!("===========================================");
    println!("Welcome To Assignment 2 Question 2 Solution");
    println!("===========================================");
    let mytup=("Mango", 1.0, 500);
    let (fruitname, boxsize, priceperbox) = mytup;
    let purchasedquantity:i32 = 10; 
    println!("ITEM = {} | BOX SIZE = {}KG | PRICE PER BOX = {}", fruitname,boxsize,priceperbox);
    println!("-------------------------------------------");
    println!("You Purchased {} Boxes Of {}", purchasedquantity, fruitname);
    println!("Price Per Box is PKR.{}/=", priceperbox);
    println!("Calculation : {} x {} = {}", purchasedquantity, priceperbox, purchasedquantity*priceperbox);
    println!("Please Pay {} Amount On The Counter", purchasedquantity*priceperbox);
    println!("_*_ Thanks For Your Visit To Our Store _*_");
    println!("-------------------------------------------");
}
