val principal = 1000
val rate = 0.05
val time = 10
val compound_factor = math::pow(1 + rate, time)
val final_amount = principal * compound_factor
io::println(final_amount)