fn main() {
    //scalar and compounds
    //rust is statically typed
    let variable: u32 = "42".parse().expect("not a number!");
    println!("\nvariable: {}", variable);

    //scalar type -> single value
    // 1-integer
    // 2-floating
    // 3-boolean
    // 4-char

    //integers (default -> i32 (best performance on modern cpus))
    /*
        8bit      i8        u8
        16bit     i16       u16
        32bit     i32*      u32
        64bit     i64       u64
        128bit    i128      u128
        arch      isize     usize
    */
    //2's complement
    //from -(2^n-1) or 0 to 2^n-1 inclusive
    //n num of bytes
    //isize and usize depend on cpu architecture
    /*
        decimal:    98_222 (_ visual separator)
        hex:        0xff
        octal:      0o77
        binary:     0b1111_0000
        byte:       b'A' (only u8)
    */
    //in case of runtime error (panic) -> integer overflow error
    //in --release compile mode the program doesn't crash because rust
    //performs a "wrap around" conversion (formally an error)
    //wrap around u8 example: 256 -> 0, 257 -> 1, etc...
    //for explicit manually wrapping use the Wrapping std library

    //floating point (default -> f64 (roughtly same performance of f32 on modern cpus))
    /*
        32bit     f32 (single precision)
        64bit     f64* (double precision)
    */
    //represented according to the IEEE-754 standard
    let x = 2.12121212121212121212;
    let y: f32 = 2.12121212121212121212;
    println!("\ny f32: {}", y);
    println!("x f64: {}", x);

    //numeric operations
    //addition
    let sum = 5 + 10;
    println!("\nsum: {}", sum);
    //subtraction
    let diff = 95.5 - 4.3;
    println!("diff: {}", diff);
    //multiplication
    let multi = 4 * 30;
    println!("multi: {}", multi);
    //division
    let quot = 56.7 / 32.2;
    println!("quot: {}", quot);
    //remainder
    let rem = 43 % 5;
    println!("rem: {}", rem);

    //boolean
    //1 byte size (true, false)
    let t = true;
    let f: bool = false; //explicit type annotation
    println!("\nt: {}", t);
    println!("f: {}", f);

    //char
    //single quotes '', string literals double quotes ""
    //unicode scalar value -> more than ascii
    //(accented letters,chinese,japanese,korean,emoji,zero-width spaces,...)
    let c = 'z';
    let z = 'ℤ';
    let emoji = '😀';
    println!("\nc: {}", c);
    println!("z: {}", z);
    println!("emoji: {}", emoji);

    //compound type -> multiple value
    // 1-tuple
    // 2-array

    //tuple
    //multiple types values
    //fixed lenght
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    //destructuring tuple and print
    let (a, b, c) = tup;
    println!("\ntup (destructuring): ({}, {}, {})", a, b, c);
    //pattern matching for selecting tuple elements
    //first index is 0
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    println!(
        "tup (pattern matching): ({}, {}. {})",
        five_hundred, six_point_four, one
    );

    //array
    //single type values
    //fixed lenght
    //values on a stack
    //not flexible like the vector type
    let nar = [1, 2, 3, 4, 5];
    let _snar: [i32; 5] = [1, 2, 3, 4, 5]; //[type, lenght]
    let dnar = [3; 5]; //[values, lenght] -> [3,3,3,3,3]
    let _sar = [
        "jan", "feb", "mar", "apr", "may", "jun", "jul", "aug", "sep", "oct", "nov", "dec",
    ];
    //static print
    let frs = nar[0];
    let sec = nar[1];
    let thr = nar[2];
    let frt = nar[3];
    let fft = nar[4];
    //access by index
    let index = 4;
    let element = dnar[index];
    println!("\narr: [{}, {}, {}, {}, {}]", frs, sec, thr, frt, fft);
    println!("dnar indexing: [{}]", element);
    //access an element of an array that past the end of the array
    //cause a runtime error (panic)
    //rust compare the index given with the lenght of the array
    //if the index is greater than or equal to the array lenght,rust will panic
}
