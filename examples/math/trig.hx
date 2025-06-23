val pi = 3.14159
val angle_deg = 45
val angle_rad = angle_deg * pi / 180
val sin_val = math::sin(angle_rad)
val cos_val = math::cos(angle_rad)
val tan_val = sin_val / cos_val
io::println(sin_val)
io::println(cos_val)
io::println(tan_val)