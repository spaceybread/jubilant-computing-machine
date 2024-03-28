Hamming codes 

Error checking in $2^n$ bit message packets: 

[hamming16.rs](https://github.com/spaceybread/jubilant-computing-machine/blob/main/hamming16.rs): first naive attempt at writing a checking and fixing algorithm for $16$-bit messages

[slowhamming.rs](https://github.com/spaceybread/jubilant-computing-machine/blob/main/slowhamming.rs): extension of the previous algorithm to work for $256$-bit message blocks

I did some more reading and finished the 3B1B videos on this topic and realised that all I had to do was XOR across the array and as such: 

[hamming.rs](https://github.com/spaceybread/jubilant-computing-machine/blob/main/hamming.rs): a much more efficient and clean algorithm to check and fix Hamming codes for $256$-bit message blocks


