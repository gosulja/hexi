val nums = [ 1, 2, 3, 4, 5 ]

io::println( string::fmt("Nums before: {}", nums) )

nums.push(6)
nums.push(7)
nums.push(8)
nums.push(9)

io::println( string::fmt("Nums after: {}", nums) )