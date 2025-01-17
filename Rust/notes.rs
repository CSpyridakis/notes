// Single line comment
/*
    Multi-line commnents
*/

// Notes based on:
// https://doc.rust-lang.org/rust-by-example/hello/print.html
// Formatted print

// https://doc.rust-lang.org/rust-by-example/hello/print.html
// 

// https://learning-rust.github.io/docs/variable-bindings-constants-and-statics/
// Contol Flows

// https://www.programiz.com/rust/hello-world
// Rust Hello World Program


macro_rules! pvar {
    { $( $var:expr ), * } => {
        {
            fn type_of<T>(_: &T) -> &'static str {
                std::any::type_name::<T>()
            }

            $(
                println!("\t- [Name]: {:<25} | [Type]: {:<30} | [Size]: {:<4} bytes | [Value]: {:?}", stringify!($var), type_of(&$var), st:mem::size_of_val(&$var), &$var);
            )*
        }
    };
}

// ---------------------------------------------------------------------------
// ---------------------------------------------------------------------------
// ---------------------------------------------------------------------------

/// Generate library docs for the following item.
fn library_docs_following(){
    println!("\n======================================================================");
    println!("1.1. library_docs_following");
}

fn library_docs_envlosing(){
    //! Generate library docs for the enclosing item.
    println!("\n======================================================================");
    println!("1.1. library_docs_envlosing");
}

// ---------------------------------------------------------------------------
// ---------------------------------------------------------------------------
// ---------------------------------------------------------------------------

fn formatted_print(){
    println!("\n======================================================================");
    println!("* FORMATING PRINTS: ");

    // Printing is handled by a series of macros defined in std::fmt some of which include:
    //      format!: write formatted text to String
    let print_statement = format!("{} {}", "This is a print", "statement");
    pvar!(print_statement);

    //      print!: same as format! but the text is printed to the console (io::stdout).
    //      println!: same as print! but a newline is appended.
    let argu=16;
    
    println!("{print_statement} arg0: {0} name is: {name} and its value in hex is: 0x{argument:x} and in binary: 0b{argument:b}", 
        argu, 
        name="argument",
        argument=argu
    );

    //      eprint!: same as print! but the text is printed to the standard error (io::stderr).
    //      eprintln!: same as eprint! but a newline is appended.

    eprintln!("This is a print statement in stderr");

    // Only types that implement fmt::Display can be formatted with `{}`. User-
    // defined types do not implement fmt::Display by default.
    // std::fmt contains many traits which govern the display of text. 
    // The base form of two important ones are listed below:
    //      #[derive(Debug)]
    //      fmt::Debug: Uses the {:?} marker. Format text for debugging purposes.
    //      (All std library types; i.e. primitive, are automatically printable with {:?} too)
    //      Pretty print: {:#?} to display types as they are

    // #[derive(Debug)]
    // struct Person<'a> {
    //     _name: &'a str,
    //     _age: u8
    // }
    // let _name = "Peter";
    // let _age = 27;
    // let peter = Person { _name, _age };

    // // Pretty print
    // println!("Pretty print struct with: {{:#?}}:");
    // println!("{:#?}", peter);

    //      fmt::Display: Uses the {} marker. Format text in a more elegant, user friendly fashion.
    //      fmt::Debug hardly looks compact and clean, so it is often advantageous to customize the 
    //      output appearance. This is done by manually implementing fmt::Display, which uses the {} 
    //      print marker. Implementing it looks like this:

    // Print structs
    // Define a structure for which `fmt::Display` will be implemented. This is
    // a tuple struct named `Point2D` that contains two `f64`.
    #[derive(Debug)]
    struct Point2D {
        x: f64,
        y: f64,
    }

    // To use the `{}` marker, the trait `fmt::Display` must be implemented manually for the type.
    impl std::fmt::Display for Point2D {
        // This trait requires `fmt` with this exact signature.
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            // Write strictly the first element into the supplied output
            // stream: `f`. Returns `fmt::Result` which indicates whether the
            // operation succeeded or failed. Note that `write!` uses syntax which
            // is very similar to `println!`.
            // write!(f, "x: {}, y: {}", self.x, self.y) // If it was a tupple, we could access if like self.0, self.1

            // Rust provides the ? operator, which can be combined with write! for complex writes
            // Try `write!` to see if it errors. If it errors, return the error. Otherwise continue.
            // As an example (This is the equivalent solution from the above):
            // Use this trick for more complex structures that need to break in steps the print sequence
            write!(f, "x: {}", self.x)?;
            write!(f, ", y: {}", self.y)
        }
    }

    let point = Point2D { x: 3.3, y: 7.2 };
    pvar!(point);
    println!("\n- Display        (print utilizing {{}})    => \n{}", point);
    println!("\n- Display        (print utilizing {{}} & .to_string())    => \n{}", point.to_string());
    println!("\n- Debug          (print utilizing {{:?}})  => \n{:?}", point);
    println!("\n- Pretty Debug   (print utilizing {{:#?}}) => \n{:#?}", point);

}

// ---------------------------------------------------------------------------
// ---------------------------------------------------------------------------
// ---------------------------------------------------------------------------

fn variable_binding(){
    // Primitive Data Types:
    // 
    // > Scalar Types
    //  - bool:                 either true or false
    //  - char:                 Because of Unicode support, char is not a single byte, but four(32 bits / 4 bytes)
    //  - Signed integers:      i8, i16, i32, i64, i128 and isize (pointer size)
    //  - Unsigned integers:    u8, u16, u32, u64, u128 and usize (pointer size)
    //  - Floating point:       f32, f64
    //  - The unit type ():     whose only possible value is an empty tuple: ()
    // 
    // > Compound Types
    //  - Array:                let array: [T; length] = [0; length];               like [1, 2, 3]
    //  - Tuple:                let tuple: (i32, char) = (1, 'a');                  like (1, true)
    //  - Slice                 let slice: &[T] = &parent_array[start..end];        like [1, 2]
    //  - str
    //  - Function

    println!("\n======================================================================");
    println!("* VARIABLES BINDING: ");

    // ------------------------------------------------------------------------
    let true_var = true;                // Declaration + assignment; without data type (type annotation)
    let false_var: bool = false;        // Declaration + assignment; with data type (type annotated)
    pvar!(true_var, false_var);

    // ------------------------------------------------------------------------
    let a;                              // Declaration; without data type (type annotation)
    a = 1_000_000;                      // Assignment (Underscores can be inserted in numeric literals to improve readability)

    let b: u8;                          // Declaration; with data type 
    b = 2u8;                            // Assignment (use the u8 suffix to indicate that the literal is an unsigned 8-bit integer)
    pvar!(a, b);


    // ------------------------------------------------------------------------
    // variables are immutable by default, to make them mutable
    let mut _mut_var:u16 = 3;                       // Assign a literal to a mutable variable
    _mut_var = 4;                                   // casting to smaller number 
    pvar!(_mut_var);

    // ------------------------------------------------------------------------
    let suffix_literal      = 3u16;                                 // Suffixed literals, their types are known at initialization
    let unsuffix_literal    = 0x23231;                              // Unsuffixed literals, their types depend on how they are used
    let casted_literal:u16  = (0x12345678 % 0xFFFF) as u16;         // casting to smaller number 
    pvar!(suffix_literal, unsuffix_literal, casted_literal);

    // ------------------------------------------------------------------------
    // the left-hand side of a let expression is a “pattern”
    let (scientific_annot_1, scientific_annot_2) = (1e6, 7.6e-4);         // Rust also supports scientific E-notation
    pvar!(scientific_annot_1, scientific_annot_2);

    // ------------------------------------------------------------------------
    // Function like assignements
    let f:i64 = { 6 + 1 };  // f = 7
    let g:u64 = {
        let h = 7;
        let i = 1;

        h + i
    }; // g = 8
    pvar!(f, g);

    // ------------------------------------------------------------------------
    // Constants - An unchangeable value (the common case)
    //      Constants should be upper case.  
    //      Their values are not allowed to change. 
    //      They live for the entire lifetime of a program but has no fixed address in the memory.
    const CONSTANT_VAR: u128 = 9;  
    pvar!(CONSTANT_VAR);
    
    // ------------------------------------------------------------------------
    // Statics - A possibly mutable variable with 'static lifetime
    //      Statics should be upper case.  
    //      The static keyword is used to define a “global variable” type facility. 
    //      There is only one instance for each value, and it’s at a fixed location in memory.
    static STATIC_VAR: i128 = 10;
    pvar!(STATIC_VAR);

    // Aliasing
    // The type statement can be used to give a new name to an existing type
    type CamelCaseTypeAliased = u64;
    let type_aliased: CamelCaseTypeAliased = 5 ;
    pvar!(type_aliased);
}

