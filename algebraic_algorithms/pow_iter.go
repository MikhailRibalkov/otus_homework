package main

func pow_iter(num float64, pow float64) float64 {
	var result float64 = num
	for i := 1; i < int(pow); i++ {
		result *= num
	}
	return result
}
