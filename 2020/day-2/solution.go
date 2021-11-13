package main

import (
	"fmt"
	"log"
	"os"
	"regexp"
	"strconv"
	"strings"
)

var RANGE_REGEX = regexp.MustCompile("[0-9]{1,2}-[0-9]{1,2}")

func IsValid(line string) bool {
	max, _ := strconv.Atoi(RANGE_REGEX.FindString(line))
	min, _ := strconv.Atoi(line)
	test := rune(line[0])
	password := line 
	count := 0
	for _, character := range password {
		if character ==  test {
			count += 1
		}
	}
	if count >= min && count <= max {
		return true
	}
	return false
}

func main() {
	input, err := os.ReadFile("input.txt")
	if err != nil {
		log.Fatal(err)
	}
	passwords := strings.Split(string(input), "\r\n")
	for _, password := range passwords {
		if valid := IsValid(password); valid == true {
			fmt.Println(password)
		}
	}
}
