fn main(){
    // shallow copy -> move, s1 lost its ownership
    let s1 = String::from("fuck");
    let s2 = s1;
    println!("s2:{s2}");

    // deep copy
    let s1 = String::from("nice");
    let s2 = s1.clone();
    println!("s1:{s1},s2:{s2}");

    // the "takes_and_gives_back" function has taken the
    // ownership of s2 by moving(arg passing), s2 lost
    // its ownership
    let s3 = takes_and_gives_back(s2);
    println!("s3:{s3}");
}

fn takes_and_gives_back(a_string:String)->String{
    a_string
}