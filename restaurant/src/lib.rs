mod front_of_house {
  pub mod hosting {
      fn add_to_waitlist() {
        let a: u8 = 1;
        println!("等餐中{a}");
      }

      fn seat_at_table() {}
  }

  mod serving {
      fn take_order() {}

      fn serve_order() {}

      fn take_payment() {}
  }
}

pub fn eat_at_restaurant() {
  // 绝对路径
  crate::front_of_house::hosting::add_to_waitlist();
  // 相对路径
  front_of_house::hosting::add_to_waitlist();
}