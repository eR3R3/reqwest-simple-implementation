trait Fight {
    fn rifle(&self){
        println!("rifle");
    }
}

trait Hide {
    fn hide(&self) {
        println!("hide");
    }
}

struct China {
    name: String,
}

struct America {
    name: String,
}

impl Fight for China {}
impl Fight for America {}
impl Hide for America {}

fn get_name(country: impl Fight){
    let a = country;

}

fn main() {
    let x: impl Fight = China{name: "x".to_string()};

}
