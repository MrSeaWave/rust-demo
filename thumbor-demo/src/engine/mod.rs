mod photon;
pub use photon::Photon;

use image::ImageOutputFormat;
use crate::pb::Spec;

// Engine trait: 未来可以添加更多的engine，主流程只需要替换engine
pub trait Engine{
    // 对 engine 按照specs 进行一系列有序的处理
    fn apply(&mut self,specs:&[Spec]);
    // 从 engine 生成目标图片，这里用的是self的引用
    fn generate(self,format:ImageOutputFormat)->Vec<u8>;
}

// SpecTransform 未来如果添加更多的spec，只需要实现它即可
pub trait SpecTransform<T>{
    // 对图片使用 op 做transform
    fn transform(&mut self,op:T);
}

