fn check16(arr: [[i32; 4]; 4]) -> (bool, i32) {
    //checks if a 16 bit message block was recieved as expected

    let mut out: i32 = 0; 
    // Q1 
    let mut q1: i32 = 0;
    for n in 0..4 {
        if n != 0 {q1 += arr[n][1];}
        q1 += arr[n][3];
    }

    if q1 % 2 != arr[0][1] {
        out += 1;
    }

    // Q2 
    let mut q2: i32 = 0;
    for n in 0..4 {
        if n != 0 {q2 += arr[n][2];}
        q2 += arr[n][3];
    }

    if q2 % 2 != arr[0][2] {
        out += 2;
    }

    // Q3
    let mut q3: i32 = 0;
    for n in 0..4 {
        if n != 0 {q3 += arr[1][n];}
        q3 += arr[3][n];
    }
    
    if q3 % 2 != arr[1][0] {
        out += 4;
    }

    // Q4 
    let mut q4: i32 = 0;
    for n in 0..4 {
        if n != 0 {q4 += arr[2][n];}
        q4 += arr[3][n];
    }
    
    if q4 % 2 != arr[2][0] {
        out += 8;
    }

    if out != 0 {
        return (false, out);
    }
    return (true, out);
}

fn build16(mut arr: [[i32; 4]; 4]) -> [[i32; 4]; 4] {
    //sets the parity bits of a 16 bit message block
    
    // Q1 
    let mut q1: i32 = 0;
    for n in 0..4 {
        if n != 0 {q1 += arr[n][1];}
        q1 += arr[n][3];
    }
    arr[0][1] = q1 % 2;
    
    // Q2 
    let mut q2: i32 = 0;
    for n in 0..4 {
        if n != 0 {q2 += arr[n][2];}
        q2 += arr[n][3];
    }
    arr[0][2] = q2 % 2;

    // Q3
    let mut q3: i32 = 0;
    for n in 0..4 {
        if n != 0 {q3 += arr[1][n];}
        q3 += arr[3][n];
    }
    arr[1][0] = q3 % 2;

    // Q4 
    let mut q4: i32 = 0;
    for n in 0..4 {
        if n != 0 {q4 += arr[2][n];}
        q4 += arr[3][n];
    }
    arr[2][0] = q4 % 2;

    return arr;
}

fn main() {
    let mut test16: [[i32; 4]; 4] = [
        [0, 0, 0, 1],
        [0, 1, 0, 0],
        [0, 1, 1, 1],
        [1, 0, 1, 1],
    ];

    let mut ans: (bool, i32) = check16(test16);
    let mut ansbool = ans.0;
    let mut ans32 = ans.1;
    println!("{ansbool}: {ans32}");
    for row in test16.iter() {
        println!("{:?}", row);
    }

    test16 = build16(test16);
    ans = check16(test16);
    ansbool = ans.0;
    ans32 = ans.1;
    println!("{ansbool}: {ans32}");
    for row in test16.iter() {
        println!("{:?}", row);
    }
}