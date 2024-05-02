package main

import (
	"fmt"
)

func img1() {
	for x := 0; x < 25; x++ {
		fmt.Println("")
		for y := 0; y < 25; y++ {
			if x < y {
				fmt.Print("# ")
			} else {
				fmt.Print(". ")
			}
		}
	}
	fmt.Println("")
}

func img2() {
	for x := 0; x < 25; x++ {
		fmt.Println("")
		for y := 0; y < 25; y++ {
			if x == y {
				fmt.Print("# ")
			} else {
				fmt.Print(". ")
			}
		}
	}
	fmt.Println("")
}

func img3() {
	var element_count = 25
	for x := 0; x < element_count; x++ {
		fmt.Println("")
		for y := 0; y < element_count; y++ {
			if y == 24-x {
				fmt.Print("# ")
			} else {
				fmt.Print(". ")
			}
		}
	}
	fmt.Println("")
}

func img5() {
	var element_count = 25
	for x := 0; x < element_count; x++ {
		fmt.Println("")
		for y := 0; y < element_count; y++ {
			if x == y-x || x == y-x-1 {
				fmt.Print("# ")
			} else {
				fmt.Print(". ")
			}
		}
	}
	fmt.Println("")
}

func img8() {
	var element_count = 25
	for x := 0; x < element_count; x++ {
		fmt.Println("")
		for y := 0; y < element_count; y++ {
			if x*y == 0 {
				fmt.Print("# ")
			} else {
				fmt.Print(". ")
			}
		}
	}
	fmt.Println("")
}

func img9() {
	var element_count = 25
	for x := 0; x < element_count; x++ {
		fmt.Println("")
		for y := 0; y < element_count; y++ {
			if x < y-10 || y < x-10 {
				fmt.Print("# ")
			} else {
				fmt.Print(". ")
			}
		}
	}
	fmt.Println("")
}

func img4() {
	var element_count = 25
	for x := 0; x < element_count; x++ {
		fmt.Println("")
		for y := 0; y < element_count; y++ {
			if x+y < 30 {
				fmt.Print("# ")
			} else {
				fmt.Print(". ")
			}
		}
	}
	fmt.Println("")
}

func img24() {
	var element_count = 25
	for x := 0; x < element_count; x++ {
		fmt.Println("")
		for y := 0; y < element_count; y++ {
			if x == y || x == 24-y {
				fmt.Print("# ")
			} else {
				fmt.Print(". ")
			}
		}
	}
	fmt.Println("")
}

func main() {
	img5()
}
