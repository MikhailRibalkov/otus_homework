package main

import (
	"fmt"
	"testing"
)

func TestIterFibonacci(t *testing.T) {
	var expected int64 = 21
	var num int64 = 10

	result := iter_fibonacci(num)

	if expected != result {
		t.Errorf("expected %d, received %d", expected, result)
	}
}

func TestRecursiveFibonacci(t *testing.T) {
	var expected int64 = 21
	var num int64 = 10
	result := recursive_fibonacci(num)

	if expected != result {
		t.Errorf("expected %d, received %d", expected, result)
	}

	fmt.Println("fibonacci ", num, " = ", result)
}
