val v0 = 20
val theta = 45
val g = 9.81
val pi = 3.14159
val theta_rad = theta * pi / 180
val sin_theta = math::sin(theta_rad)
val v0_y = v0 * sin_theta
val max_height = math::pow(v0_y, 2) / (2 * g)
io::println(max_height)