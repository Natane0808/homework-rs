mod test_mod1;

use test_mod1::test_mod2;

fn main() {
    //循环打印从’a’~’Z’ 之间的所有字符
    println!("循环打印从’a’~’Z’ 之间的所有字符");
    test_mod1::test1();
    println!("循环打印从’A’~’z’ 之间的所有字符");
    //循环打印从’A’~’z’ 之间的所有字符
    test_mod2::test2();
}