fn strings(){
    // ------------------------------------------------------------------------
    // STRINGS & str
    // The String type is a heap-allocated string. 
    // This string is growable and is also guaranteed to be UTF-8. 
    // In general, you should use String when you need ownership, and &str when you just need to borrow a string.
    // The String data type is a UTF-8 encoded vector. But you can not index into a String because of encoding.

    println!("\n======================================================================");
    println!("* STRINGS: ");

    println!("1. Conversions: ");
    println!("\n\tA.Str -> String");
    let s1: &str = "str";                       // &str
    pvar!(s1);
    println!("\tlet s1 = s1.to_string();");
    let s1 = s1.to_string();                    // &str -> String
    pvar!(s1);

    println!("\n\tB.String -> str");
    let s2 = String::from("String");            // String
    pvar!(s2);
    println!("\tlet s2 = s2.as_str();");
    let s2 = s2.as_str();                       // String -> &str
    pvar!(s2);

    println!("\n\tC.String -> Integer");
    let s3: String = String::from("10");
    pvar!(s3);
    let s3:i32 = s3.parse().expect("Please enter a valid number.");
    println!("\tlet s3:i32 = s3.parse().expect(\"Please enter a valid number.\");");
    pvar!(s3);

    println!("\n\tD.Integer -> String");
    let s4: i32 = 50;
    pvar!(s4);
    let s4 = s4.to_string();
    println!("\tlet s4 = s4.to_string();");
    pvar!(s4);

    // ------------------------------------------------------------------------
    println!("\n---------------------------------------\n");
    println!("2. Concatenations: ");

    // All bellow codes return `String`; something
    let (s1, s2) = ("some", "thing");           // both &str
    println!("let (s1, s2) = (\"some\", \"thing\");           // both &str");
    
    let s = String::from(s1) + s2;              // String + &str
    println!("\n\tA. let s = String::from(s1) + s2;      // String + &str");
    pvar!(s);

    let mut s = String::from(s1);               // String
    s.push_str(s2); // + &str
    println!("\n\tB.  let mut s = String::from(s1);      // String");
    println!("\ts.push_str(s2);                     // + &str");
    pvar!(s);

    let s = format!("{}{}", s1, s2); // &str/String + &str/String
    println!("\n\tC. let s = format!(\"{{ }} {{ }}\", s1, s2);    // &str/String + &str/String");
    pvar!(s);

    let s = [s1, s2].concat(); // &str or String array
    println!("\n\tD. let s = [s1, s2].concat();          // &str or String array");
    pvar!(s);
}

fn var_shadowing(){
    // Sometimes, while dealing with data, initially we get them in one unit but 
    // need to transform them into another unit for further processing. In this 
    // situation, instead of using different variable names, Rust allows us to 
    // redeclare the same variable with a different data type and/ or with a 
    // different mutability setting. We call this Shadowing.

    println!("\n======================================================================");
    println!("* VARIABLE SHADOWING: (Redeclare the same variable with a different data type)");

    let x: f64 = -20.48; // float
    println!("let x: f64 = -20.48; // float");

    let x: i64 = x.floor() as i64; // int
    println!("let x: i64 = x.floor() as i64; // int");
    println!("x = {}", x); // -21
}

// ---------------------------------------------------------------------------
// ---------------------------------------------------------------------------
// ---------------------------------------------------------------------------


fn plus_one(a: i32) -> i32 {
    a + 1
}

fn function_pointers(){
    println!("\n======================================================================");
    println!("* FUNCTION POINTERS: ");

    // ------------------------------------------------
    // Function pointers, Usage as a Data Type
    // Call function without type declaration.
    let p1 = plus_one;
    println!("  - let p1 = plus_one;              // Call function without type declaration");
    println!("    p1(5): {}", p1(5));

    //Call function With type declarations.
    let p1: fn(i32) -> i32 = plus_one;
    println!("  - let p1: fn(i32) -> i32 = plus_one;  // Call function With type declarations");
    println!("    p1(5): {}", p1(5));
}

// -----------------------

fn function_closures(){
    // Also known as anonymous functions or lambda functions.
    // The data types of arguments and returns are optional

    // Other characteristics of closures include:
    //  - using || instead of () around input variables.
    //  - optional body delimitation {} for a single line expression (mandatory otherwise).
    //  - the ability to capture the outer environment variables.
    //  - Using move before vertical pipes forces closure to take ownership of captured variables: let closure = move || {}

    // - Fn: Represents closures that can be called multiple times without mutating their environment.



    // - FnMut: Represents closures that can mutate their environment when called.
    //      fn call_fnmut<F>(mut f: F)
    //      where
    //          F: FnMut(),
    //      {
    //          f();
    //          f(); // FnMut can be called multiple times
    //      }

    // - FnOnce: Represents closures that can be called at least once but might consume variables 
    //          they capture, meaning they might not be callable more than once.
    //      fn call_fnonce<F>(f: F)
    //      where
    //          F: FnOnce(),
    //      {
    //          f(); // FnOnce can be called at least once
    //      }

    println!("\n======================================================================");
    println!("* FUNCTION CLOSURES: (Also known as anonymous functions or lambda functions)");

    // ---------------------------------------------------------------------------------------
    // With optional type declarations of input and return types
    println!("\n1. With optional type declarations of input and return types");
    // ---------------------------------------------------------------------------------------
    let square = |i: i32| -> i32 { // Input parameters are passed inside | | and expression body is wrapped within { }
        i * i 
    };
    println!("\tlet square = |i: i32| -> i32 {{ i * i }};           // Input parameters are passed inside | | and expression body is wrapped within {{ }}");
    println!("\tsquare(2): {}", square(2));

    // ---------------------------------------------------------------------------------------
    // Without type declarations of input and return types
    println!("\n2. Without type declarations of input and return types");
    // ---------------------------------------------------------------------------------------
    let square = |i| i * i; // { } are optional for single-lined closures
    println!("\tlet square = |i| i * i;                             //  {{ }} are optional for single-lined closures");
    println!("\tsquare(2): {}", square(2));

    // ---------------------------------------------------------------------------------------
    // With optional type declarations; Creating and calling together
    println!("\n3. With optional type declarations; Creating and calling together");
    // ---------------------------------------------------------------------------------------
    let x_square = |i: i32| -> i32 { i * i }(2); // { } are mandatory while creating and calling same time.
    println!("\tlet x_square = |i: i32| -> i32 {{ i * i }}(2);      //  {{ }} are mandatory while creating and calling same time");
    println!("\tx_square: {}", x_square);

    // ---------------------------------------------------------------------------------------
    // Without optional type declarations; Creating and calling together
    println!("\n4. Without optional type declarations; Creating and calling together");
    // ---------------------------------------------------------------------------------------
    let x_square = |i| -> i32 { i * i }(2);                                             
    println!("\tlet x_square = |i| -> i32 {{ i * i }}(2);             // The return type is mandatory");
    println!("\tx_square: {}", x_square);

}

// -----------------------

fn functions(){
    // Functions are declared using the fn keyword. Its arguments are type 
    // annotated, just like variables, and, if the function returns a value, 
    // the return type must be specified after an arrow ->.

    // The final expression in the function will be used as return value. 
    // Alternatively, the return statement can be used to return a value 
    // earlier from within the function, even from inside loops or if statements.
    // E.g. : fn is_divisible_by(lhs: u32, rhs: u32) -> bool {}

    // Functions that "don't" return a value, actually return the unit type `()`
    // E.g. : fn fizzbuzz(n: u32) -> () {}

    // When a function returns `()`, the return type can be omitted from the signature
    // E.g. : fn fizzbuzz_to(n: u32) {}

    function_pointers();    
    function_closures();
}

fn macros(){
    println!("\n======================================================================");
    println!("* MACROS)");

    // [MACRO SYNTAX]
    // 
    // macro_rules! macro_name {
    //    ($designator_name:designator) => {                         
    //        // BODY
    //    };
    //    ($designator_name:designator) => {                         
    //        // BODY
    //    };
    // }
    // 
    // Where ($designator_name:designator) some of the available designators:
    // -    ($X:expr)       : is used for expressions               (numbers, etc)
    // -    ($X:ident)      : is used for variable/function names   (strings)
    // -    ($X:block)      :
    // -    ($X:item)       :
    // -    ($X:literal)    : is used for literal constants
    // -    ($X:pat)        : (pattern)
    // -    ($X:path)       :
    // -    ($X:stmt)       : (statement)
    // -    ($X:tt)         : (operators and tokens tree)           (symbols, e.g. +=)
    // -    ($X:ty)         : (type)
    // -    ($X:vis)        : (visibility qualifier)

    // [Overload] Macros can be overloaded to accept different combinations of arguments. 
    // In that regard, macro_rules! can work similarly to a match block.
    // 

    // [Repeat] Macros can use 
    // +    in the argument list to indicate that an argument may repeat at least once, or 
    // *    to indicate that the argument may repeat zero or more times.

    //  ---------------------------------
    // [NO DESIGNATOR EXAMPLE] 

    // This is a simple macro named `say_hello`.
    macro_rules! say_hello {
        () => {                         // `()` indicates that the macro takes no argument.
            println!("Hello Macro!")    // The macro will expand into the contents of this block.
        };
    }
    say_hello!();

    //  ---------------------------------
    // [`ident` EXAMPLE] 

    // The arguments of a macro are prefixed by a dollar sign $ and type annotated with a designator
    macro_rules! create_function {
        // This macro takes an argument of designator `ident` and creates a function named `$func_name`.
        // The `ident` designator is used for variable/function names.
        ($func_name:ident) => { 
            fn $func_name() {
                // The `stringify!` macro converts an `ident` into a string.
                println!("You called {:?}()", stringify!($func_name));
            }
        };
        () =>{ // Send case

        }
    }
    create_function!(foo);
    foo();

    //  ---------------------------------

}

fn conversion(){
    println!("\n======================================================================");
    println!("* CONVERSION)");

    // Primitive types can be converted to each other through casting.

    // Rust addresses conversion between custom types (i.e., struct 
    // and enum) by the use of traits. The generic conversions will 
    // use the From and Into traits. However there are more specific 
    // ones for the more common cases, in particular when converting to and from Strings.

    // FROM
    {
        use std::convert::From;

        // Struct
        #[derive(Debug)]
        struct Number {
            _value: i32,
        }

        // Conversion
        impl From<i32> for Number {
            fn from(item: i32) -> Self {
                Number { _value: item }
            }
        }

        // Usage
        let num = Number::from(10);
        pvar!(num);
    }

    // INTO
    {
        use std::convert::Into;

        // Struct
        #[derive(Debug)]
        struct Number {
            _value: i32,
        }

        // Conversion
        impl Into<Number> for i32 {
            fn into(self) -> Number {
                Number { _value: self }
            }
        }

        // Usage
        let int = 20;
        let num: Number = int.into();
        pvar!(num);
    }    

    // From and Into are interchangable
}

