fn main() {

    //Mut is use to redeclare 
    let mut x = 5;
    println!("The Value is x is {}",x);
    
    x = 6;
    println!("The Value is x is {}",x);

    const SUBSCRIBER_COUNT:u32 = 200;

    //Shadowing is creating a new variable with existing name
    
    let y:u32 = 5;
    println!("Th Value of y is {}",y);
    let y:&str = "six";
    println!("Th Value of y is {}",y);


    //Datatype Scaler and Compound
    
    //Scalar Datatype when there is single Value

    //Integer
    // let a:i32 = 98_45; //Decimal
    // let a:i32 = 0xff; //Hex
    // let a:i32 = 0o77; //Octal
    // let a:i32 = 0b111_0000; //Binary
    // let a:i32 = b'A'.into(); //u8 only
    // let a:u32 = 255; // if i give value greater than this it will wrap them means goes to negative line

    //Floating Point Number
    //Booleans
    //Character

    //Compound Datatype when there is multiple Value
    //Tuple
    // let tup:(&str,i32)  = ("Hello World in Rust",100_100);
    // let (channel,sub_count) = tup;
    // let sub_count = tup.1;

    // //Array
    // let error_code = [200,404,500];
    // let not_found = error_code[1];
    // let byte = [0; 8]; //Create an array with 8 value and all set to 0

    // Function in Rust 
    let ans = my_function(45,789);
    println!("The Sum is {}",ans);

    control_flow()

}


fn my_function(x:i32,y:i32)->i32{
    
    println!("The x value is {}",x);
    println!("The y value is {}",y);
    x+y

}


fn control_flow(){
    
    let number = 45;
    if number > 45 {
        println!("The Number is Greater than {}",number);
    }else if number <45 {
        println!("The Number is less than {}",number);
    }else{
        println!("The Number is Equal to {}",number);
    }

    //loop is the basic which break when we call break
    
    let mut number = 5;
    loop{
        number+=1;
        println!("{} ",number);
        if number==10 {
            break;
        }
    }
    
    //while loop 
    while number!=0 {
        number-=1;
        println!("{} ",number);
    }


    //For Loop
    let a = [10,20,30,40,50];
    for element in a.iter() {
        println!("The Value is: {}",element );
    }
    
    //For Print in particular range
    for number in 1..4 {
        println!("The Value is: {}",number );
    }

}