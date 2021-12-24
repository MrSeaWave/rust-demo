// 定义用到的字符类型
#[derive(Clone)]
pub struct ChristmasString {
  pub apple: String,
  pub star: String,
  pub bell: String,
  pub tree: String,
  pub door: String,
  pub gift: String,
  pub space: String,
}

impl ChristmasString {
  pub fn new(flag: bool) -> Self {
    let mut apple = "🍎".to_owned();
    let mut star = "🌟".to_owned();
    let mut bell = "🔔".to_owned();
    let mut tree = "🎄".to_owned();
    let mut door = "🚪".to_owned();
    let mut gift = "🎁".to_owned();
    let mut space = "  ".to_owned();
    if !flag {
      apple = "A".to_owned();
      star = "*".to_owned();
      bell = "O".to_owned();
      tree = "X".to_owned();
      door = "I".to_owned();
      gift = "G".to_owned();
      space = " ".to_owned();
    }
    return Self {
      apple,
      star,
      bell,
      tree,
      door,
      gift,
      space,
    };
  }
}
