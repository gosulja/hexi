val age = string::to_number(io::input("enter your age: "))

if age >= 18 {
    io::println("you are eligible to vote!")
} else {
    io::println("you are not eligible to vote!")
}

if age != 18 {
    io::println("you aren't exactly 18")
} else {
    io::println("you are 18!")
}