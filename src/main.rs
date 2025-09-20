fn main() {
    println!("=== Rust Language Primitives Exercise ===\n");
    
    // Basic Data Types
    demonstrate_basic_types();
    
    // String Types
    demonstrate_strings();
    
    // Collections
    demonstrate_collections();
    
    // Custom Types
    demonstrate_custom_types();
    
    // Control Flow
    demonstrate_control_flow();
    
    // Functions
    demonstrate_functions();
    
    // Ownership and Borrowing
    demonstrate_ownership();
    
    // Error Handling
    demonstrate_error_handling();
    
    println!("\n=== Exercise Complete! ===");
}

/// Demonstrates basic primitive data types in Rust
fn demonstrate_basic_types() {
    println!("1. Basic Data Types:");
    
    // Integer types
    let small_int: i8 = 127;
    let big_int: i64 = 9_223_372_036_854_775_807;
    let unsigned: u32 = 4_294_967_295;
    
    println!("  Integers: i8={}, i64={}, u32={}", small_int, big_int, unsigned);
    
    // Floating point types
    let pi: f64 = 3.141592653589793;
    let e: f32 = 2.718;
    
    println!("  Floats: f64={:.3}, f32={:.3}", pi, e);
    
    // Boolean type
    let is_learning: bool = true;
    let is_expert: bool = false;
    
    println!("  Booleans: learning={}, expert={}", is_learning, is_expert);
    
    // Character type (Unicode scalar value)
    let letter: char = 'R';
    let emoji: char = '🦀';
    let chinese: char = '中';
    
    println!("  Characters: letter='{}', emoji='{}', chinese='{}'", letter, emoji, chinese);
    
    // Type inference
    let inferred = 42; // Rust infers i32
    println!("  Type inference: inferred={} (i32)", inferred);
    
    println!();
}

/// Demonstrates string types and string manipulation
fn demonstrate_strings() {
    println!("2. String Types:");
    
    // String literals (string slices)
    let string_slice: &str = "Hello, Rust!";
    println!("  String slice: \"{}\"", string_slice);
    
    // Owned strings
    let mut owned_string: String = String::from("Rust");
    println!("  Owned string: \"{}\"", owned_string);
    
    // String manipulation
    owned_string.push_str(" is awesome!");
    println!("  After push_str: \"{}\"", owned_string);
    
    owned_string.push('🚀');
    println!("  After push char: \"{}\"", owned_string);
    
    // String formatting
    let name = "Rustacean";
    let age = 15; // Rust's age in years
    let formatted = format!("{} is {} years old", name, age);
    println!("  Formatted string: \"{}\"", formatted);
    
    // String slicing
    let slice = &owned_string[0..4];
    println!("  String slice [0..4]: \"{}\"", slice);
    
    println!();
}

/// Demonstrates arrays, vectors, and tuples
fn demonstrate_collections() {
    println!("3. Collections:");
    
    // Arrays (fixed size, same type)
    let array: [i32; 5] = [1, 2, 3, 4, 5];
    println!("  Array: {:?}", array);
    println!("  Array length: {}", array.len());
    println!("  First element: {}", array[0]);
    
    // Array with repeated values
    let zeros = [0; 3]; // [0, 0, 0]
    println!("  Array of zeros: {:?}", zeros);
    
    // Vectors (dynamic arrays)
    let mut vector: Vec<i32> = vec![10, 20, 30];
    println!("  Vector: {:?}", vector);
    
    vector.push(40);
    vector.push(50);
    println!("  After pushing: {:?}", vector);
    
    let popped = vector.pop();
    println!("  Popped element: {:?}, Vector: {:?}", popped, vector);
    
    // Tuples (different types)
    let tuple: (i32, f64, char, &str) = (42, 3.14, 'x', "tuple");
    println!("  Tuple: {:?}", tuple);
    
    // Destructuring tuples
    let (a, b, c, d) = tuple;
    println!("  Destructured: a={}, b={}, c={}, d={}", a, b, c, d);
    
    // Accessing tuple elements
    println!("  Tuple.0: {}, Tuple.1: {}", tuple.0, tuple.1);
    
    // Unit type (empty tuple)
    let unit: () = ();
    println!("  Unit type: {:?}", unit);
    
    println!();
}

