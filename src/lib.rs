pub fn twofer(name: &str) -> String {
    if name == ""{
        return String::from ("One for you, one for me.");
    } else {
        return format! ("One for {}, one for me.", name);
    }


    //unimplemented!("how many for {name}")

}
