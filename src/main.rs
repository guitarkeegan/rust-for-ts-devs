
fn main(){
    fn my_funk(n: Option<i32>) -> Option<i32> {
        let n = n?;
        return Some(5*n);
    }

    println!("{:?}", my_funk(Some(55)));
    println!("{:?}", my_funk(None));

}