fn destructoring_dereferencing(){
    // Dereferencing uses *
    // Destructuring uses &, ref, and ref mut

    // For pointers, a distinction needs to be made between destructuring and 
    // dereferencing as they are different concepts which are used differently 
    // from languages like C/C++.

    println!("\n======================================================================");
    println!("* DESCTRUCTORING & DEREFERENCING");

    // Destructuring in Rust refers to breaking apart a data structure to access its 
    // components. This is often done using pattern matching with let, match, or function arguments.

    // Tuple Destructuring:
    let tuple = (1, "hello", 4.5);
    let (a, b, c) = tuple;
    println!("Destructure tuple => a: {}, b: {}, c: {}", a, b, c);

    // Struct Destructuring:
    struct Point {
        x: i32,
        y: i32,
    }
    
    let point = Point { x: 5, y: 10 };
    let Point { x, y } = point;
    println!("Destructure struct => x: {}, y: {}", x, y);
    

    // Enum Destructuring:
    #[allow(dead_code)]
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
    }
    
    let msg = Message::Move { x: 3, y: 4 };
    match msg {
        Message::Quit => println!("Destructure enum => Quit"),
        Message::Move { x, y } => println!("Destructure enum => Move to x: {}, y: {}", x, y),
        Message::Write(text) => println!("Destructure enum => Text: {}", text),
    }


    // When doing pattern matching or destructuring via the let binding, the ref keyword can be used 
    // to take references to the fields of a struct/tuple. The example below shows a few instances where 
    // this can be useful:
    let Point { ref x, ref y } = point;
    println!("Destructure struct => x: {}, y: {}", x, y);

    // -----------------------------------------------------------

    // Dereferencing in Rust involves accessing the value that a reference 
    // or pointer points to. The dereference operator is *. This is commonly 
    // used with references (&T), mutable references (&mut T), and smart pointers 
    // like Box, Rc, and Arc.

    //B.1 Assign a reference of type `i32`. The `&` signifies there
    // is a reference being assigned.
    let reference = &4;
    pvar!(reference);

    //B.2 What if you don't start with a reference? `reference` was a `&`
    // because the right side was already a reference. This is not
    // a reference because the right side is not one.
    let _not_a_reference = 3;
    pvar!(_not_a_reference);

    //B.3 Rust provides `ref` for exactly this purpose. It modifies the
    // assignment so that a reference is created for the element; this
    // reference is assigned.
    // A `ref` borrow on the left side of an assignment is equivalent to
    // an `&` borrow on the right side.
    let ref _is_a_reference = 3;
    pvar!(_is_a_reference);

    // ACCESS references
    match reference {
        // If `reference` is pattern matched against `&val`, it results
        // in a comparison like:
        // `&i32`
        // `&val`
        // ^ We see that if the matching `&`s are dropped, then the `i32`
        // should be assigned to `val`.
        &val => println!("Got a value of reference via destructuring: {:?}", val),
    }

    // To avoid the `&`, you dereference before matching.
    match *_is_a_reference {
        val => println!("Got a value of _is_a_reference via dereferencing: {:?}", val),
    }
}

fn binding(){
    // Indirectly accessing a variable makes it impossible to branch and use that variable without 
    // re-binding. match provides the @ sigil for binding values to names:

    println!("\n======================================================================");
    println!("* BINDING - @ operator");

    // A function `age` which returns a `u32`.
    fn age() -> u32 { 15 }
    match age() {
        0             => println!("I haven't celebrated my first birthday yet"),
        // Could `match` 1 ..= 12 directly but then what age
        // would the child be? Instead, bind to `n` for the
        // sequence of 1 ..= 12. Now the age can be reported.
        n @ 1  ..= 12 => println!("I'm a child of age {:?}", n),
        n @ 13 ..= 19 => println!("I'm a teen of age {:?}", n),
        // Nothing bound. Return the result.
        n             => println!("I'm an old person of age {:?}", n),
    }
}

// ---------------------------------------------------------------------------
// ---------------------------------------------------------------------------
// ---------------------------------------------------------------------------

fn arrays(){ // Fixed size list of elements of same data type T, stored in contiguous memory
    // Arrays are immutable by default and even with mut, its element count cannot be changed.
    // Syntax: let array: [T; length];

    println!("\n======================================================================");
    println!("* ARRAYS: (Fixed size list of elements of same data type)");

    let a = [1, 2, 3];
    pvar!(a);
    
    let a: [i64; 3] = [1, 2, 3];                // [Type; number of elements]
    pvar!(a);

    let empty_array: [i32; 0] = [];             // An empty array
    pvar!(empty_array);

    let mut mut_array: [i32; 3] = [1, 2, 3];    // Mutable array
    mut_array[0] = 2;
    mut_array[1] = 4;
    mut_array[2] = 6;
    pvar!(mut_array);

    let init_with_zeros = [0; 5];               // Initialize with zeros    [0, 0, 0, 0, 0]
    pvar!(init_with_zeros);

    let init_with_char = ["x"; 5];              // Initialize with char     ["x", "x", "x", "x", "x"]
    pvar!(init_with_char);

    // Arrays can be safely accessed using `.get`, which returns an `Option`
    print!("Access Array elements utilizing .get: ");
    let xs: [i32; 5] = [0, 1, 2, 3, 4];
    for i in 0..xs.len() + 1 {          // Oops, one element too far!
        match xs.get(i) {
            Some(xval) => print!("[{}] = {}, ", i, xval),
            None => println!(" Slow down! {} is too far!", i),
        }
    }
}


fn tupples(){ // Fixed size ordered list of elements of different(or same) data types
    // Tuples are also immutable by default and even with mut, its element count cannot be changed. 
    // Also, if you want to change an element’s value, the new value should have the same data type of previous value.

    println!("\n======================================================================");
    println!("* TUPPLES: (Fixed size ordered list of elements of different(or same) data types)");


    let a = (1, 1.5, true, 'a');                            // Declaration + assignment; without data type                        
    pvar!(a);
    let a: (i32, f64, bool, char) = (1, 1.5, true, 'a');    // Declaration + assignment; with data type 

    let mut mut_tupple = (1, 1.5);                          // Mutable tupple
    mut_tupple.0 = 2;                                       // Access tupple values via tuple indexing
    mut_tupple.1 = 3.0;

    let (c, d) = mut_tupple;                                // To variables <= from tupple
    let (e, _, _, f) = a;                                   // variables <= from tupple, and also not use some values
    let single_element_tupple = (0,);                       // Single-element tuple
    let nested_tupple = (mut_tupple, (2, 4), 5);            // Nested tupples

    pvar!(a, mut_tupple, c, d, e, f, single_element_tupple, nested_tupple);
}


fn slices(){
    // Syntax: let slice: &[T] = &parent_array[start..end];

    // Slices are similar to arrays, but their length is not known at compile time. 
    // Instead, a slice is a two-word object; the first word is a pointer to the data, 
    // the second word is the length of the slice
    
    // Dynamically-sized reference to another data structure
    // Imagine you want to get/ pass a part of an array or any other data structure. 
    // Instead of copying it to another array (or same data structure), Rust allows 
    // for creating a view/ reference to access only that part of the data. This view/ reference can be mutable or immutable.

    // Slices can be used to borrow a section of an array and have the type signature &[T].

    println!("\n======================================================================");
    println!("* SLICES: (Dynamically-sized reference to another data structure)");
    
    let parent_array: [i32; 4] = [1, 2, 3, 4];                  // Parent Array

    let whole_array_c1: &[i32] = &parent_array;                 // Slicing whole array
    let whole_array_c2 = &parent_array[..];                     // Slicing whole array
    let whole_array_c3 = &parent_array[0..4];                   // From 0th position to 4th(excluding)

    let from1_to3       = &parent_array[1..3];                  // From 1th position to 3th(excluding)  -> [2, 3]
    let from1_to_end    = &parent_array[1..];                   // From 1th position to end             -> [2, 3, 4]
    let from_start_to3  = &parent_array[..3];                   // From start to 3th(excluding)         -> [1, 2, 3]

    pvar!(parent_array, whole_array_c1, whole_array_c2, whole_array_c3, from1_to3, from1_to_end, from_start_to3);
}

