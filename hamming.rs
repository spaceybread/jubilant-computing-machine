fn check256(arr: [[i32; 16]; 16]) -> (bool, i32) {
    let mut out: i32 = 0;

    //Q1
    let mut q1: i32 = 0; 
    for n in 0..16 {
        if n != 0 {q1 += arr[n][1];}
        q1 += arr[n][3];
        q1 += arr[n][5];
        q1 += arr[n][7];
        q1 += arr[n][9];
        q1 += arr[n][11];
        q1 += arr[n][13];
        q1 += arr[n][15];
    }

    if q1 % 2 != arr[0][1] {
        out += 1;
    }

    //Q5
    let mut q5: i32 = 0; 
    for n in 0..16 {
        if n != 0 {q5 += arr[1][n];}
        q5 += arr[3][n];
        q5 += arr[5][n];
        q5 += arr[7][n];
        q5 += arr[9][n];
        q5 += arr[11][n];
        q5 += arr[13][n];
        q5 += arr[15][n];
    }

    if q5 % 2 != arr[1][0] {
        out += 16;
    }

    //Q2
    let mut q2: i32 = 0; 
    for n in 0..16 {
        if n != 0 {q2 += arr[n][2];}
        q2 += arr[n][3];
        q2 += arr[n][6];
        q2 += arr[n][7];
        q2 += arr[n][10];
        q2 += arr[n][11];
        q2 += arr[n][14];
        q2 += arr[n][15];
    }

    if q2 % 2 != arr[0][2] {
        out += 2;
    }

    //Q6
    let mut q6: i32 = 0; 
    for n in 0..16 {
        if n != 0 {q6 += arr[2][n];}
        q6 += arr[3][n];
        q6 += arr[6][n];
        q6 += arr[7][n];
        q6 += arr[10][n];
        q6 += arr[11][n];
        q6 += arr[14][n];
        q6 += arr[15][n];
    }

    if q6 % 2 != arr[2][0] {
        out += 32;
    }

    //Q3
    let mut q3: i32 = 0; 
    for n in 0..16 {
        if n != 0 {q3 += arr[n][4];}
        q3 += arr[n][5];
        q3 += arr[n][6];
        q3 += arr[n][7];
        q3 += arr[n][12];
        q3 += arr[n][13];
        q3 += arr[n][14];
        q3 += arr[n][15];
    }

    if q3 % 2 != arr[0][4] {
        out += 4;
    }

    //Q7
    let mut q7: i32 = 0; 
    for n in 0..16 {
        if n != 0 {q7 += arr[4][n];}
        q7 += arr[5][n];
        q7 += arr[6][n];
        q7 += arr[7][n];
        q7 += arr[10][n];
        q7 += arr[11][n];
        q7 += arr[14][n];
        q7 += arr[15][n];
    }

    if q7 % 2 != arr[4][0] {
        out += 64;
    }


    //Q4
    let mut q4: i32 = 0; 
    for n in 0..16 {
        if n != 0 {q4 += arr[n][8];}
        q4 += arr[n][9];
        q4 += arr[n][10];
        q4 += arr[n][11];
        q4 += arr[n][12];
        q4 += arr[n][13];
        q4 += arr[n][14];
        q4 += arr[n][15];
    }

    if q4 % 2 != arr[0][8] {
        out += 8;
    }

    //Q8
    let mut q8: i32 = 0; 
    for n in 0..16 {
        if n != 0 {q8 += arr[8][n];}
        q8 += arr[9][n];
        q8 += arr[10][n];
        q8 += arr[11][n];
        q8 += arr[12][n];
        q8 += arr[13][n];
        q8 += arr[14][n];
        q8 += arr[15][n];
    }

    if q8 % 2 != arr[8][0] {
        out += 128;
    }

    if out != 0 {
        return (false, out);
    }
    return (true, out);
}

