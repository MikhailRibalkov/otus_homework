package main

func iter_fibonacci(n int64) int64 {
	var f0 int64 = 0
	var f1 int64 = 1
	var f2 int64
	for i := 2; i < int(n); i++ {
		f2 = f0 + f1
		f0 = f1
		f1 = f2
	}

	return f2
}

func recursive_fibonacci(n int64) int64 {
	if n <= 1 {
		return n
	}
	return recursive_fibonacci(n-1) + recursive_fibonacci(n-2)
}
