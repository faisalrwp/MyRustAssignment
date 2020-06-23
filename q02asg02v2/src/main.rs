use std::io;
use rand::Rng;
use std::collections::HashMap;

fn main() {
    myUH();
    myOutput("\nGame Limits : Maximum Players 255 : Minimum Players 01\n".to_string()   );
    myUH();
    let myMaxPlayer = myInput("Enter No. Of Players : ".to_string());
    // let myMaxPlayer:usize = myMaxPlayer.trim().parse().expect("Error ...");
    let myMaxPlayer:usize = match myMaxPlayer.trim().parse::<usize>() {
        Ok(x) => x,
        Err(_) => 0,
    };
    if myMaxPlayer == 0 || myMaxPlayer > 255
    {
        myOutput("Enter Valid Values For No. Of Players (1-255)".to_string());
        std::process::exit(0);
    }
    println!("{} Players will play this game", myMaxPlayer) ;

    //let mut myPlayer = vec!["-".to_string(); myMaxPlayer];
    let mut myPlayer = HashMap::new();
    //let mut myPlayer = ["-"; 255];
    let mut myScore = vec![0; myMaxPlayer];
    
    for i in 0..myMaxPlayer
    {
        myPlayer.insert(i, myInput("Player Name".to_string()));
    }


    let mut myTurn:u8 = 1;
    let mut myVal:u8=0;
    loop
    {
        myOutput("".to_string());
        myDL();
        println!("Turn {}", myTurn);
        mySL();
        for i in 0..myMaxPlayer
        {
            let myName=myPlayer.get(&i).unwrap().trim().to_string();
            myVal = myRandom();
            print!("[{}]-",myVal);
            if myVal==6
            {
                let myVal2 = myRandom();
                print!("[{}]-",myVal2);
                myVal += myVal2;
                if myVal2==6
                {
                    let myVal3 = myRandom();
                    print!("[{}]-",myVal3);
                    if myVal3 == 6
                    {
                        myVal = 0;
                    }
                    else
                    {
                        myVal += myVal3;
                    }
                }
            }

            if (myScore[i] + myVal) < 100
            {
                myScore[i] +=  myVal;
            }
            else if (myScore[i] + myVal) == 100
            {
                println!("Player {} Dice Value Is {} And Total Score is {}", myName.to_string(), myVal, myScore[i]);
                myDL();
                myUH();
                println!("\n\nPlayer {} Wins This Game By Rolling {}.. Congratulations\n\n", myName.to_string(),myVal);
                myUH();
                myDL();
                std::process::exit(0);
            }
            println!("Player {} Dice Value Is {} And Total Score is {}", myName.to_string(), myVal, myScore[i]);
            for j in 0..myMaxPlayer {
                if myScore[j] == myScore[i] 
                {
                    if j != i
                    {
                        let myKickedName=myPlayer.get(&j).unwrap().trim().to_string();
                        myUH();
                        println!("\n{} Kicked {} At The Score Of {}\n",myName.to_string(), myKickedName.to_string(), myScore[i]);
                        myUH();
                        myScore[j] = 0;
                    }
                    
                }
            }
        }
        
        myTurn+=1;
        if myTurn==255
        {
            myTurn=0;
        }
    }

}
fn myOutput(myIn:String)
{
    println!("{}",myIn);
}
fn myInput(myIn:String) -> String
{
    println!("{}", myIn);
    let mut myOut=String::new();
    io::stdin().read_line(&mut myOut).expect("Failed to Get Input");
    return  myOut;
}
fn myRandom()->u8
{
    let mySecret = rand::thread_rng().gen_range(1, 7);
    return mySecret;
}
fn myDice()->u8
{
    return myRandom();
}

fn myDL()
{
    myOutput("====================".to_string());
}
fn mySL()
{
    myOutput("--------------------".to_string());
}
fn myUH()
{
    myOutput("<><><><><><><><><><>".to_string());
}
