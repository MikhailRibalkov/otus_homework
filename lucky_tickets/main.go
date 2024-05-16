package main

import "fmt"

func LuckyTickets6() int {
	var count = 0
	for a1 := 0; a1 < 10; a1++ {
		for a2 := 0; a2 < 10; a2++ {
			for a3 := 0; a3 < 10; a3++ {
				sumA := a1 + a2 + a3
				for b1 := 0; b1 < 10; b1++ {
					for b2 := 0; b2 < 10; b2++ {
						b3 := sumA - (b1 + b2)
						if b3 >= 0 && b3 < 10 {
							count++
						}
					}
				}
			}
		}
	}
	return count
}

type LuckyTickets struct {
	count int64
}

func (lt *LuckyTickets) Next(n int, sumA int, sumB int) {
	if n == 0 {
		if sumA == sumB {
			lt.count++
		}
		return
	}

	for a := 0; a < 10; a++ {
		for b := 0; b < 10; b++ {
			lt.Next(n-1, sumA+a, sumB+b)
		}
	}
}

func (Lt *LuckyTickets) LuckyTicketsCount(N int) int64 {
	Lt.count = 0
	Lt.Next(N, 0, 0)
	return Lt.count
}

func main() {
	lucky := LuckyTickets{}
	fmt.Println(lucky.LuckyTicketsCount(6))
}
