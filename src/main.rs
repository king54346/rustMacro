use std::collections::HashMap;
macro_rules! my_macro {
    ($num:expr) => {
        let x=$num;
        println!("{}",x);
    };
}

macro_rules! array_macro {
    ($($num:expr),*) => {
      $(
      let x=$num;
      println!("{}",x);
      )*
    };
}
macro_rules! map_macro {
    ($($key:expr=>$value:expr),*) => {
        {
          let mut hm = HashMap::new();
         $(hm.insert($key,$value);)*
        hm
        }
    };
}
trait ASBytes{
    fn as_bytes(&self)->Vec<u8>;
}
macro_rules! impl_AsBytes {
    ($($ty:ty),*) => {
        $(
      impl ASBytes for $ty{
          fn as_bytes(&self)->Vec<u8>{
              Vec::from(<$ty>::to_ne_bytes(*self))
          }
      }
      )*
    };
}
fn main() {
    let string = "HELLO".to_string();

    array_macro!(1,"2");
    let a =map_macro!("a"=>"b","c"=>"b");
    println!("{:?}",a);
    my_macro!("1");
    println!("Hello, world!");
    impl_AsBytes!(u16,u32);
    let u32_item=0x1024u32;
    println!("{:?}",u32_item.to_ne_bytes());
    println!("{:?}",u32_item.as_bytes());
    let bytes = <u32>::to_ne_bytes(u32_item);
    println!("{:?}",bytes);
}
