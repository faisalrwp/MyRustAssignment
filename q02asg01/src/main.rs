struct Myres <KFS>
{
    myroll:u16,
    myname:String,
    mygrade:KFS
}

impl<KFS: std::fmt::Debug> Myres<KFS>
{
    fn myprint(&self)
    {
        println!("Roll Number {:?} Mr./Miss {:?} Secured {:?} Grade.",&self.myroll, &self.myname, &self.mygrade);
    }
}
fn main() 
{
    let myst01 = Myres{myroll:49775,myname:"Faisal".to_string(),mygrade:5.0};
    let myst02 = Myres{myroll:11223,myname:"Nadia".to_string(),mygrade:"A+".to_string()};

    myst01.myprint();
    myst02.myprint();
}
