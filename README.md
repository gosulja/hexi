# hexc
The Hex language compiler & interpreter.

# Changes and Features
- `date: 22/06/2025`
* Native function such as `print("hey");`
* Variable declarations: `val x = 5;`
* REPL
* Fixed identifier parser.
* Created HashMap for keywords such as `val`

- `date: 23/06/2025`
* Added `stdlib` with modules.
* Modules consist of: `io`, `math` and `string`
* Cleaned up syntax, semi colons are no longer needed.
* Improved parsing.
* Added `::` operator which allows access to module consts or functions.
## example
```
val name = io::input("name>> ")
io::print(name)
```
```
name>> blinx
blinx
```