fn main() {
    println!("Hello, world!");//a normal println macro!
    let mut x = 50;//declaration of the "mutable" variable x with value = 50
    println!("Before muting x : {}",x);//printing before muting x
    x=100;//muting x
    println!("After Muting x : {}",x);//printing the muted value of x
    another_function(x);//invoking function call and passing x
}

fn another_function(a:u8)
    //accepting the parameter as var "a" type unsigned 8 (0- 255 capacity)
{
println!("another function!(addition): {}",a+50);//function definition
}
/*here we define the function to add the value "50" to the existing(muted)value of x. This method
does not need the variable to be mutable as the value of the variable x  is passed as a
parameter, thus it would run even if x wasnt mutable.*/ 
