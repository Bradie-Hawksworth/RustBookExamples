fn main() {
     //Data type work from; https://doc.rust-lang.org/book/ch03-02-data-types.html
    //There are two different subsets of data types; compound and scalar.
    
    //This causes an issue with the compiler.
    //let guess: u32 = "42".parse().expect("Not a number!");
    
    //Being statically typed, RUST must know all types of variables at the point
    //of compilation.
    
    //Scalar Types
    //These types represent a single value. Scalar types are; integer, floating points, numbers, booleans and characters.
    
    //Integer Types
    ///////////////
    //Integers are numbers without a fractional component. Within chapter 2, we use the 
    //u32 data type (this is a 32 bit integer). The utilisation of u32 indicates that the value being stored
    //should be of the type of an unsigned integer. Signed variables start with an 'i' whereas unsigend variables start 
    //with a 'u'
    //Within rust there are many different types of integer; 8-bit, 16-bit, 32-bit, 64-bit, 128-bit and arch.
    //The arch being of size isize or usize depends on the architecture of the computer which the program is 
    //running on. 64-bit on a 64bit architecture and 32 on a 32bit architecture.
    //Integer literals can be wrote in any of the following formats; 
    //Decimal - 98_222
    //Hex - 0xff
    //Octal - 0o77
    //Binary - 0b1111_0000
    //Byte (u8 only) - b'A'
    
    //Rust's default integer type, resorts to i32. The primary situation where you would use isize or usize is when
    //indexing some sort of collection.
    
    //Floating point types
    ///////////////////////
    //There are two different types of floating point numbers.
    //f32 - 32 bit floating point value
    //f64 - 64 bit floating point value
    
    //Numeric operations
    //Rust supports the basic mathematical operations. This includes; addition, subtraction, multiplication, division, and remainder.
    //Integer division truncates towards zero to the nearest integer.
    
    //The following code shows how each of the numeric operations can be used in a let statement.
    //Addition
    let sum = 5 + 10;
    println!("The value of sum is {sum}");
    
    //Subtraction 
    let difference = 95.5 - 43.3;
    println!("The value of difference is {difference}");
    
    //Multiplication
    let product = 3 * 30;
    println!("The value of product is {product}");
    
    //Division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3;
    println!("The value of quotient is {quotient}");
    println!("The value of truncated is {truncated}");
    
    //Remainder
    let remainder = 43 % 5;
    println!("The value of remainder is {remainder}");
    
    //Boolean data types
    ////////////////////
    //Booleans only have two different values 'True' or 'False'
    //Booleans are one byte in size.
    let t = true;
    println!("The value of t is {t}");
    
    let f: bool = false; //This is the declaration of a boolean data type with the explicit type annotation.
    println!("The value of f is {f}");
    
    //The Character data type
    /////////////////////////
    //The char type is the languages most primitive alpahbetic type. 
    //The following are some declarations of the char type;
    let c = 'z';
    println!("The value of c is {c}");
    
    let z: char = 'Z';
    println!("The value of z is {z}"); //This has explicit type annotation in the declaration of this variable.
    
    let heart_eyed_cat = '😻';
    println!("The value of heart_eyed_cat is {heart_eyed_cat}");
    
    //Characters are created using single quotes, this is using character litterals rather than
    //using the string character which is the double quote symbols.
    //Rust's char variables are a 4 bytes and represents a unicode scalar value.
    //^ This means that the chars can represent more than just ASCII - Chinese, Japanese, and Korean characters; emoji; 
    //and zero-width spaces are all valid char values in Rust.
    //Unicode scalar values range everywhere from; U+0000 all the way to U+10FFFF
    //A concept of a char isn't a thing in Unicode. Human intuitions for what a character is may not match up with what a 
    //character is in rust. This is discussed later in chapter 8.
    
    //Compound Types
    ////////////////
    //Compound types can group multiple values in to type. Rust has two primitive compound types; Tuples and Arrays.
    
    //The tuple types
    //A tuple is a way of grouping together a number of values with a veriety of types into one compound type.
    //Tuples have a fixed size. Once they are declared they cannot grow or shrink.
    //A tuple data type can be created by writing a comma seperated list of values inside parentheses. Each position of the 
    //tuple has a type and the types within the positions do not have to be the same.
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("The value of tup is {:?}", tup); //This is the notation for concatenating a tuple into a string.
    
    //Variable tup is bound to a single compound element. To get the individual elements out of the tuple,
    //we can use pattern matching to destructure a tuple value like;
    let (i, j, k) = tup;
    println!("The value of i in the tup is {i}");
    println!("The value of j in the tup is {j}");
    println!("The value of k in the tup is {k}");
    //The program creates a tuple and binds it to the variable tup. It then uses a pattern with let to take tup
    //and turn it into three seperate variables. 
    //This is called destructuring as it takes the single variable and breaks it down into parts. The tuple can also
    //be accessed using the dot notation.
    
    //This notation had to be implemented due string: tuple index access isn't supported.
    let tup0 = tup.0;
    let tup1 = tup.1;
    let tup2 = tup.2;
    //The above code doesn't explicitly state what type of values will be stored in each of the declared variables.
    
    println!("The first value of tup in the tup is {tup0}");
    println!("The second value of tup in the tup is {tup1}");
    println!("The third value of tup in the tup is {tup2}");
    
    //Tuples without any values have special names. These tuples are known as units. 
    //This value and the corresponding type are written () they represent an empty value or an empty return type.
    //Expressions implicitly return the unit value if they dont return any other type.
    
    //The Array Type
    ////////////////
    //As all other programming languages, the values in arrays must all have the same data type. 
    //Arrays in rust also have a fixed length. Arrays have values stored in a comma seperated list between square brackets.
    let a = [1, 2, 3, 4, 5];
    println!("The values stored in array 'a' are: {:?}", a);
    
    //Arrays are useful when we want your data allocated on the stack. This is rather than the heap. The heap and
    //the stack are discussed late in chapter 4. Another reason for the use of an array is when the amount
    //of data to be stored is of a know value. Arrays aren't as flexible as vectors.
    //Vectors are provided by a standard library which allows the size of the vector to grow and shrink
    //as more data is added or taken away. Vectors are discussed in more detail within chapter 8.
    //Arrays are more useful when the number of elements aren't going to change during the running of the program.
    //An example of the use of an array would be when including a variable that stores the months... If the system is using
    //the gregorian callender it will have 2 months.
    
    let months = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sept", "Oct", "Nov", "Dec"];
    println!("The months are: {:?}",months); //This prints out the contents of the array but in the format of the array. See the output window
                             //For this representation.
    
    //Arrays can be created and instantiated by defining the data type and the number of elements within the array. 
    //The following notation implements this.
    let another_array: [i32; 5] = [1, 2, 3, 4, 5]; //Here the array is initialised with 5 elements of the type i32.
    println!("The values of 'another_array' are: {:?}", another_array);
    
    //Another way of initialising an array is of the following;
    let another_array_of_the_same_number = [3; 5]; //This  array will contain the value 3 5 times.
    println!("The values of 'another_array_of_the_same_number' are: {:?}", another_array_of_the_same_number);
    //The above declaration of an array is the same as writing let another_array_of_the_same_number = [3, 3, 3, 3, 3];
    
    //Accessing Array elements
    //////////////////////////
    //Arrays are single chunks (contiguous) blocks of memory of a known and fixed size. This can be allocated on the stack.
    //the elements of an array can be accessed using indexing as so;
    let array_fist_pos = another_array[0];
    let array_second_pos = another_array[1];
    
    println!("The first element of another_array is: {array_fist_pos}. The second element is {array_second_pos}");
    

}
