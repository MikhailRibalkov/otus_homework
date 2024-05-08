package main

import (
	"testing"
)

func TestLuckyTickets(t *testing.T) {
	expected := 55252
	count_sums := 28
	sums := make([]int, count_sums)

	InitSums(sums)

	for number := 0; number < 1000; number++ {
		PerformNumber(sums, number)
	}

	result := GetFullCount(sums, count_sums)

	if result != expected {
		t.Errorf("Error: got %d expected %d lucky tickets", result, expected)
	}
}
