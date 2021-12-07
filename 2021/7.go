package main

import (
	"log"
	"math"
)

func MinNumber(numbers []int) int {
	min := 2147483647 // max int

	for _, n := range numbers {
		if n < min {
			min = n
		}
	}

	return min
}

func getCheapestPosition(positions []int) int {
	total_costs := make([]int, len(positions))

	for i := range total_costs {
		for _, p := range positions {
			total_costs[i] += int(math.Abs(float64(p - i)))
		}
	}

	return MinNumber(total_costs)
}

func getFuelCost(distance int) int {
	return distance * (distance + 1) / 2
}

func getNewCheapestPosition(positions []int) int {
	total_costs := make([]int, len(positions))

	for i := range total_costs {
		for _, p := range positions {
			total_costs[i] += getFuelCost(int(math.Abs(float64(p - i))))
		}
	}

	return MinNumber(total_costs)
}

func main7() {
	data, err := ReadInputFrom("7.inp")

	if err != nil {
		log.Fatal(err)
		return
	}

	positions := ReadIntegersOnLine(data[0])

	log.Println(getCheapestPosition(positions))
	log.Println(getNewCheapestPosition(positions))
}
