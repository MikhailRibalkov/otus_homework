package main

import "fmt"

const (
	COUNT_SUMS = 28
)

var sums = make([]int, COUNT_SUMS)

// инициализируем массив сумм
func initSums() {
	for i := 0; i < COUNT_SUMS; i++ {
		sums[i] = 0
	}
}

// обрабатываем трехзначное число
func performNumber(number int) {
	var sum int = 0
	var value int = number
	var digit int

	digit = value / 100
	sum += digit

	value %= 100
	digit = value / 10
	sum += digit

	value %= 10
	sum += value

	if sum < COUNT_SUMS {
		sums[sum] += 1
	}
}

// вычисляем общее количество счастливых билетов
func getFullCount() int {
	var count int = 0

	for i := 0; i < COUNT_SUMS; i++ {
		count += sums[i] * sums[i]
	}
	return count
}

func main() {
	initSums()

	for number := 0; number < 1000; number++ {
		performNumber(number)
	}

	fmt.Printf("happy ticket count: %d\n", getFullCount())
}