fn vectors(){
    // Mainly a vector represent 3 things,
    // A pointer to the data
    // No of elements currently have(length)
    // Capacity (Amount of space allocated for any future elements).
    // If the length of a vector exceeds its capacity, its capacity will be increased automatically.

    println!("\n======================================================================");
    println!("* VECTORS: (kind of a re-sizable array but all elements must be in the same type, Vectors always allocate their data in a dynamically allocated heap.)");

    // Create empty vectors
    let empty_vec_with_new: Vec<i32> = Vec::new();                  // With new() keyword
    let empty_vec_with_vec_macro: Vec<u16> = vec![];                // Using the vec! macro
    let empty_vec_with_capacity: Vec<i32> = Vec::with_capacity(10);
    pvar!(empty_vec_with_new, empty_vec_with_vec_macro, empty_vec_with_capacity);

    // Create with values    
    let vec_init_wo_dt      = vec![1, 2, 3];
    let vec_init_w_dt       = vec![1u8, 2, 3];          // Suffixing 1st value with data type
    let vec_init_multiples  = vec![0; 5];               // Five zeroes
    pvar!(vec_init_wo_dt, vec_init_w_dt, vec_init_multiples);


    // Access and change data
    let mut c = vec![5, 4, 3, 2, 1];
    c[0] = 1;
    c[1] = 2;

    //push and pop
    let mut d: Vec<i32> = Vec::new();
    d.push(1); //[1] : Add an element to the end
    d.push(2); //[1, 2]
    d.pop();
    pvar!(c,d);
    println!("Length of d: {}, Capacity of d: {}", d.len(), d.capacity()); 
    println!();

    // VECTORS and iterators
    println!("> Vectors and iterators");
    let mut v = vec![1, 2, 3, 4, 5];

    for _i in &v { }        // A reference to {}", i
    for _i in &mut v { }    // A mutable reference to {}", i
    for _i in v { }         // Take ownership of the vector and its element {}", i

    println!("let mut v = vec![1, 2, 3, 4, 5];");
    println!("for i in &v {{ }}         // A reference to i");
    println!("for i in &mut v {{ }}     // A mutable reference to i");
    println!("for i in v {{ }}          // Take ownership of the vector and its element i");

}

fn structs(){
    // Structs are used to encapsulate related properties into one unified data type.
    // By convention, the name of the struct should follow PascalCase.
    // There are 3 variants of structs,

    // C-like structs => struct Point { x: f32, y: f32 }
    //      One or more comma-separated name:value pairs
    //      Brace-enclosed list
    //      Similar to classes (without its methods) in OOP languages
    //      Because fields have names, we can access them through dot notation
    
    println!("\n======================================================================");
    println!("* STRUCTS");
    println!("1. C-like structs");

    #[derive(Debug)]
    struct SampleStruct {  // Struct Declaration
        var_u8: u8,
        var_char: char,
        var_i64: i64
    }
    // Creating an instance
    let struct_instance = SampleStruct {var_u8: 0, var_char: 'X', var_i64: 0};
    println!("struct_instance = rgb({}, {}, {})", struct_instance.var_u8, struct_instance.var_char, struct_instance.var_i64);
    pvar!(struct_instance);

    // Destructure the instance using a `let` binding, this will not destruct original instance
    let SampleStruct {var_u8: destructed_u8, var_char: destructed_char, var_i64: destructed_i64} = struct_instance;
    pvar!(destructed_u8, destructed_char, destructed_i64); 
    println!();


    // ----------------------------

    // Tuple structs => struct Pair (i32, f32);
    //      One or more comma-separated values
    //      A parenthesized list like tuples
    //      Looks like a named tuples
    println!("2. Tuple structs");

    #[derive(Debug)]
    struct Color(u8, u8, u8);

    #[derive(Debug)]
    struct Kilometers(i32); // When a tuple struct has only one element, 
                            // we call it newtype pattern. Because it 
                            // helps to create a new type.

    // Create instances and access fields (you can also destructure - see c-like structs to access fields)
    let black = Color(0, 0, 0);
    let distance = Kilometers(20);
    pvar!(black, distance);
    println!("   black = Color({}, {}, {})", black.0, black.1, black.2);
    println!("   distance = Kilometers({})", distance.0);
    println!();


    // ----------------------------

    // Unit structs => struct Unit;
    //      A struct with no members at all
    //      It defines a new type but it resembles an empty tuple, ()
    //      Rarely in use, useful with generics
    // This is rarely useful on its own. But in combination with other features, it can become useful.
    println!("3. Unit structs");

    #[derive(Debug)]
    struct Electron;

    // Create instance
    let unit_struct = Electron;
    pvar!(unit_struct);

}

fn enums(){
    // An enum is a single type. It contains variants, which are possible values of the enum at a given time.
    // Variants can be accessed through :: notation, ex. Day::Sunday

    // Each enum variant can have,
    //      - No data (unit variant)
    //      - Named data (struct variant)
    //      - Unnamed ordered data (tuple variant)
    
    println!("\n======================================================================");
    println!("* ENUMS");

    #[allow(dead_code)]
    #[derive(Debug)]
    enum FlashMessage {
        Success,                                        // A unit variant
        Warning{ _category: i32, _message: String },    // A struct variant
        Error(String)                                   // A tuple variant
    }

    let mut form_status = FlashMessage::Success;
    pvar!(form_status);
    form_status = FlashMessage::Warning {_category: 2, _message: String::from("Field X is required")};
    pvar!(form_status);
    form_status = FlashMessage::Error(String::from("Connection Error"));
    pvar!(form_status);

    // Type aliases
    type Mess = FlashMessage;   // Create type alias
    let x = Mess::Success;
    pvar!(x);

    // ENUMS with explicit discriminator
    #[allow(dead_code)]
    enum Color {
        Red     = 0xff0000,
        Green   = 0x00ff00,
        Blue    = 0x0000ff,
    }
    let red = Color::Red as i32;
    pvar!(red);
}

fn generics(){
    // Sometimes, when writing a function or data type, we may want it 
    // to work for multiple types of arguments. In Rust, we can do this 
    // with generics.

    // The concept is, instead of declaring a specific data type we use 
    // an uppercase letter(or PascalCase identifier). ex, instead of 
    // x : u8 we use x : T . but we have to inform to the compiler that 
    // T is a generic type(can be any type) by adding <T> at first.

    println!("\n======================================================================");
    println!("* GENERICS");

    //  Generic can be:
    //      * Functions:        fn takes_two_things<T, U>(_x: T, _xy: U) { }
    //      * Structs:          struct Point<T> { _x: T, _y: T }
    //      * Enums:            enum Option<T> { Some(T), None, }
    //      * Traits:           trait Point<T> { fn value(&self) -> &T; }
    //      * Implementations:  impl<T> Point<T> { fn value (&self) -> &T { /* implementation */ } }
    //                          impl<T, U> DoubleDrop<T> for U { fn double_drop(self, _: T) { /* implementation */ } }

    // --------------------------
    // Generalizing functions
    fn takes_anything<T>(_x: T) {  }    // x has type T, T is a generic type
    takes_anything(8);                  // Implicitly specified type parameter `u32` to `takes_anything()`.
    takes_anything::<char>('a');        // Explicitly specified type parameter `char` to `takes_anything()`.      

    fn takes_two_things<T, U>(_x: T, _xy: U) { } // Multiple types
    takes_two_things(10, 20);

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

    // T can even be a trait, and have multiple implementation for different structures
    #[allow(dead_code)]
    trait GetSound { fn get_sound(&self) -> String; }

    // T MUST implement GetSound Trait! This is called a Bound!
    #[allow(dead_code)]
    fn make_sound<T: GetSound>(t: &T) { println!("{}!", t.get_sound()) } 

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

    // [+] Multiple Bounds 
    #[allow(dead_code)]
    fn multiple_bounds_function<T: std::fmt::Debug + std::fmt::Display>(_t: &T) {  } 

    // [,] Multiple Bounds, different types
    #[allow(dead_code)]
    fn multiple_bounds_diff_types<T: std::fmt::Debug, U: std::fmt::Debug>(_t: &T, _u:&U) {  } 

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

    // > Where clauses 
    // Instead of using this 
    //      impl <A: TraitB + TraitC,       D: TraitE + TraitF> 
    //              MyTrait<A, D> 
    //                  for YourType {}
    // Expressing bounds with a `where` clause, to be more readable
    //      impl <A, D> MyTrait<A, D> for YourType where
    //              A: TraitB + TraitC,
    //              D: TraitE + TraitF {}

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

    // > Associated items
    // The use of "Associated types" improves the overall readability of code by 
    // moving inner types locally into a trait as output types. Syntax for the 
    // trait definition is as follows:

    // * Without using associated types
    //      struct Container(i32, i32);
    //      trait Contains<A, B> {
    //          fn contains(&self, _: &A, _: &B) -> bool; // Explicitly requires `A` and `B`.
    //      }
    //      
    //      impl Contains<i32, i32> for Container {}
    //
    //      fn difference<A, B, C>(container: &A) -> i32 
    //      where
    //          A: Contains<B, C>                           // This should be a valid trait 
    //      { 
    //          container.last() - container.first()        // These should be implemented in an implementation
    //      }

    // * Using associated types
    //      struct Container(i32, i32);
    //      trait Contains {
    //          type A;
    //          type B;
    //          fn contains(&self, _: &Self::A, _: &Self::B) -> bool;
    //      }
    //
    //      impl Contains for Container {
    //          // Specify what types `A` and `B` are. If the `input` type
    //          // is `Container(i32, i32)`, the `output` types are determined
    //          // as `i32` and `i32`.
    //          type A = i32;
    //          type B = i32;
    //          ...
    //      }    
    //
    //      fn difference<C: Contains>(container: &C) -> i32 { ... }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // * Phantom and Unit

    // #[derive(Debug, Clone, Copy)]
    // enum Inch {}
    // #[derive(Debug, Clone, Copy)]
    // enum Mm {}
    
    // #[derive(Debug, Clone, Copy)]
    // struct Length<Unit>(f64, std::marker::PhantomData<Unit>);
    // impl<Unit> std::ops::Add for Length<Unit> {
    //     type Output = Length<Unit>;
    
    //     fn add(self, rhs: Length<Unit>) -> Length<Unit> {
    //         Length(self.0 + rhs.0, std::marker::PhantomData)
    //     }
    // }

    // let one_foot:  Length<Inch> = Length(12.0, std::marker::PhantomData);
    // let one_meter: Length<Mm>   = Length(1000.0, std::marker::PhantomData);

    // let two_feet = one_foot + one_foot;
    // println!("one foot + one_foot = {:?} in", two_feet.0);

    // let two_meters = one_meter + one_meter;
    // println!("one meter + one_meter = {:?} mm", two_meters.0);

    // ----------------------------
    // Generalizing structs
    #[derive(Debug)]
    struct Point<T> { _x: T, _y: T }
    let _gereral_struct = Point {_x: 2 as u8, _y: 10};
    pvar!(_gereral_struct);
    // When adding an implementation for a generic struct, the type parameters should be declared after the impl as well
    //   impl<T> Point<T> {

    // ----------------------------
    // Generalizing enums

    // Option and Result types are kind of special generic types which are already defined in Rust’s standard library. 
    
    // 1. [OPTION]
    println!(">>> Option type: (see code)");
    // 
    // enum Option<T> {
    //    Some(T),
    //    None,
    // }
    // 
    // The Option type is a way to use Rust’s type system to express the possibility of absence. 
    // An optional value can have either Some value or no value/ None.
    // 
    // Option has many assosiated methods (combinators):
    // 
    // E.g.:
    //  - unwrap()
    //  - map()
    //  - and_then()
    //  - flatten()
    //  - ...

    // Option has a built in method called map(), a combinator for the simple mapping of 
    // Some -> Some and 
    // None -> None. 
    // Multiple map() calls can be chained together for even more flexibility.
    // map() is described as a chainable way to simplify match statements
    // 
    #[derive(Debug)] #[allow(dead_code)] enum Input { Opt1, Opt2 }
    #[derive(Debug)] #[allow(dead_code)] struct Results(Input);

    fn match_option(_input: Option<Input>) -> Option<Results> {
        match _input {
            Some(inp) => Some(Results(inp)),
            None => None,
        }
    }

    fn map_option(_input: Option<Input>) -> Option<Results> {
        _input.map(|inp| Results(inp))
    }

    println!("match_option(Some(Input::Opt1) -> {:?}", match_option(Some(Input::Opt1)));
    println!("match_option(None)             -> {:?}", match_option(None)); 

    println!("map_option(Some(Input::Opt1)) ->  {:?}", map_option(Some(Input::Opt1)));
    println!("map_option(None)              ->  {:?}", map_option(None)); // 

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // #[derive(Debug)] struct Input;
    // fn f1(inp: Input) -> Option<Input> {}
    // fn f2(inp: Input) -> Option<Input> {}

    // [Example] Assume we have this procedure
    // input2 = f1(input1) -> Option<input>;
    // output = f2(input2) -> Option<input>;

    // [Option1]: This can conveniently be rewritten more compactly with `and_then()`:
    // output = f1(input1).and_then(f2) -> Option<input>
    // [Option2]: Otherwise we'd need to `flatten()` an `Option<Option<Food>>` to get an `Option<Food>`:
    // f1(food).map(f2).flatten()


    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // 2. [RESULT]
    println!(">>> Result type: (see code)");
    // 
    // enum Result<T, E> {
    //     Ok(T),
    //     Err(E),
    // }
    //
    // E.g. Result<String, String> -> Ok(String) & Err(String)
    // 
    // Option has many assosiated methods (combinators):
    // 
    // E.g.:
    //  - unwrap()
    //  - map()
    //  - and_then()
    //  - ...
    // 
    // Result type is a way to use Rust’s type system to express the possibility of error.
    // A result can represent either success/ Ok or failure/ Err
    // Result has many assosiated methods!


}

