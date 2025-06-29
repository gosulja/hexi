include json

val data = '{"name": "bob", "age": 22}'
val parsed = json::parse(data)

io::print(parsed.name)
io::print(parsed.age)