package main

import "fmt"

func main() {
	container := [2]int{0, 1}
	sum := 0

	for container[1] < 4000000 {
		temp := container[0] + container[1]
		container[0] = container[1]
		container[1] = temp

		if container[1]%2 == 0 {
			sum += container[1]
		}
	}

	fmt.Printf("Sum below 4 million is %v\n", sum)
}
