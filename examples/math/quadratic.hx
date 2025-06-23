val a = 2
val b = -7
val c = 3
val discriminant = math::pow(b, 2) - 4 * a * c
io::println(discriminant)
val sqrt_disc = math::sqrt(discriminant)
val root1 = ((-1 * b) + sqrt_disc) / (2 * a)
val root2 = ((-1 * b) - sqrt_disc) / (2 * a)
io::println(root1)
io::println(root2)