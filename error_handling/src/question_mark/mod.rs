use std::fs;
use std::fs::File;
use std::io::{Error, Read};

pub struct Person {
    pub job: Option<Job>,
}

#[derive(Clone, Copy)]
pub struct Job {
    pub phone_number: Option<PhoneNumber>,
}

#[derive(Clone, Copy)]
pub struct PhoneNumber {
    pub area_code: Option<u8>,
    pub number: u32,
}

impl Person {
    pub fn get_phone_area_code(&self) -> Option<u8> {
        self.job?.phone_number?.area_code
    }
}

/// 好处：可以转换处理错误, 可控性最强
/// 坏处：处理繁琐些
pub fn read_file_match(filepath: &str) -> Result<String, Error> {
    let f = File::open(filepath);
    let mut f = match f {
        Ok(v) => v,
        Err(e) => return Err(e),
    };
    let mut content = String::new();
    match f.read_to_string(&mut content) {
        Ok(_) => return Ok(content),
        Err(e) => return Err(e),
    }
}

/// good
/// 好处：处理简单，类似java中的throws
/// 坏处1：不能转换错误，
/// 坏处2：只能在返回 Result 和 Option的函数中使用 ?
/// 在不返回 Result 或 Option 的函数中，当其函数体
/// 需要使用 match 方法来处理，而不能用 ? 将
/// 潜在的错误传播给代码调用方。
pub fn read_file_question_mark1(filepath: &str) -> Result<String, Error> {
    let mut f = File::open(filepath)?;
    let mut content = String::new();
    f.read_to_string(&mut content)?;
    Ok(content)
}

// very good
pub fn read_file_question_mark2(filepath: &str) -> Result<String, Error> {
    let mut content = String::new();
    File::open(filepath)?.read_to_string(&mut content)?;
    Ok(content)
}

// very very good rust库提供的最简单用法读取文件，利用fs提供的方法一行搞定
pub fn read_file_question_mark3(filepath: &str) -> Result<String, Error> {
    fs::read_to_string(filepath)
}

pub fn test_question_mark() {
    let p = Person {
        job: Some(Job {
            phone_number: Some(PhoneNumber {
                area_code: Some(61),
                number: 439222222,
            }),
        }),
    };
    println!("area_code:{}", p.get_phone_area_code().unwrap());

    let filepath = "error_handling/src/lib.rs";
    let content = read_file_match(filepath).unwrap();
    println!("content used match:\n{}", content);

    let content = read_file_question_mark1(filepath).unwrap();
    println!("content used question1:\n{}", content);
    let content = read_file_question_mark2(filepath).unwrap();
    println!("content used question2:\n{}", content);
    let content = read_file_question_mark3(filepath).unwrap();
    println!("content used fs:\n{}", content);
}
