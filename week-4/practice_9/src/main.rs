fn main() {
    let A:i32 = 10 ;
    let B:i32 = 20;
    
    println!("Value of A:{} ",A);
    println!("Value of B : {}",B);

    let mut result = A>B ;
    println!("A greater than B: {}", result);

    result = A<B ;
    println!("A lesser than B: {} ", result);

    result = A>=B;
    println!("A is greater than or equal to B: {}",result) ;
    
    
    result = A<=B;
    println!("A is less than or equal to B: {}", result) ;
    
    result = A==B ;
    println!("A is equalto B: {}",result) ;

    result = A!=B ;
    println!("A is not equal to B: {} ",result);
}
