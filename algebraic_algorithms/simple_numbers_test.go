package main

import "testing"

func TestSimpleNumber(t *testing.T) {

	for q := 1000; q <= 1000000000; q *= 10 {
		t.Log("n = ", q, " simple numbers = ", Eratosphene(q))
	}
}
