io::println("super secure admin panel")
val name = io::input("username> ")

if name == "blinx" {
    io::println("you are an admin! but please enter ur password")
    val password = io::input("password> ")
    if password == "password123" {
        io::println("access granted")
    } else {
        io::println("access denied!")
    }
}