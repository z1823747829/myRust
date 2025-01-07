pub(crate) fn main() {
    let x1=5;
    let x2:i64=6;
    let x3=7.0;
    println!("{x1}+{x2}+{x3}={}",x1 as f64 + x2 as f64 +x3);
    let x4=true;
    println!("x4={x4}");
    let x5='5';
    let x6="6";
    println!("x5={x5},x6={x6}");
    let x7=(11,22,33);
    println!("{},{},{}",x7.0,x7.1,x7.2);
    let x8=[8;10];
    println!("{:?}",x8);
}
