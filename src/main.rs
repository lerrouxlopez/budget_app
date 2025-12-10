fn main() {

    let name = get_name("Ikit");
    greet(name);


}

fn greet(mut name: &str) {
    if name != "Eddie"{
        name = "not Eddie"
    }
    println!("I am {} , nice to meet you", name);
}


fn get_name(name: &str)  -> &str {
    if name != "Eddie" {
        return "not Eddie";
    }
    return name;
}