fn let_else(){
    println!("\n======================================================================");
    println!("* LET_ELSE");

    // With let-else, a refutable pattern can match and bind variables in the 
    // surrounding scope like a normal let, or else diverge (e.g. break, return, 
    // panic!) when the pattern doesn't match.

    use std::str::FromStr;

    fn get_count_item(s: &str) -> (u64, &str) {
        let mut it = s.split(' ');
        println!("let mut it = s.split(' ');");
        println!("let (Some(count_str), Some(item)) = (it.next(), it.next()) else {{ }};");
        let (Some(count_str), Some(item)) = (it.next(), it.next()) else {
            panic!("Can't segment count item pair: '{s}'");
        };
        println!("let Ok(count) = u64::from_str(count_str) else {{ }};");
        let Ok(count) = u64::from_str(count_str) else {
            panic!("Can't parse integer: '{count_str}'");
        };
        (count, item)
    }

    assert_eq!(get_count_item("3 chairs"), (3, "chairs"));

    // --------------------------------------------------
    // The scope of name bindings is the main thing that makes this different 
    // from match or if let-else expressions. You could previously approximate 
    // these patterns with an unfortunate bit of repetition and an outer let:
    let s = "3 chairs";
    let mut it = s.split(' ');
    let (count_str, _item) = match (it.next(), it.next()) {
        (Some(count_str), Some(item)) => (count_str, item),
        _ => panic!("Can't segment count item pair: '{s}'"),
    };
    let _count = if let Ok(count) = u64::from_str(count_str) {
        count
    } else {
        panic!("Can't parse integer: '{count_str}'");
    };


}

fn linked_list(){
    println!("\n======================================================================");
    println!("* LINKED LIST");

    #[derive(Debug)]
    enum List {
        Cons(u32, Box<List>),
        Nil,
    }

    impl List {
        fn new() -> List {  // Create an empty list
            List::Nil
        }

        fn prepend(self, elem: u32) -> List {
            List::Cons(elem, Box::new(self))
        }

        fn append(self, elem: u32) -> List {
            match self {
                List::Cons(head, tail) => List::Cons(head, Box::new(tail.append(elem))),
                List::Nil => List::Cons(elem, Box::new(List::Nil)),
            }
        }

        fn len(&self) -> u32 {
            match *self {
                List::Cons(_, ref tail) => 1 + tail.len(),
                List::Nil => 0,
            }
        }

        fn stringify(&self) -> String {
            match *self {
                List::Cons(head, ref tail)  => format!("{}, {}", head, tail.stringify()),
                List::Nil                   => format!("Nil")
            }
        }
    }

    let mut list = List::new();    
    list = list.prepend(2);
    list = list.prepend(1);
    list = list.append(3);
    list = list.append(4);

    println!("{}", list.stringify());
    pvar!(list); 
    println!("linked list has length: {}", list.len());
}

fn control_flow(){
    println!("\n======================================================================");
    println!("* CONTROL FLOW & VARIABLES");
    println!("A. Use IF-ELSE IF -ELSE as in C++ but without () on conditions");
    println!();
    // ----------------------------

    // Using with let statement. Return data type should be the same on each block when using this as an expression.
    let is_valid = if 10 < 18 { true } else { false };
    println!("B. LET and assign");
    println!("  let is_valid = if 10 < 18 {{ true }} else {{ false }}; // Return data type should be the same on each block when using this as an expression.");
    pvar!(is_valid);
    println!();
    
    // ----------------------------
    let tshirt_width = 20;
    let tshirt_size = match tshirt_width {
        16 => "S",                      // check 16
        17 | 18 => "M",                 // check several values at the same time
        19 ..= 21 => "L",               // Match an inclusive range
        22 => "XL",                     // chech 22
        _ => "Not Available",           // other case
    };
    println!("C. Match statement (in the match, you can have tuples)");
    println!("  let tshirt_width = 20;");
    println!("  let tshirt_size = match tshirt_width {{");
    println!("    16 => \"S\",                      // check 16");
    println!("    17 | 18 => \"M\",                 // check several values at the same time");
    println!("    19 ..= 21 => \"L\",               // Match an inclusive range");
    println!("    22 => \"XL\",                     // chech 22");
    println!("    _ => \"Not Available\",           // other cases");
    println!("  }};");
    pvar!(tshirt_size);
    println!();

    // ----------------------------
    println!("D. LOOPs");
    println!("D.1 loop {{");
    println!("        println!(\"Infinity loop!\");");
    println!("        // Here you can use breaks and continue");
    println!("    }}");
    println!();

    // ----------------------------
    println!("D.2 labels in loops");
    'outer_loop: loop {
        loop {
            break 'outer_loop; // kill outer_loop
        }
    }
    println!("'outer_loop: loop {{");
    println!("    loop {{");
    println!("        break 'outer_loop; // kill outer_loop");
    println!("    }}");
    println!(" }}");
    println!();


    // ----------------------------
    println!("D. WHILE (You can use labels also in this situation, along with breaks & continue)");
    let mut a = 1;
    while a <= 10 { a += 1; }
    println!("   let mut a = 1;");
    println!("   while a <= 10 {{ a += 1; }}");
    println!();

    // ----------------------------
    println!("E. FOR (You can use labels also in this situation, along with breaks & continue)");
    println!("E.1 for i in 0..10 {{ // 0 to 10 (10 exclusive) upper limit can be infinity -> 0.. ");
    println!("    print!(\"{{i}} \");");
    println!("}}");

    for i in 0..10 {
        print!("{i} ");
    }
    println!();
    println!();

    // ----------------------------

    println!("E.2. for i in 0..=10 {{ // 0 to 10 (10 inclusive)");
    println!("    print!(\"{{i}} \");");
    println!("}}");
    for i in 0..=10 {
        print!("{i} ");
    }
    println!();
}

