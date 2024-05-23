package main

import (
	"math"
	"testing"
)

func TestIterPow(t *testing.T) {
	var testNum float64
	var n = 2.0
	for i := 2; i < 51; i++ {
		testNum = pow_iter(n, float64(i))
		expected := math.Pow(n, float64(i))

		if float64(testNum) != expected {
			t.Errorf("ожидалось число %.1f, получено %.1f", expected, testNum)
			return
		}
	}
}