/// Demonstrates structs and enums
fn demonstrate_custom_types() {
    println!("4. Custom Types:");
    
    // Struct definition and usage
    #[derive(Debug)]
    struct Person {
        name: String,
        age: u32,
        is_active: bool,
    }
    
    let person = Person {
        name: String::from("Alice"),
        age: 30,
        is_active: true,
    };
    
    println!("  Struct: {:?}", person);
    println!("  Person name: {}, age: {}", person.name, person.age);
    
    // Tuple struct
    #[derive(Debug)]
    struct Point(i32, i32, i32);
    
    let origin = Point(0, 0, 0);
    println!("  Tuple struct: {:?}", origin);
    println!("  Point coordinates: ({}, {}, {})", origin.0, origin.1, origin.2);
    
    // Unit struct
    #[derive(Debug)]
    struct UnitStruct;
    
    let unit_struct = UnitStruct;
    println!("  Unit struct: {:?}", unit_struct);
    
    // Enum with different variants
    #[derive(Debug)]
    enum Color {
        Red,
        Green,
        Blue,
        Rgb(u8, u8, u8),
        Hsv { hue: f64, saturation: f64, value: f64 },
    }
    
    let colors = [
        Color::Red,
        Color::Rgb(255, 0, 0),
        Color::Hsv { hue: 0.0, saturation: 1.0, value: 1.0 },
    ];
    
    for (i, color) in colors.iter().enumerate() {
        println!("  Color {}: {:?}", i + 1, color);
    }
    
    println!();
}

/// Demonstrates control flow constructs
fn demonstrate_control_flow() {
    println!("5. Control Flow:");
    
    let number = 7;
    
    // if/else expressions
    let description = if number < 5 {
        "small"
    } else if number < 10 {
        "medium"
    } else {
        "large"
    };
    
    println!("  Number {} is {}", number, description);
    
    // loop with break and continue
    println!("  Loop with break:");
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 3 {
            continue;
        }
        if counter == 5 {
            break counter * 2;
        }
        println!("    Iteration: {}", counter);
    };
    println!("  Loop result: {}", result);
    
    // while loop
    println!("  While loop:");
    let mut countdown = 3;
    while countdown > 0 {
        println!("    Countdown: {}", countdown);
        countdown -= 1;
    }
    println!("    Blast off! 🚀");
    
    // for loops
    println!("  For loop with range:");
    for i in 1..=5 {
        println!("    For loop: {}", i);
    }
    
    println!("  For loop with iterator:");
    let items = vec!["apple", "banana", "cherry"];
    for (index, item) in items.iter().enumerate() {
        println!("    Item {}: {}", index, item);
    }
    
    // match expression
    let value = 3;
    let result = match value {
        1 => "one",
        2 => "two",
        3 => "three",
        4..=10 => "between four and ten",
        _ => "something else",
    };
    println!("  Match result for {}: {}", value, result);
    
    println!();
}

/// Demonstrates functions and closures
fn demonstrate_functions() {
    println!("6. Functions:");
    
    // Regular function call
    let sum = add_numbers(5, 3);
    println!("  Function result: add_numbers(5, 3) = {}", sum);
    
    // Function with multiple return values
    let (quotient, remainder) = divide_with_remainder(17, 5);
    println!("  Division: 17 ÷ 5 = {} remainder {}", quotient, remainder);
    
    // Function with no return value (returns unit type)
    print_greeting("Rustacean");
    
    // Closures (anonymous functions)
    let multiply = |x: i32, y: i32| x * y;
    let product = multiply(4, 6);
    println!("  Closure result: multiply(4, 6) = {}", product);
    
    // Closure capturing environment
    let factor = 10;
    let scale = |x: i32| x * factor;
    println!("  Closure with capture: scale(5) = {}", scale(5));
    
    // Higher-order functions
    let numbers = vec![1, 2, 3, 4, 5];
    let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
    println!("  Map function: {:?} -> {:?}", numbers, doubled);
    
    let even_numbers: Vec<&i32> = numbers.iter().filter(|&&x| x % 2 == 0).collect();
    println!("  Filter function: even numbers from {:?} = {:?}", numbers, even_numbers);
    
    println!();
}

/// Helper function for addition
fn add_numbers(a: i32, b: i32) -> i32 {
    a + b // Implicit return (no semicolon)
}

/// Helper function that returns multiple values
fn divide_with_remainder(dividend: i32, divisor: i32) -> (i32, i32) {
    (dividend / divisor, dividend % divisor)
}

/// Helper function with no return value
fn print_greeting(name: &str) {
    println!("  Hello, {}!", name);
}

/// Demonstrates ownership, borrowing, and lifetimes
fn demonstrate_ownership() {
    println!("7. Ownership and Borrowing:");
    
    // Ownership transfer (move)
    let s1 = String::from("Hello");
    let s2 = s1; // s1 is moved to s2
    // println!("{}", s1); // This would cause a compile error
    println!("  Moved string: {}", s2);
    
    // Clone to avoid move
    let s3 = String::from("World");
    let s4 = s3.clone(); // Explicit copy
    println!("  Original: {}, Clone: {}", s3, s4);
    
    // Copy types (stack data)
    let x = 5;
    let y = x; // Copy, not move
    println!("  Copy types: x = {}, y = {}", x, y);
    
    // Borrowing (references)
    let mut message = String::from("Rust");
    
    // Immutable borrow
    let len = calculate_length(&message);
    println!("  String '{}' has length {}", message, len);
    
    // Mutable borrow
    append_exclamation(&mut message);
    println!("  After mutation: '{}'", message);
    
    // Multiple immutable borrows are allowed
    let r1 = &message;
    let r2 = &message;
    println!("  Multiple immutable borrows: '{}' and '{}'", r1, r2);
    
    // Slice borrowing
    let slice = &message[0..4];
    println!("  String slice: '{}'", slice);
    
    println!();
}

