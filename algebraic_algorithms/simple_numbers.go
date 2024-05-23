package main

import "math"

func IsPrime(n int) bool {
	sqrt := int(math.Sqrt(float64(n)))
	if n == 2 {
		return true
	}

	if n%2 == 0 {
		return false
	}

	for i := 3; i <= sqrt; i += 2 {
		if n%i == 0 {
			return false
		}
	}
	return true
}

func SimpleNumbers(num int) int {
	count := 0
	// side_num := int(math.Sqrt(float64(num)))

	for i := 2; i <= num; i++ {
		if IsPrime(i) {
			count++
		}
	}
	return count
}

func Eratosphene(n int) int {
	var divs []bool = make([]bool, n+1)
	count := 0
	sqrt := int(math.Sqrt(float64(n)))

	for i := 2; i <= n; i++ {
		if !divs[i] {
			count++
			if i <= sqrt {
				for j := i * i; j <= n; j += i {
					divs[j] = true
				}
			}
		}
	}
	return count
}
