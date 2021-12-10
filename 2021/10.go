package main

import (
	"log"
	"sort"
)

type Token = rune
type Program = []Token

func parseInput(data []string) []Program {
	programs := []Program{}

	for _, line := range data {
		programs = append(programs, Program(line))
	}

	return programs
}

func getSyntaxErrorScore(programs []Program) int {
	score := 0
	closingChars := map[rune]rune{
		'(': ')',
		'[': ']',
		'{': '}',
		'<': '>',
	}
	scores := map[rune]int{
		')': 3,
		']': 57,
		'}': 1197,
		'>': 25137,
	}

	for _, program := range programs {
		open := []rune{}

		for _, ch := range program {
			if val, ok := closingChars[ch]; ok {
				open = append(open, val)
			} else {
				if open[len(open)-1] != ch {
					score += scores[ch]
					break
				}

				open = open[:len(open)-1]
			}
		}
	}

	return score
}

func getAutocompleteScore(programs []Program) int {
	scores := []int{}
	closingChars := map[rune]rune{
		'(': ')',
		'[': ']',
		'{': '}',
		'<': '>',
	}
	scoresFor := map[rune]int{
		')': 1,
		']': 2,
		'}': 3,
		'>': 4,
	}

	for _, program := range programs {
		open := []rune{}
		corrupted := false
		score := 0

		for _, ch := range program {
			if val, ok := closingChars[ch]; ok {
				open = append(open, val)
			} else {
				if open[len(open)-1] != ch {
					corrupted = true
					break
				}

				open = open[:len(open)-1]
			}
		}

		if !corrupted {
			for i := (len(open) - 1); i >= 0; i-- {
				score = (score * 5) + scoresFor[open[i]]
			}

			scores = append(scores, score)
		}
	}

	sort.Ints(scores)

	return scores[len(scores)/2]
}

func main10() {
	data, err := ReadInputFrom("10.inp")

	if err != nil {
		log.Fatal(err)
		return
	}

	programs := parseInput(data)

	log.Println(getSyntaxErrorScore(programs))
	log.Println(getAutocompleteScore(programs))
}
