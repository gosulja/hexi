val a = 3
val b = 4
val c = 5
val s = (a + b + c) / 2
val area_squared = s * (s - a) * (s - b) * (s - c)
val area = math::sqrt(area_squared)
io::println(area)