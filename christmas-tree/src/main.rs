mod config;

use crate::config::ChristmasString;
use clap::{
  crate_authors, crate_description, crate_name, crate_version, value_t_or_exit, App, Arg,
};
use rand::{thread_rng, Rng};

fn main() {
  // 命令行参数解析
  let matches = App::new(crate_name!())
    .version(crate_version!())
    .author(crate_authors!())
    .about(crate_description!())
    .arg(
      Arg::with_name("floor")
        .long("floor")
        .short("f")
        .takes_value(true)
        .required(true)
        .default_value("4")
        .help("floor of the tree"),
    )
    //   TODO 以下参数被接受使用
    .arg(
      Arg::with_name("christmasPendant")
        .short("c")
        .long("christmasPendant")
        .takes_value(true)
        .default_value("true")
        .help("Christmas pendant"),
    )
    .arg(
      Arg::with_name("useEmoji")
        .short("u")
        .long("useEmoji")
        .takes_value(true)
        .default_value("true")
        .help("Use Emoji 🌟"),
    )
    // .arg(
    //   Arg::with_name("withStar")
    //     .short("w")
    //     .long("withStar")
    //     .help("Adds a little start"),
    // )
    .get_matches();

  // println!("useEmoji: {}", matches.is_present("useEmoji"));
  // println!(
  //   "useEmoji: matches.value_of() {}",
  //   value_t_or_exit!(matches.value_of("useEmoji"), bool)
  // );
  let christmas_str = ChristmasString::new(value_t_or_exit!(matches.value_of("useEmoji"), bool));
  let tree = ChristmasTree {
    floor: value_t_or_exit!(matches.value_of("floor"), u32),
    // has_star: matches.is_present("withStar"),
    christmas_str,
  };

  tree.draw()
}

// 常量定义，以下常量可以被 useEmoji 控制 ---> ChristmasString
// const APPLE: &str = "🍎";
// const STAR: &str = "🌟";
// const BELL: &str = "🔔";
// const TREE: &str = "🎄";
// const DOOR: &str = "🚪";
// const GIFT: &str = "🎁";
// const SPACE: &str = "  ";

// const APPLE: &str = "a";
// const STAR: &str = "s";
// const BELL: &str = "x";
// const TREE: &str = "l";
// const DOOR: &str = "m";
// const GIFT: &str = "G";
// const SPACE: &str = " ";

struct ChristmasTree {
  floor: u32,
  christmas_str: ChristmasString,
}

impl ChristmasTree {
  fn draw(&self) {
    self.draw_tree();
  }

  // 生成一颗圣诞树
  fn draw_tree(&self) {
    let floor_num = self.floor;
    let bottom_amount: u32 = ChristmasTree::get_line_amount(floor_num, floor_num + 4);
    // 一层，一行的生成
    for f in 0..floor_num {
      for l in 0..f + 5 {
        // println!(" f: {},l: {}", f, l);
        let line_amount = ChristmasTree::get_line_amount(f, l);
        // println!("line_amount :{}", line_amount);

        let mut str_list: Vec<String> = Vec::new();

        // 倒叙循环
        for _i in (0..(bottom_amount - line_amount) / 2 - 1).rev() {
          // 输出空格
          // println!("i :{}", i);
          str_list.push(self.christmas_str.space.clone());
        }

        for _i in 0..line_amount {
          // 输出特定字符 苹果 or 树 or 星星
          let r = self.rand_apple_tree();
          str_list.push(r)
        }

        println!("{}", str_list.join(""))
      }
    }

    // 居中、生成圣诞树根
    for f in 0..floor_num {
      let line_amount = floor_num + (floor_num + 1) % 2;

      let mut str_list: Vec<String> = Vec::new();
      for _i in (0..(bottom_amount - line_amount) / 2 - 1).rev() {
        str_list.push(self.christmas_str.space.clone());
      }
      for _i in 0..line_amount {
        str_list.push(self.christmas_str.door.clone())
      }
      // 在圣诞树下放点礼物
      str_list.push(self.gen_gift(f));
      println!("{}", str_list.join(""))
    }
  }

  //获取第f层的第line行的圣诞树数量
  fn get_line_amount(f: u32, line: u32) -> u32 {
    return 1 + line * 2 + f * 4 + (f / 2 * 2) * ((f + 1) / 2);
  }

  // 随机生成树 or 挂件
  fn rand_apple_tree(&self) -> String {
    let mut rng = thread_rng();
    let n: u32 = rng.gen_range(0..100);
    if n < 1 {
      return self.christmas_str.star.clone();
    } else if n < 2 {
      return self.christmas_str.bell.clone();
    } else if n < 10 {
      return self.christmas_str.apple.clone();
    } else {
      return self.christmas_str.tree.clone();
    }
  }

  // 生成礼物
  fn gen_gift(&self, f: u32) -> String {
    if f == 0 {
      return self.christmas_str.space.clone();
    }
    // 第一层祝福语
    if f == 1 {
      return [
        self.christmas_str.space.clone(),
        "Merry Christmas!".to_owned(),
      ]
      .join("");
    }
    let mut str_list: Vec<String> = Vec::new();

    str_list.push(self.christmas_str.space.clone());

    // 第二层开始放礼物
    for _f in 1..f {
      str_list.push(self.christmas_str.gift.clone())
    }

    return str_list.join("");
  }
}
