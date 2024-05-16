package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"testing"
)

func TestLuckyTickets6(t *testing.T) {
	expected := 55252

	result := LuckyTickets6()

	if expected != result {
		t.Errorf("Ожидалось число 55252, получено %d", result)
	}

}

func TestLuckyTicketsCount(t *testing.T) {
	var in int
	var out int
	lt := LuckyTickets{}

	for i := 0; i < 5; i++ {
		t.Logf("test %d\n", i)
		fIn, err := os.Open(fmt.Sprintf("./1.Tickets/test.%d.in", i))
		if err != nil {
			t.Errorf("cannot open test file test.%d.in", i)
		}
		defer fIn.Close()

		fOut, err := os.Open(fmt.Sprintf("./1.Tickets/test.%d.out", i))
		if err != nil {
			t.Errorf("cannot open test file test.%d.out", i)
		}
		defer fOut.Close()

		sc := bufio.NewScanner(fIn)
		sc.Scan()

		in, err = strconv.Atoi(sc.Text())
		if err != nil {
			t.Error(err)
		}

		sco := bufio.NewScanner(fOut)
		sco.Scan()

		out, err = strconv.Atoi(sco.Text())

		if err != nil {
			t.Error(err)
		}

		luckyCount := lt.LuckyTicketsCount(in)
		if out != int(luckyCount) {
			t.Errorf("Ошибка: ожидалость число %d, получено %d", out, luckyCount)
		} else {
			t.Logf("Из всех %d-значных билетов, %d счастливых\n", in*2, luckyCount)
		}
	}

}
