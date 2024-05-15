package main

import (
	"testing"
)

func TestLuckyTickets6(t *testing.T) {
	expected := 55252

	result := LuckyTickets6()

	if expected != result {
		t.Errorf("Ожидалось число 55252, получено %d", result)
	}

}
