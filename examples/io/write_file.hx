val path = "hello.txt"
val content = "this is the content to be written to"
val success = io::write_file(path, content)
io::println(success)