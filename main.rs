fn main() {
    let s1 = String::from("hello "); 
    let s2 = String::from("world!"); 
    println!("{}",concatenate_strings(&s1,&s2));
}

fn concatenate_strings(s1: &String, s2: &String) -> String{
    let mut my_string = String::from(""); 
    my_string.push_str(s1);
    my_string.push_str(s2);
    my_string
}
