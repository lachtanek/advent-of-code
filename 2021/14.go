package main

import (
	"fmt"
	"log"
	"math"
	"strings"
)

type Polymer struct {
	groups map[string]int
}

func parsePolymerTransforms(data []string) map[string]string {
	transforms := make(map[string]string)

	for _, line := range data {
		splits := strings.Split(line, " -> ")
		transforms[splits[0]] = splits[1]
	}

	return transforms
}

func transformPolymerGroup(group string, transforms map[string]string) []string {
	transform := transforms[group]
	return []string{group[0:1] + transform, transform + group[1:2]}
}

func parsePolymer(polymer string) Polymer {
	newPolymer := Polymer{}
	newPolymer.groups = make(map[string]int)

	for i := 0; i < len(polymer)-1; i++ {
		g := polymer[i : i+2]
		newPolymer.modify(g, 1)
	}

	return newPolymer
}

func (polymer Polymer) modify(group string, value int) {
	if _, ok := polymer.groups[group]; ok {
		polymer.groups[group] += value
	} else {
		polymer.groups[group] = value
	}
}

func (polymer Polymer) getScore() int64 {
	nums := make(map[rune]int64)

	for g, v := range polymer.groups {
		for _, c := range g {
			if _, ok := nums[c]; !ok {
				nums[c] = int64(v)
			} else {
				nums[c] += int64(v)
			}
		}
	}

	var min int64 = 9223372036854775807
	var max int64 = 0

	for _, v := range nums {
		if v > max {
			max = v
		}
		if v < min {
			min = v
		}
	}

	return int64(math.Round(float64(max)/2) - math.Round(float64(min)/2))
}

func getNextPolymer(polymer Polymer, transforms map[string]string) Polymer {
	newPolymer := Polymer{}
	newPolymer.groups = make(map[string]int)

	for g, n := range polymer.groups {
		newGroups := transformPolymerGroup(g, transforms)
		newPolymer.modify(newGroups[0], n)
		newPolymer.modify(newGroups[1], n)
	}

	return newPolymer
}

func main14() {
	data, err := ReadInputFrom("14.inp")

	if err != nil {
		log.Fatal(err)
		return
	}

	polymer := parsePolymer(data[0])
	transforms := parsePolymerTransforms(data[2:])

	for i := 0; i < 40; i++ {
		polymer = getNextPolymer(polymer, transforms)

		if i == 9 {
			fmt.Println("After 10 steps: ", polymer.getScore())
		}
	}

	fmt.Println("After 40 steps: ", polymer.getScore())
}
