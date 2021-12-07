package main

import (
	"log"
)

const NEW_LANTERNFISH_TIMER = 8
const LANTERNFISH_TIMER = 6

func breedLanternfish(fish []int, days int) int {
	for day := 0; day < days; day++ {
		for i := range fish {
			if fish[i] == 0 {
				fish[i] = LANTERNFISH_TIMER
				fish = append(fish, NEW_LANTERNFISH_TIMER)
			} else {
				fish[i]--
			}
		}
	}

	return len(fish)
}

func betterBreedLanternfish(initial_fish []int, days int) int64 {
	day_fish := make([]int64, NEW_LANTERNFISH_TIMER+1)

	for _, fish := range initial_fish {
		day_fish[fish] += 1
	}

	for day := 0; day < days; day++ {
		new_breeds := day_fish[0]

		for i := 1; i <= NEW_LANTERNFISH_TIMER; i++ {
			day_fish[i-1] = day_fish[i]
		}

		day_fish[LANTERNFISH_TIMER] += new_breeds
		day_fish[NEW_LANTERNFISH_TIMER] = new_breeds
	}

	return Sum64(day_fish)
}

func main6() {
	data, err := ReadInputFrom("6.inp")

	if err != nil {
		log.Fatal(err)
		return
	}

	fish := ReadIntegersOnLine(data[0])
	fish2 := ReadIntegersOnLine(data[0])

	log.Println(breedLanternfish(fish, 80))
	log.Println(betterBreedLanternfish(fish2, 256))
}
