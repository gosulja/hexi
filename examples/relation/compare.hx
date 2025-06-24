val num1 = io::input("first number: ")
val num2 = io::input("second number: ")

val real1 = string::to_number(num1)
val real2 = string::to_number(num2)

io::println(num1, "==", num2, ":", (real1 == real2))
io::println(num1, "<", num2, ":", (real1 < real2))
io::println(num1, ">", num2, ":", (real1 > real2))
io::println(num1, "<=", num2, ":", (real1 <= real2))
io::println(num1, ">=", num2, ":", (real1 >= real2))
io::println(num1, "!=", num2, ":", (real1 != real2))
