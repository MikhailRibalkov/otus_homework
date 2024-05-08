package main

import "fmt"

const (
	COUNT_SUMS = 28
)

var sums = make([]int, COUNT_SUMS)

// инициализируем массив сумм
func InitSums(sum []int) {
	for i := 0; i < COUNT_SUMS; i++ {
		sum[i] = 0
	}
}

// обрабатываем трехзначное число
func PerformNumber(s []int, number int) {
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
		s[sum] += 1
	}
}

// вычисляем общее количество счастливых билетов
func GetFullCount(sum []int, count_sums int) int {
	var count int = 0

	for i := 0; i < count_sums; i++ {
		count += sum[i] * sum[i]
	}
	return count
}

func main() {
	InitSums(sums)

	for number := 0; number < 1000; number++ {
		PerformNumber(sums, number)
	}

	fmt.Printf("happy ticket count: %d\n", GetFullCount(sums, COUNT_SUMS))
}
