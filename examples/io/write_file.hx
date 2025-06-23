var path = "hello.txt"
var content = "this is the content to be written to"
var success = io::write_file(path, content)
io::println(success)