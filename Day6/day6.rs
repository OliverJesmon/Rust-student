fn main() {
    let gender:&str="Female";
    let typ:&str="Straight"; //didn't used type because it's a keyword
    if (gender=="Male") && (typ=="Straight") {

        println!("{typ} {gender}, huh? Find a woman");
    } else if (gender=="Female") && (typ=="Straight") {
        println!("{typ} {gender}, huh? Find a man");
    }

    else {
        println!("Find a partner, you deserve.");
    }
}
