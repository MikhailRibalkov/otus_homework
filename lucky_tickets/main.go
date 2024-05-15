package main

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

func main() {

}
