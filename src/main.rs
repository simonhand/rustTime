use std::{io};
use crate::garden::vegetables::Asparagus;

pub mod garden;


// fn main() {
//     // hellworld.
//     // println!("请猜下这个数字吧");
//     // let mut guess_str = String::new();
//     // io::stdin()
//     //     .read_line(&mut guess_str)
//     //     .expect("fail to readString");
//     // println!("you guess string is {}", guess_str);

//     let plant: Asparagus = Asparagus {};
//     // println!("I'm growing {:?}!", plant);
// }

// fn main() {
//   let mut count = 0;
//   'a: loop {
//       println!("count = {}", count);
//       let mut remaining = 10;

//       'b: loop {
//           println!("remaining = {}", remaining);
//           if remaining == 9 {
//               break;
//           }
//           if count == 2 {
//               break 'a;
//           }
//           remaining -= 1;
//       }

//       count += 1;
//   }
//   println!("End count = {}", count);
// }

// fn main() {
//   let mut counter = 0;

//   let result = loop {
//       counter += 1;

//       if counter == 7 {
//           break counter * 2;
//       }
//   };
//   println!("The result is {}", result);

//   for number in (1..=5).rev() {
//     println!("number is {number}")
//   }
// }

// 所有权
// fn main() {
//     let mut s: String = String::from("hello");
//     // s.push('a');
//     s.push_str(", Rust");
//     println!("{}", s);
// }

// 所有权
// fn main() {
//   // let mut s: String = String::from("hello");
//   // let mut b = s;
//   // s.push_str("bar");
//   let s1 = String::from("hello");
//   let s2 = s1;

//   println!("{}", s2);
// }

// // 所有权克隆
// fn main() {
//     let s1 = "hello";
//     let s2 = s1.clone();
//     print!("s1: {}, s2: {}", s1, s2)
// }

// 所有权的关系流转
// fn main() {
//   let num = 8;
//   display_num(num);
//   let mut s= String::from("程璐");
//   s = display_str(s);
//   println!("s: {}", s);
//   println!("give {}", give_a_string())
// }

// fn display_num(_num: i32) {
//     println!("num {}", _num);
// }

// fn display_str(str: String) -> String {
//     println!("string {}", str);
//     str
// }

// fn give_a_string() -> String {
//   String::from("晨梦")
// }

// 引用&借用
// fn main() {
//     let s1 = String::from("你好");
//     let (len, newStr) = calc_len(s1);
//     println!("{}`s len is {}", newStr, len);
//     println!("s1 is {}, {}", newStr, calc_len_ref(&newStr))
// }

// fn calc_len(str: String) -> (usize, String) {
//     (str.len(), str)
// }
 
// fn calc_len_ref(str: &String) -> usize {
//   println!("str is {}", str);
//   str.len()
// }

// 可变引用
// fn main() {
//     let mut a = String::from("hello");
//     let r = &mut a;
//     changeValue(r);
//     println!("{}", a)
// }
// fn changeValue(r: &mut String) {
//     r.push_str(", chenlu");
// }

// 悬垂引用
// fn main() {
//     let r = give_ref();
// }

// fn give_ref() -> &String {
//   let s = String::from("你好");
// 这里因为s变量提前释放了所以s的引用也就无效了
//   &s
// }

// fn give_ref() -> String {
//     let s = String::from("你好");
//     s
// }

// fn main() {
//     // let a = String::from("hello");
//     // let b = first_word(&a);
//     // let slice1 = &a[3..b];
//     // let slice1 = &a[1..];
//     // println!("slice1, {}", slice1);
//     // println!("{}", b)
//     let mut s = String::from("hello world");

//     let word = first_word(&s);
    
//     println!("the first word is: {}", word);

//     let a = [1, 2, 3, 4, 5];

//     let slice = &a[1..3];

//     assert_eq!(slice, &[2, 3]);

//     println!("assert_eq!(slice, &[2, 3]), {:?}", assert_eq!(slice, &[2, 4]));

//     s.clear(); // 错误！

// }

// fn first_word(s: &String) -> &str {
//   let bytes = s.as_bytes();

//   println!("bytes {:?}", bytes);
//   println!("bytes {:?}", bytes.iter());

//   // Some(())

//   for (i, &item) in bytes.iter().enumerate() {
//       if item == b' ' {
//           return &s[0..i];
//       }
//   }
//   return  &s[..];
//   // let newlist = bytes.iter().map(|i| i + 1);
//   // println!("{:?}", newlist);
//   // s.len()
// }

// 结构体
// struct User {
//   name: String,
//   age: u64,
// }
// fn main() {
//     let user1 = User {
//       name: String::from("陈璐"),
//       age: 12,
//     };
//     // println!("{:?}", user1);
// }

fn main() {
  let width1 = 30;
  let height1 = 50;

  println!(
      "The area of the rectangle is {} square pixels.",
      area(width1, height1)
  );
}

fn area(width: u32, height: u32) -> u32 {
  width * height
}
