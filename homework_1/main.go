package main

import (
	"fmt"
)

func img1() {
	for x := 0; x < 24; x++ {
		fmt.Println("")
		for y := 0; y < 24; y++ {
			if x < y {
				fmt.Print("#")
			} else {
				fmt.Print(".")
			}
		}
	}
	fmt.Println("")
}

func main() {
	img1()
}
