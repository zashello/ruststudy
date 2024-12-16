fn main() {
    println!("{} days a month",30); //可以用"{}",30的表达print出数字30，不能直接print!(30)
    println!("{0} wish {1} happy,{0} am happpy and {0} hope {1} too","i","you");//可以用{0}形式代替print！里需要的字符，{}里的数必须从0开始
    println!("{subject} {verb} {object}",verb="catch",object="me",subject="the dog");//可以使用命名参数
    println!("{} of {:b} people is foolish", 1, 2);//可以在{}内加上:与字母指定特殊格式
    println!("{number:<width$}{a}",number=1,width=6,a=1);//通过指定宽度，用</>,左/右对齐文本
    println!("{a}{number:>0width$}",number=1,width=6,a=1);//用0替代补齐中造成的空格
    #[allow(dead_code)]
    #[derive(Debug)]
    struct Stru(i32);
    println!("this struct `{:#?}` wont print", Stru(3));
    let pi = 3.1415926;
    println!("{} is rougthly 3.142",pi);
}
