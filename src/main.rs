// use thiserror::Error;
// use url::Url;

// #[derive(Error, Debug)]
// pub enum MyError {
//     #[error("I/O 错误: {0}")]
//     Io(#[from] std::io::Error),
//
//     #[error("URL 解析错误: {0}")]
//     UrlParse(#[from] url::ParseError),
//
//     #[error("自定义错误: {0}")]
//     Custom(String),
// }

struct MyStruct {
    value: String,
}

impl MyStruct {
    fn change_value(&mut self) {
        self.value = self.value.to_uppercase();
    }
}

fn main() {
    let mut s = MyStruct { value: String::from("hello") };
    s.change_value();
}


// trait Fight {
//     fn rifle(&self){
//         println!("rifle");
//     }
// }
//
// trait Hide {
//     fn hide(&self) {
//         println!("hide");
//     }
// }
//
// struct China {
//     name: String,
// }
//
// struct America {
//     name: String,
// }
//
// impl Fight for China {}
// impl Fight for America {}
// impl Hide for America {}
//
// fn get_name(country: impl Fight){
//     let a = country;
//
// }