/// Helper function that borrows a string
fn calculate_length(s: &String) -> usize {
    s.len()
}

/// Helper function that mutably borrows a string
fn append_exclamation(s: &mut String) {
    s.push('!');
}

/// Demonstrates error handling with Option and Result
fn demonstrate_error_handling() {
    println!("8. Error Handling:");
    
    // Option<T> for values that might be None
    let numbers = vec![1, 2, 3, 4, 5];
    
    match numbers.get(2) {
        Some(value) => println!("  Element at index 2: {}", value),
        None => println!("  No element at index 2"),
    }
    
    match numbers.get(10) {
        Some(value) => println!("  Element at index 10: {}", value),
        None => println!("  No element at index 10"),
    }
    
    // Using if let for Option
    if let Some(first) = numbers.first() {
        println!("  First element: {}", first);
    }
    
    // Result<T, E> for operations that can fail
    let valid_number = parse_number("42");
    match valid_number {
        Ok(num) => println!("  Parsed number: {}", num),
        Err(err) => println!("  Parse error: {}", err),
    }
    
    let invalid_number = parse_number("abc");
    match invalid_number {
        Ok(num) => println!("  Parsed number: {}", num),
        Err(err) => println!("  Parse error: {}", err),
    }
    
    // Using unwrap_or for default values
    let result1 = parse_number("123").unwrap_or(0);
    let result2 = parse_number("xyz").unwrap_or(0);
    println!("  With unwrap_or: '123' -> {}, 'xyz' -> {}", result1, result2);
    
    // Chaining operations with map and and_then
    let doubled = parse_number("21")
        .map(|n| n * 2)
        .unwrap_or(0);
    println!("  Chained operations: parse('21') * 2 = {}", doubled);
    
    // The ? operator for early return
    match divide_numbers(10.0, 2.0) {
        Ok(result) => println!("  Division result: {}", result),
        Err(err) => println!("  Division error: {}", err),
    }
    
    match divide_numbers(10.0, 0.0) {
        Ok(result) => println!("  Division result: {}", result),
        Err(err) => println!("  Division error: {}", err),
    }
    
    println!();
}

/// Helper function that returns Result
fn parse_number(s: &str) -> Result<i32, String> {
    match s.parse::<i32>() {
        Ok(num) => Ok(num),
        Err(_) => Err(format!("'{}' is not a valid number", s)),
    }
}

/// Helper function demonstrating Result with division
fn divide_numbers(dividend: f64, divisor: f64) -> Result<f64, String> {
    if divisor == 0.0 {
        Err("Cannot divide by zero".to_string())
    } else {
        Ok(dividend / divisor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_numbers() {
        assert_eq!(add_numbers(2, 3), 5);
        assert_eq!(add_numbers(0, 0), 0);
        assert_eq!(add_numbers(-5, 3), -2);
    }

    #[test]
    fn test_divide_with_remainder() {
        let (quotient, remainder) = divide_with_remainder(17, 5);
        assert_eq!(quotient, 3);
        assert_eq!(remainder, 2);
        
        let (quotient, remainder) = divide_with_remainder(10, 3);
        assert_eq!(quotient, 3);
        assert_eq!(remainder, 1);
    }

    #[test]
    fn test_parse_number() {
        assert_eq!(parse_number("42"), Ok(42));
        assert_eq!(parse_number("0"), Ok(0));
        assert_eq!(parse_number("-123"), Ok(-123));
        
        assert!(parse_number("abc").is_err());
        assert!(parse_number("").is_err());
        assert!(parse_number("12.34").is_err());
    }

    #[test]
    fn test_divide_numbers() {
        assert_eq!(divide_numbers(10.0, 2.0), Ok(5.0));
        assert_eq!(divide_numbers(7.0, 2.0), Ok(3.5));
        
        assert!(divide_numbers(10.0, 0.0).is_err());
        assert_eq!(
            divide_numbers(10.0, 0.0).unwrap_err(),
            "Cannot divide by zero"
        );
    }

    #[test]
    fn test_calculate_length() {
        let test_string = String::from("Hello, Rust!");
        assert_eq!(calculate_length(&test_string), 12);
        
        let empty_string = String::new();
        assert_eq!(calculate_length(&empty_string), 0);
    }
}