fn build256(mut arr: [[i32; 16]; 16]) -> [[i32; 16]; 16] {
    
    //Q1
    let mut q1: i32 = 0; 
    for n in 0..16 {
        if n != 0 {q1 += arr[n][1];}
        q1 += arr[n][3];
        q1 += arr[n][5];
        q1 += arr[n][7];
        q1 += arr[n][9];
        q1 += arr[n][11];
        q1 += arr[n][13];
        q1 += arr[n][15];
    }

    arr[0][1] = q1 % 2;

    //Q5
    let mut q5: i32 = 0; 
    for n in 0..16 {
        if n != 0 {q5 += arr[1][n];}
        q5 += arr[3][n];
        q5 += arr[5][n];
        q5 += arr[7][n];
        q5 += arr[9][n];
        q5 += arr[11][n];
        q5 += arr[13][n];
        q5 += arr[15][n];
    }
    arr[1][0] = q5 % 2;

    //Q2
    let mut q2: i32 = 0; 
    for n in 0..16 {
        if n != 0 {q2 += arr[n][2];}
        q2 += arr[n][3];
        q2 += arr[n][6];
        q2 += arr[n][7];
        q2 += arr[n][10];
        q2 += arr[n][11];
        q2 += arr[n][14];
        q2 += arr[n][15];
    }
    arr[0][2] = q2 % 2;

    //Q6
    let mut q6: i32 = 0; 
    for n in 0..16 {
        if n != 0 {q6 += arr[2][n];}
        q6 += arr[3][n];
        q6 += arr[6][n];
        q6 += arr[7][n];
        q6 += arr[10][n];
        q6 += arr[11][n];
        q6 += arr[14][n];
        q6 += arr[15][n];
    }
    arr[2][0] = q6 % 2;

    //Q3
    let mut q3: i32 = 0; 
    for n in 0..16 {
        if n != 0 {q3 += arr[n][4];}
        q3 += arr[n][5];
        q3 += arr[n][6];
        q3 += arr[n][7];
        q3 += arr[n][12];
        q3 += arr[n][13];
        q3 += arr[n][14];
        q3 += arr[n][15];
    }
    arr[0][4] = q3 % 2;

    //Q7
    let mut q7: i32 = 0; 
    for n in 0..16 {
        if n != 0 {q7 += arr[4][n];}
        q7 += arr[5][n];
        q7 += arr[6][n];
        q7 += arr[7][n];
        q7 += arr[10][n];
        q7 += arr[11][n];
        q7 += arr[14][n];
        q7 += arr[15][n];
    }
    arr[4][0] = q7 % 2;

    //Q4
    let mut q4: i32 = 0; 
    for n in 0..16 {
        if n != 0 {q4 += arr[n][8];}
        q4 += arr[n][9];
        q4 += arr[n][10];
        q4 += arr[n][11];
        q4 += arr[n][12];
        q4 += arr[n][13];
        q4 += arr[n][14];
        q4 += arr[n][15];
    }
    arr[0][8] = q4 % 2;

    //Q8
    let mut q8: i32 = 0; 
    for n in 0..16 {
        if n != 0 {q8 += arr[8][n];}
        q8 += arr[9][n];
        q8 += arr[10][n];
        q8 += arr[11][n];
        q8 += arr[12][n];
        q8 += arr[13][n];
        q8 += arr[14][n];
        q8 += arr[15][n];
    }
    arr[8][0] = q8 % 2;
    
    // extended Hamming check
    let mut parity: i32 = 0;
    for r in arr.iter() {
        for e in r.iter() {
            parity += e;
        }
    }
    
    arr[0][0] = parity % 2;

    return arr;
}

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

    let mut parity: i32 = 0;
    for r in arr.iter() {
        for e in r.iter() {
            parity += e;
        }
    }
    
    arr[0][0] = parity % 2;

    return arr;
}

fn hamming16() {
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

fn hamming256() {
    let mut test256: [[i32; 16]; 16] = [
        [0, 1, 0, 1, 1, 0, 0, 1, 0, 1, 1, 0, 1, 1, 1, 1], // 0
        [0, 1, 1, 1, 0, 1, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0], // 1
        [0, 1, 1, 0, 0, 1, 0, 0, 0, 1, 1, 0, 0, 1, 0, 1], // 2
        [0, 1, 1, 0, 0, 0, 1, 1, 0, 1, 1, 0, 1, 1, 1, 1], // 3
        [0, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 1, 0, 1], // 4
        [0, 1, 1, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0], // 5
        [0, 1, 1, 0, 0, 0, 0, 1, 0, 1, 1, 0, 1, 1, 1, 0], // 6 
        [0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 1, 0, 1], // 7 
        [0, 1, 1, 0, 0, 0, 0, 1, 0, 1, 1, 1, 0, 0, 1, 1], // 8
        [0, 1, 1, 1, 0, 1, 0, 0, 0, 1, 1, 0, 0, 1, 0, 1], // 9  
        [0, 1, 1, 1, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0], // 10
        [0, 1, 1, 0, 0, 1, 0, 1, 0, 1, 1, 0, 0, 1, 1, 1], // 11 
        [0, 1, 1, 0, 0, 1, 1, 1, 0, 0, 1, 0, 1, 1, 1, 0], // 12 
        [0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 1, 1, 0], // 13  
        [0, 1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 0, 0, 0, 1, 1], // 14
        [0, 1, 1, 0, 0, 1, 0, 1, 0, 0, 1, 0, 0, 0, 0, 1], // 15  
    ];

    let mut ans: (bool, i32) = check256(test256);
    let mut ansbool = ans.0;
    let mut ans32 = ans.1;
    println!("{ansbool}: {ans32}"); // expected: "false: 82"
    let mut i:i32 = 0; 
    for row in test256.iter() {
        for c in row.iter() {
            print!("{:?}", c);
            i += 1;
            if i % 16 == 0 {
                println!();
            }
        } 
    }

    test256 = build256(test256);
    ans = check256(test256);
    ansbool = ans.0;
    ans32 = ans.1;
    println!("{ansbool}: {ans32}"); // expected: "true: 0"
    for row in test256.iter() {
        for c in row.iter() {
            print!("{:?}", c);
            i += 1;
            if i % 16 == 0 {
                println!();
            }
        } 
    }

}

fn main() {
    hamming16();
    hamming256();
}