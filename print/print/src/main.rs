fn main() {
    print!("{} days a month",30); //可以用"{}",30的表达print出数字30，不能直接print!(30)
    print!(" {0} wish {1} happy,{0} am happpy and {0} hope {1} too","i","you");//可以用{0}形式代替print！里需要的字符，{}里的数必须从0开始
    print!(" {subject} {verb} {object}",verb="catch",object="me",subject="the dog");//可以使用命名参数
}
