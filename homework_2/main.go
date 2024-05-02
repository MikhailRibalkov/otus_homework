package main

import (
	"fmt"
)

func isLucky(ticket []int) int {
	max_number := ticket

	var sum1 int = 0
	var sum2 int = 0
	len_number := len(max_number)
	for i := 0; i < len_number/2; i++ {
		sum1 += max_number[i]
		sum2 += max_number[len_number-1-i]
	}

	if sum1 != sum2 {
		return 0
	}
	return 1
}

func lucky_count(ticket int, len int) int {
	if ticket == 0 {
		return 1
	}

	current_ticket := ticket
	split_ticket := make([]int, 0)

	for i := 0; i < len; i++ {
		split_ticket = append(split_ticket, current_ticket%10)
		current_ticket = current_ticket / 10
	}
	return isLucky(split_ticket) + lucky_count(ticket-1, len)
}

func lucky_tickets(len_num int) int {
	max_number := 0

	for i := 0; i < int(len_num); i++ {
		max_number += max_number*10 + 9
	}

	return lucky_count(max_number, len_num)
}

func main() {
	tickets_count := lucky_tickets(6)
	fmt.Println("Lucky tickets count: ", tickets_count)
}
