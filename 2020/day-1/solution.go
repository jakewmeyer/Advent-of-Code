package main

import (
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

func Problem1(numbers []int) int {
	for _, x := range numbers {
		for _, y := range numbers {
			if x+y == 2020 {
				return x * y
			}
		}
	}
	return 0
}

func Problem2(numbers []int) int {
	for _, x := range numbers {
		for _, y := range numbers {
			for _, z := range numbers {
				if x+y+z == 2020 {
					return x * y * z
				}
			}
		}
	}
	return 0
}

func main() {
	input, err := os.ReadFile("input.txt")
	if err != nil {
		log.Fatal(err)
	}
	inputs := strings.Split(string(input), "\r\n")

	numbers := make([]int, len(inputs))
	for i, v := range inputs {
		numbers[i], err = strconv.Atoi(v)
		if err != nil {
			log.Fatal(err)
		}
	}

	value1 := Problem1(numbers)
	fmt.Println(value1)

	value2 := Problem2(numbers)
	fmt.Println(value2)
}
