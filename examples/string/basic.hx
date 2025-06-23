val text = "  Hello World  ";
val clean = string::trim(text);
val upper = string::upper(clean);
io::println(upper);

if string::contains("hello world", "world") {
    io::println("Found world!");
}