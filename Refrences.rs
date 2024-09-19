fn main(){
    let a = 'A';
    let b = 'B';
    let point = (1,2);
    let mut r: &char = &a;
    println!("r: {}", *r);
    r = &b;
    println!("r: {}", *r);
    //let c: i32= 5;
    //x_axis(&c);
    x_clusive(point);
}

//dangling refrences
/*fn x_axis(x: &i32) -> &(i32, i32) {
    let point = (*x, 0);
    return &point;
}*/

//Exclusive refrences
fn x_clusive(x: (i32, i32)) {
    let mut point = x;
    let x_coord = &mut point.0;
    *x_coord = 20;
    println!("point: {point:?}");
}