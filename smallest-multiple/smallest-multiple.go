package main

import (
	"fmt"
	"os"
	"strconv"
)

var n int

func init() {
	var err error
	n, err = strconv.Atoi(os.Args[1])
	if err != nil || n < 1 {
		fmt.Print("Argument must be positive integer")
		os.Exit(1)
	}
}

func gcd(a, b int) int {
	for b != 0 {
		a, b = b, a%b
	}
	return a
}

func lcm(a, b int) int {
	return (a * b) / gcd(a, b)
}

func lfold(s []int, fn func(a, b int) int) int {
	accum := s[0]

	for _, item := range s[1:] {
		accum = fn(accum, item)
	}

	return accum
}

func main() {
	s := make([]int, n, n)
	for i := range s {
		s[i] = i + 1
	}
	fmt.Println(lfold(s, lcm))
}
