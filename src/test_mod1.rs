pub mod test_mod2;


// 循环打印从’a’~’Z’ 之间的所有字符 函数
pub fn test1() {
    for char in ('Z'..='a').rev() {
        println!("{}", char);
    }
}