fn iterators(){
    println!("\n======================================================================");
    println!("* ITERATORS");
    
    // iter - This borrows each element of the collection through each iteration. Thus leaving the 
    // collection untouched and available for reuse after the loop
    println!("1. for name in names.iter() {{ // let names = vec![\"Bob\", \"Frank\", \"Ferris\"];");
    println!("    match name {{ &\"Ferris\" => println!(\"Not valid\"), _ => println!(\"Name: {{ }}\", name) }} ");
    println!("}}");
    println!();
    let mut _vec1 = vec![1, 2, 3];
    let mut _iter = _vec1.iter();
    let _index_of_first_even_number = _vec1.iter().position(|&x| x % 2 == 0);
    pvar!(_vec1, _iter, _index_of_first_even_number);
    println!("* [Iterator::position]    2 in _vec1: {:?}",      _index_of_first_even_number);               // Iterator::position
    println!("* [Iterator::any]         2 in _vec1: {}",        _vec1.iter().any(|&x| x == 2));             // Iterator::any
    println!("* [Iterator::find]        Find 2 in vec1: {:?}",  _iter.find(|&&x| x == 2));                  // Iterator::find
    println!();


    // into_iter - This consumes the collection so that on each iteration the exact data is provided. 
    // Once the collection has been consumed it is no longer available for reuse as it has been 'moved' within the loop.
    println!("2. for name in names.into_iter() {{ // let names = vec![\"Bob\", \"Frank\", \"Ferris\"];");
    println!("    match name {{ \"Ferris\" => println!(\"Not valid\"), _ => println!(\"Name: {{ }}\", name) }} ");
    println!("}}");
    println!();
    let mut _vec2 = vec![4, 5, 6];
    let mut _into_iter = _vec2.into_iter();
    let _index_of_first_negative_number = _into_iter.position(|x| x < 0);
    pvar!(_into_iter, _index_of_first_negative_number);
    println!("- [Iterator::position]    2 in _vec1: {:?}",      _index_of_first_negative_number);    // Iterator::position 
    println!("* [Iterator::any]         2 in vec2: {}",         _into_iter.any(|x| x == 2));        // Iterator::any
    println!("* [Iterator::find]        Find 2 in vec2: {:?}",  _into_iter.find(| &x| x == 2));     // Iterator::find
    println!();


    // iter_mut - This mutably borrows each element of the collection, allowing for the collection to be modified in place.
    println!("3. for name in names.iter_mut() {{ // let names = vec![\"Bob\", \"Frank\", \"Ferris\"];");
    println!("    *name = match name {{ &mut \"Ferris\" => \"Not valid\", _ => \"Name: \" }} ");
    println!("}}");
    println!();

    // =================================
    // Examples:

    // A.
    //  - Assume this function to get the sum of all items that have a specific process: 
    //      fn count_iterator(map: &HashMap<String, Progress>, value: Progress) -> usize
    //  - Solution:
    //  map.values().filter(|&&val| val == value).count()

    // B: 
    //  - Assume this function to do the same, but this time on a slice instead of a hashmap
    //      fn count_collection_iterator(collection: &[HashMap<String, Progress>], value: Progress) -> usize {
    //  - Solution 
    //      collection.iter()               // Slice to iterator
    //      .flat_map(|map| map.values())   // Flatten the iterattors from each map
    //      .filter(|&&val| val == value)   // Same as before for each map
    //      .count()                        //
}

fn modules(){
    // Rust provides a powerful module system that can be used to hierarchically split 
    // code in logical units (modules), and manage visibility (public/private) between them.

    // A module is a collection of items: functions, structs, traits, impl blocks, and even 
    // other modules

    println!("\n======================================================================");
    println!("* MODULES");

    // By default, the items in a module have private visibility, but this can be overridden with the pub modifier. 
    // Only the public items of a module can be accessed from outside the module scope.
    mod my_mod {
        #[allow(dead_code)]
        fn private_function() { }   // Items in modules default to private visibility.
        
        #[allow(dead_code)]
        pub fn function() {}        // Use the `pub` modifier to override default visibility.
        
        #[allow(dead_code)]
        pub(crate) fn pub_in_crate() {} // pub(crate) makes functions visible only within the current crate
        // --------------------------------------------------

        // Modules can also be nested
        pub mod nested {
            // Functions declared using `pub(in path)` syntax are only visible
            // within the given path. `path` must be a parent or ancestor module
            // #[allow(dead_code)]
            // pub(in crate::my_mod) fn public_function_in_my_mod() {}

            // Functions declared using `pub(self)` syntax are only visible within
            // the current module, which is the same as leaving them private
            #[allow(dead_code)]
            pub(self) fn public_function_in_nested() {}

            // Functions declared using `pub(super)` syntax are only visible within
            // the parent module
            #[allow(dead_code)]
            pub(super) fn public_function_in_super_mod() {}
        }
    }
}

fn crates(){
    // A crate is a compilation unit in Rust. Whenever rustc some_file.rs 
    // is called, some_file.rs is treated as the crate file. If some_file.rs 
    // has mod declarations in it, then the contents of the module files 
    // would be inserted in places where mod declarations in the crate file
    // are found, before running the compiler over it. In other words, 
    // modules do not get compiled individually, only crates get compiled.

    // A crate can be compiled into a binary or into a library. By default, 
    // rustc will produce a binary from a crate. This behavior can be 
    // overridden by passing the --crate-type flag to lib.
    println!("\n======================================================================");
    println!("* CRATES");

    // 1. Create a library 
    // Libraries get prefixed with "lib", and by default they get named 
    // after their crate file, but this default name can be overridden by 
    // passing the --crate-name option to rustc or by using the crate_name 
    //      rustc --crate-type=lib rary.rs
    //      ls lib*
    //          library.rlib

    // 2. Using a Library
    // To link a crate to this new library you may use rustc's --extern flag. 
    // All of its items will then be imported under a module named the same 
    // as the library. This module generally behaves the same way as any other module.

    // ------------------------------
    // extern crate rary; // May be required for Rust 2015 edition or earlier
    // fn main() {
    //      rary::public_function();
    // }
    // ------------------------------

    // # Where library.rlib is the path to the compiled library, assumed that it's
    // # in the same directory here:
    // $ rustc executable.rs --extern rary=library.rlib && ./executable 
}

fn impls(){
    // When we discussed about C-like structs, I mentioned that those are 
    // similar to classes in OOP languages but without their methods. impls 
    // are used to define methods for Rust structs and enums.

    println!("\n======================================================================");
    println!("* IMPL");

    // Impls without traits

    struct Player {
        _first_name: String,
        _last_name: String,
    }
    
    // Implementation must appear in the same crate as the self type and have the same name as struct
    impl Player {
        fn get_full_name(&self) -> String {
            format!("{} {}", self._first_name, self._last_name)
        }
    }
    // -------------

    // Example, create instanse, and utilize function
    let _player_1 = Player { _first_name: "Rafael".to_string(), _last_name: "Nadal".to_string() };
    println!("Player's full name: {}", _player_1.get_full_name());

}


fn traits(){
    // Traits are kind of similar to interfaces in OOP languages. 
    // They are used to define the functionality a type must provide. 
    // Multiple traits can be implemented for a single type.
    // But traits can also include default implementations of methods. 
    // Default methods can be overridden when implementing types.

    // Other than functions, traits can contain constants and types.
    // And also in Rust, new traits can be implemented for existing types even for types like i8, f64 and etc.
    // Same way existing traits can be implemented for new types you are creating.

    println!("\n======================================================================");
    println!("* TRAIT");

    
    // Impls & traits, without default methods

    struct Player {
        _first_name: String,
        _last_name: String,
    }

    trait FullName {
        // As you can see methods take a special first parameter, the type itself. 
        // It can be either self, &self, or &mut self; 
        //  - self        if it’s a value on the stack (taking ownership), 
        //  - &self       if it’s a reference 
        //  - &mut self   if it’s a mutable reference.
        fn get_full_name(&self) -> String;

        // Some other languages support static methods. At such times,
        // we call a function directly through the class without creating 
        // an object. In Rust, we call them Associated Functions. we 
        // use :: instead of . when calling them from the struct. 
        // ex. Player::new(“Elon", "Musk”);
        fn new(first_name: String, last_name: String) -> Player {
            Player {
                _first_name: first_name,
                _last_name: last_name,
            }
        }
    }
    
    impl FullName for Player {
        fn get_full_name(&self) -> String {
            format!("{} {}", self._first_name, self._last_name)
        }
    }

    // Example, create instanse, and utilize function
    let _player_1 = Player::new("Elon".to_string(), "Musk".to_string());
    println!("Player's full name: {}", _player_1.get_full_name());

    // Returning Traits with dyn
    struct Sheep {}
    struct Cow {}
    trait Animal { }
    impl Animal for Sheep {}
    impl Animal for Cow {}
    fn random_animal(random_number: f64) -> Box<dyn Animal> {
        if random_number < 0.5 {
            Box::new(Sheep {})
        } else {
            Box::new(Cow {})
        }
    }
    let _animal = random_animal(0.1);
}

fn traits_inheritance(){
    #[allow(dead_code)]
    trait Person {
        fn full_name(&self) -> String;
    }

    #[allow(dead_code)]
    trait Expat {
    }
    
    #[allow(dead_code)]
    trait Employee : Person { // Employee inherits from person trait
      fn job_title(&self) -> String;
    }
    
    #[allow(dead_code)]
    trait ExpatEmployee : Employee + Expat { // ExpatEmployee inherits from Employee and Expat traits
      fn additional_tax(&self) -> f64;
    }
}

