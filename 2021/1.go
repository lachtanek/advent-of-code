package main

import (
	"log"
)

func countIncreases(depths []int) int {
	increases := 0
	prev_depth := 10000000
	for _, depth := range depths {
		if depth > prev_depth {
			increases++
		}

		prev_depth = depth
	}

	return increases
}

func sum(nums []int) int {
	s := 0

	for _, n := range nums {
		s += n
	}

	return s
}

func getSums(depths []int) []int {
	sums := make([]int, len(depths)/3)

	for i := range depths {
		if i+3 < len(depths) {
			sums = append(sums, sum(depths[i:i+3]))
		}
	}

	return sums
}

func main1() {
	numbers, err := ReadNumbersInputFrom("1.inp")

	if err != nil {
		log.Fatal(err)
		return
	}

	increases := countIncreases(numbers)

	log.Println(increases)

	windowedIncreases := countIncreases(getSums(numbers))

	log.Println(windowedIncreases)
}