fn associated_functions_methods(){
    // Some functions are connected to a particular type. These come in two forms: 
    // associated functions, and methods. Associated functions are functions that 
    // are defined on a type generally, while methods are associated functions that 
    // are called on a particular instance of a type.

    // Associated functions are called using double colons
    //      Point::origin()

    // Methods are called using the dot operator
    //      rectangle.perimeter()
    println!("\n======================================================================");
    println!("* ASSOCIATED FUNCTIONS AND METHODS");

    #[allow(dead_code)]
    struct Point {
        x: f64,
        y: f64,
    }
    
    // Implementation block, all `Point` associated functions & methods go in here
    impl Point {
        // This is an "associated function" because this function is associated with
        // a particular type, that is, Point.
        //
        // Associated functions don't need to be called with an instance.
        // These functions are generally used like constructors.
        #[allow(dead_code)]
        fn origin() -> Point {
            Point { x: 0.0, y: 0.0 }
        }

        // Another associated function, taking two arguments:
        #[allow(dead_code)]
        fn new(x: f64, y: f64) -> Point {
            Point { x: x, y: y }
        }
    }

    // ------
    #[allow(dead_code)]
    struct Rectangle {
        p1: Point,
        p2: Point,
    }

    impl Rectangle {
        // This is a method
        // `&self` is sugar for `self: &Self`, where `Self` is the type of the
        // caller object. In this case `Self` = `Rectangle`
        #[allow(dead_code)]
        fn area(&self) -> f64 {
            // `self` gives access to the struct fields via the dot operator
            let Point { x: x1, y: y1 } = self.p1;
            let Point { x: x2, y: y2 } = self.p2;
    
            // `abs` is a `f64` method that returns the absolute value of the
            // caller
            ((x1 - x2) * (y1 - y2)).abs()
        }
    
        #[allow(dead_code)]
        fn perimeter(&self) -> f64 {
            let Point { x: x1, y: y1 } = self.p1;
            let Point { x: x2, y: y2 } = self.p2;
    
            2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
        }
    
        // This method requires the caller object to be mutable
        // `&mut self` desugars to `self: &mut Self`
        #[allow(dead_code)]
        fn translate(&mut self, x: f64, y: f64) {
            self.p1.x += x;
            self.p2.x += x;
    
            self.p1.y += y;
            self.p2.y += y;
        }
    }
}

fn attributes(){
    // An attribute is metadata applied to some module, crate or item. This metadata can be used to/for:
    //  - conditional compilation of code
    //  - set crate name, version and type (binary or library)
    //  - disable lints (warnings)
    //  - enable compiler features (macros, glob imports, etc.)
    //  - link to a foreign library
    //  - mark functions as unit tests
    //  - mark functions that will be part of a benchmark
    //  - attribute like macros

    // Attributes look like:
    //  - #[outer_attribute] 
    //  - #![inner_attribute] 
    // with the difference between them being where they apply.

    // Attributes can take arguments with different syntaxes:
    //      #[attribute = "value"]
    //      #[attribute(key = "value")]
    //      #[attribute(value)]
    //      #[attribute(value1, value2)]


    println!("\n======================================================================");
    println!("* ATTRIBUTES");

    // The compiler is capable of providing basic implementations for some traits via the 
    // #[derive] attribute. These traits can still be manually implemented if a more complex 
    // behavior is required.
    // The following is a list of derivable traits:
    // -    Comparison traits: Eq, PartialEq, Ord, PartialOrd.
    // -    Clone, to create T from &T via a copy.
    // -    Copy, to give a type 'copy semantics' instead of 'move semantics'.
    // -    Hash, to compute a hash from &T.
    // -    Default, to create an empty instance of a data type.
    // -    Debug, to format a value using the {:?} formatter.

    // Examples:
    // -    #[derive(Debug)]

    // -----------------

    // #[allow(dead_code)]      # disables the `dead_code` lint (warn about unused functions)

    // -----------------

    // Crates
    // This crate is a library
    // #![crate_type = "lib"]
    // The library is named "rary"
    // #![crate_name = "rary"]

    // -----------------

    // Configuration conditional checks are possible through two different operators:
    //      #[cfg(...)]         the cfg attribute: in attribute position
    //      cfg!(...)           the cfg! macro: in boolean expressions
    // While the former enables conditional compilation, the latter conditionally evaluates
    // to true or false literals allowing for checks at run-time. Both utilize identical argument syntax.
    // cfg!, unlike #[cfg], does not remove any code and only evaluates to true or false. 
    // For example, all blocks in an if/else expression need to be valid when cfg! is used for the condition, regardless of what cfg! is evaluating.

    // This function only gets compiled if the target OS is linux
    #[cfg(target_os = "linux")]
    fn are_you_on_linux() { println!("You are running linux!"); }
    // And this function only gets compiled if the target OS is *not* linux
    #[cfg(not(target_os = "linux"))]
    fn are_you_on_linux() { println!("You are *not* running linux!"); }

    are_you_on_linux();
    if cfg!(target_os = "linux") { println!("Yes. It's definitely linux!"); } 
    else { println!("Yes. It's definitely *not* linux!"); }

    // -----------------

    // Some conditionals like target_os are implicitly provided by rustc, but 
    // custom conditionals must be passed to rustc using the --cfg flag.
    // #[cfg(some_condition)]
    // fn conditional_function() { println!("condition met!"); }
    // rustc --cfg some_condition custom.rs

}

fn oop(){
    // When regarding OOP in Rust, attributes and methods are 
    // placed separately on structs and traits. Structs contain 
    // only attributes, traits contain only methods. They are 
    // getting connected via impls.

    println!("\n======================================================================");
    println!("* OOP");

    modules();
    impls();
    traits();
    traits_inheritance();
    associated_functions_methods();
}

fn error_handling(){
    println!("\n======================================================================");
    println!("* ERROR_HANDLING");

    // ---------------------------------------------------------------------------------------

    // abort and unwind (set flag: rustc main.rs -C panic=abort)
    #[cfg(panic = "unwind")]                                        // Define if unwind
    fn ah() { println!("Spit it out!!!!"); }

    #[cfg(not(panic = "unwind"))]                                   // Define if NOT unwind
    fn ah() { panic!("AAAaaaaa!!!!"); }                             // Panic is considered to be used for errors

    ah();                                                           // Use

    // ---------------------------------------------------------------------------------------
    // Use Options<> & unwrap to handle errors
    // - Option can hold a None
    // - `unwrap` returns a `panic` when it receives a `None`.
    // Example:

    let _drink: Option<&str> = Some("lemonade");
    let _drink = _drink.unwrap();        // This will run without problem

    let _drink: Option<&str> = None;
    // let _drink = _drink.unwrap();             // This will return a panic

    // ---------------------------------------------------------------------------------------
    // Unpacking options with ?
    // You can unpack Options by using match statements, but it's often easier to use the ? operator. 
    // If x is an Option, then evaluating x? will return the underlying value if x is Some, otherwise 
    // it will terminate whatever function is being executed and return None.

    // If `current_age` is `None`, this returns `None`.
	// If `current_age` is `Some`, the inner `u8` value + 1
    // gets assigned to `next_age`
    fn age(current_age: Option<u32>) -> Option<u32>{
        let _next_age: u32 = current_age? + 1;
        Some(_next_age)
    }
    println!("age[Some(10)]: {:?}", age(Some(10)));
    println!("age[None]    : {:?}", age(None));

}

fn ownership(){
    // Variable bindings have ownership of what they’re bound to. 
    // A piece of data can only have one owner at a time. When 
    // a binding goes out of scope, Rust will free the bound resources. 
    // This is how Rust achieves memory safety.

    println!("\n======================================================================");
    println!("* OWNERSHIP");

    // When assigning a variable binding to another variable binding or when passing it to a function(Without referencing), if its data type is a

    // Copy Type
    //    Bound resources are made a copy and assign or pass it to the function.
    //    The ownership state of the original bindings is set to “copied” state.
    //    Mostly Primitive types

    // Move type
    //    Bound resources are moved to the new variable binding and we can not access the original variable binding anymore.
    //    The ownership state of the original bindings is set to “moved” state.
    //    Non-primitive types


    // ----------------

    // By reference: &T
    //      assing by reference means you are borrowing the value without taking ownership. 
    //      The function can read the value but cannot modify it. This is useful when you 
    //      want to allow read access to a value without giving up ownership or allowing modifications.
    // E.g.: fn print_length(s: &String) {}

    // By mutable reference: &mut T
    //      Passing by mutable reference means you are borrowing the value and allowing the function to 
    //      modify it. The function can read and modify the value. This is useful when you want to allow 
    //      modifications to a value without taking ownership.
    // E.g.: fn append_world(s: &mut String) {}

    // By value: T
    //      Passing by value means you are transferring ownership of the value to the function. The function 
    //      can read and modify the value, but the original owner loses access to the value. This is useful 
    //      when you want the function to take full control of the value.
    // E.g.: fn consume_string(s: String) {} 

    // --------------------
    // Key Differences

    // Ownership:
    //     - &T (By reference): The caller retains ownership. The function borrows the value.
    //     - &mut T (By mutable reference): The caller retains ownership. The function borrows and can modify the value.
    //     - T (By value): The function takes ownership. The caller loses access to the value after passing it.

    // Mutability:
    //     - &T: The function cannot modify the value.
    //     - &mut T: The function can modify the value.
    //     - T: The function can modify the value, but the caller loses access.

    // Use Cases:
    //     - &T: Use when you need to read a value without modifying it or transferring ownership.
    //     - &mut T: Use when you need to modify a value without transferring ownership.
    //     - T: Use when you want the function to take full control of the value, possibly modifying or consuming it.


    // Because variables are in charge of freeing their own resources, resources can only have one 
    // owner. This prevents resources from being freed more than once. Note that not all variables 
    // own resources (e.g. references).

    // When doing assignments (let x = y) or passing function arguments by value (foo(x)), the 
    // ownership of the resources is transferred. In Rust-speak, this is known as a move.

    // Mutability of data can be changed when ownership is transferred.

    // -------------------------------------------------
    // Multiple ownership
    println!("> Multiple ownership");

    // RC
    println!();
    println!(" * std::rc::Rc");
    use std::rc::Rc;

    let parent_vec = vec![1,2,3,4];
    let shared_vec = Rc::new(parent_vec);
    
    let _child_vec1 = Rc::clone(&shared_vec);
    println!("  - Create child_vec1, reference count of shared_vec = {}", Rc::strong_count(&shared_vec));
    let _child_vec2 = Rc::clone(&shared_vec);
    println!("  - Create child_vec2, reference count of shared_vec = {}", Rc::strong_count(&shared_vec));

    drop(_child_vec1);
    drop(_child_vec2);

    // ARC (Thread safe)
    println!();
    println!(" * std::sync::Arc // Thread safe!");
    use std::sync::Arc;

    let numbers: Vec<u32> = (0..100u32).collect();
    let shared_numbers = Arc::new(numbers);

    let _child_numbers_1 = Arc::clone(&shared_numbers);
    println!("  - Create _child_numbers_1, reference count of shared_numbers = {}", Arc::strong_count(&_child_numbers_1));
    let _child_numbers_2 = Arc::clone(&shared_numbers);
    println!("  - Create _child_numbers_2, reference count of shared_numbers = {}", Arc::strong_count(&_child_numbers_2));

    drop(_child_numbers_1);
    drop(_child_numbers_2);
}

fn borrowing(){
    // To receive something with the promise of returning it.

    // In real life applications, most of the times we have to pass variable 
    // bindings to other functions or assign them to other variable bindings. 
    // In this case, we are referencing the original binding; borrow the data of it.
    
    // There are two types of Borrowing,
    // 1. Shared Borrowing (&T)
    //    A piece of data can be borrowed by a single or multiple users, but data should not be altered.
    // 2. Mutable Borrowing (&mut T)
    //    A piece of data can be borrowed and altered by a single user, but the data should not be accessible for any other users at that time.

    // Rules for borrowings
    //  There are very important rules regarding borrowing,
    //      1. One piece of data can be borrowed either as a shared borrow or as a mutable borrow at a given time. But not both at the same time.
    //      2. Borrowing applies for both copy types and move types.
    //      3. The concept of Liveness

    println!("\n======================================================================");
    println!("* BORROWING");

    // Examples for Shared Borrowing
    let a = [1, 2, 3];
    let b = &a;

    println!("* Examples for Shared Borrowing");
    println!("   let a = [1, 2, 3];");
    println!("   let b = &a;");
    println!("   a: {:?} | b: {:?}", a, b);

    // Examples for Mutable Borrowing
    println!("* Examples for Mutable Borrowing");
    println!("   let mut a = [1, 2, 3];");
    println!("   {{");
    println!("      let b = &mut a;");
    println!("      b[0] = 4");

    let mut a = [1, 2, 3];
    {
        let b = &mut a;
        b[0] = 4;
        println!("      b: {:?}", b); // [4, 2, 3]
    }

    println!("   }}");
    println!("   a: {:?}", a); // [4, 2, 3]
}

fn lifetimes(){    
    // Lifetimes in Rust are a way to ensure that references are always valid

    // They are used by the Rust compiler (borrow checker actually) to track how long references should be 
    // valid, which helps prevent dangling references and other common memory 
    // safety issues. Lifetimes do not change the runtime behavior of a program; 
    // they only affect how code is compiled.

    println!("\n======================================================================");
    println!("* LIFETIMES:");

    // Lifetime Explicit annotation
    // A lifetime annotation is written as an apostrophe followed by a name, such as 'a. 
    // Lifetime annotations appear in 
    //      - function signatures, 
    //      - struct definitions, 
    //      - and impl blocks.

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~

    // [Function Signatures] E.g.
    // fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {}
    // This means the returned reference will be valid as long as both x and y are valid.

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~

    // [Struct Definitions] E.g.
    // struct Book<'a> {
    //     title: &'a str,
    //     author: &'a str,
    // }
    // Here, the Book struct holds references to strings, and these references must be valid for the same lifetime 'a.

    // struct Borrowed<'a>(&'a i32);

    // enum Either<'a> {
    //     Num(i32),
    //     Ref(&'a i32),
    // }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~

    // [Impl Blocks] E.g.
    // impl<'a> Book<'a> {
    //     fn get_title(&self) -> &str {
    //         self.title
    //     }
    // }
    // Here, self and the return value &str both have the same lifetime 'a.

    // #[derive(Debug)]
    // struct Borrowed<'a> { x: &'a i32, }
    //
    // impl<'a> Default for Borrowed<'a> {
    //     fn default() -> Self {
    //         Self { x: &10, }
    //     }
    // }
    // 
    // let b: Borrowed = Default::default();

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // Bounds
    #[derive(Debug)]   
    #[allow(dead_code)] 
    struct Ref<'a, T: 'a>(&'a T);
    fn print_ref<'a, T>(_t: &'a T) where T: std::fmt::Debug + 'a {  }   // Trait to bound with: std::fmt::Debug;

    let ref_x = Ref(&7);
    print_ref(&ref_x);

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // Static keyword 
    
    let _s: &'static str = "hello world";        // A reference with 'static lifetime:
    #[allow(dead_code)] 
    fn generic<T>(_x: T) where T: 'static {}     // 'static as part of a trait bound:
}

fn user_input(){
    println!("\n======================================================================");
    println!("* USER INPUT");

    // print!("Enter a number:");
    // let mut input = String::new();
    // std::io::stdin().read_line(&mut input).expect("Failed to read input.");
    // let num: f64 = input.trim().parse().expect("Please enter a valid number.");
    // println!("Your number is: {num}")
}

fn execute_system_cmd(){
    println!("\n======================================================================");
    println!("* SYSTEM COMMAND EXECUTION");

    use std::process::Command;
    let rust_version_output = Command::new("rustc").arg("--version").output().expect("Failed to execute rustc");
    let rust_version = String::from_utf8_lossy(&rust_version_output.stdout);
    println!("Rust version: {}", rust_version);
}

fn tests(){
    println!("\n======================================================================");
    println!("* TESTS");

    // fn function_under_test(num: u32){
    //      num
    // }

    // #[cfg(test)]
    // mod tests {
    // use super::*;

    //     #[test]
    //     fn test_function_returns_1() {
    //         assert_eq!(function_under_test(1), 1)
    //     }
    // }
}

fn threads(){
    println!("\n======================================================================");
    println!("* THREADS");

    use std::thread;
    let mut handles = Vec::new();          // Thread handlers (Joinhandle objs)

    for i in 0..10 {                                        // Will create 10 threads
        let handle = thread::spawn(move || {    // Spawn each thread
            println!("[SPAWNED] Thread {i}");                      // Thread boby
            i                                                    // Thread return value
        });
        handles.push(handle);                                    // Push each thread JoinHandle to the vector
    }
    for handle in handles {
        let _thread_ret = if let Ok(i) = handle.join() {i} else {-1} ;    // Wait until each thread finish. handle.join returns Result<T>
        println!("  [KILL] Thread {_thread_ret}");           
    }

    // ---------------------------------------------------
    // Access mutable data in threads
    // If not need mut part, use the same without Mutex and lock mechanism
    use std::sync::{Arc, Mutex};
    #[derive(Debug)] struct MySharedData { data: u32 }

    // Create a SharedData struct wrapped in a Mutex and then in an Arc
    let shared_data = Arc::new(Mutex::new(MySharedData { data: 0 }));

    // In the thread
    {
        // Create a clone of the Arc for the i thread
        let thread_data = Arc::clone(&shared_data); 
          
        // Lock data and update their value
        let mut locked_data = thread_data.lock().unwrap(); 
        locked_data.data += 10;
        // The lock is automatically released when `locked_data` goes out of scope
    }

    // Access global data at the end
    let _total_data = shared_data.lock().unwrap(); // if during spawn move is used then we need *shared_data

    // ---------------------------------------------------
    // Communicate between threads using channels
    println!("\nUse Channels to send data... ");

    use std::sync::mpsc;
    let (tx, rx) = mpsc::channel();

    // 
    let send_val: i32 = 32;
    println!(" > Send data [{send_val}] to tx");
    tx.send(send_val).unwrap();   

    let recv_val = rx.recv().unwrap();
    println!(" > Received data [{recv_val}] from rx");

}

fn time(){
    println!("\n======================================================================");
    println!("* TIME");

    // Measure time
    use std::{ thread, time::{Duration, Instant}};           // Includes
    let start = Instant::now();                     // Start time
    thread::sleep(Duration::from_millis(250));          // Delay
    let time_elapsed = start.elapsed().as_millis();   // Calculate time elapsed
    println!("Time elapsed: {}ms", time_elapsed);
}


// ---------------------------------------------------------------------------
// ---------------------------------------------------------------------------
// ---------------------------------------------------------------------------

fn main() {
    library_docs_following();
    library_docs_envlosing();
    // --------------------------
    formatted_print();
    variable_binding();
    // --------------------------
    strings();
    var_shadowing();
    functions();
    macros();
    conversion();
    destructoring_dereferencing();
    binding();
    let_else();
    // --------------------------
    arrays();
    tupples();
    slices();
    vectors();
    structs();
    enums();
    generics();
    linked_list();
    // --------------------------
    control_flow();
    iterators();
    crates();
    attributes();
    // --------------------------
    oop();
    error_handling();
    // --------------------------
    ownership();
    borrowing();
    lifetimes();
    // --------------------------
    execute_system_cmd();
    user_input();
    tests();
    threads();
    time();
}
