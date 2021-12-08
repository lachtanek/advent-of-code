package main

import (
	"log"
	"strings"
)

type DisplayData struct {
	digits []string
	result []string
}

func parseDigits(line string) []string {
	return strings.Split(line, " ")
}

func parseDisplayData(line string) DisplayData {
	inAndOut := strings.Split(line, " | ")
	return DisplayData{parseDigits(inAndOut[0]), parseDigits(inAndOut[1])}
}

func parseAllDisplayData(data []string) []DisplayData {
	displays := make([]DisplayData, len(data))

	for i, line := range data {
		displays[i] = parseDisplayData(line)
	}

	return displays
}

func getNumUniqueDigits(displays []DisplayData) int {
	total := 0

	for _, display := range displays {
		for _, digit := range display.result {
			if len(digit) == 2 || len(digit) == 3 || len(digit) == 4 || len(digit) == 7 {
				total += 1
			}
		}
	}

	return total
}

func toBits(digit string) int {
	bits := 0

	for _, c := range digit {
		bits |= 1 << (c - 'a')
	}

	return bits
}

/**
How the numbers look:
  0:      1:      2:      3:      4:
 aaaa    ....    aaaa    aaaa    ....
b    c  .    c  .    c  .    c  b    c
b    c  .    c  .    c  .    c  b    c
 ....    ....    dddd    dddd    dddd
e    f  .    f  e    .  .    f  .    f
e    f  .    f  e    .  .    f  .    f
 gggg    ....    gggg    gggg    ....

  5:      6:      7:      8:      9:
 aaaa    aaaa    aaaa    aaaa    aaaa
b    .  b    .  .    c  b    c  b    c
b    .  b    .  .    c  b    c  b    c
 dddd    dddd    ....    dddd    dddd
.    f  e    f  .    f  e    f  .    f
.    f  e    f  .    f  e    f  .    f
 gggg    gggg    ....    gggg    gggg

Variables named sX and sXX represent bitmask value of segment X
for the current display.

`ds` array represents bitmask values of numbers for the
current display (index is the number, value is the bitmask).
*/
func decodeDisplayNumber(display DisplayData) int {
	ds := make([]int, 10)
	bds := []int{}

	for _, digit := range display.digits {
		bitDigit := toBits(digit)

		if len(digit) == 2 {
			ds[1] = bitDigit
		} else if len(digit) == 3 {
			ds[7] = bitDigit
		} else if len(digit) == 4 {
			ds[4] = bitDigit
		} else if len(digit) == 7 {
			ds[8] = bitDigit
		} else {
			bds = append(bds, bitDigit)
		}
	}

	sA := ds[7] ^ ds[1]
	sCF := ds[1]
	sEG := ds[8] ^ (ds[4] | sA)
	sBD := ds[4] ^ ds[1]

	for _, bd := range bds {
		if bd&(127^sEG) == (127^sEG) && bd^sEG != sEG {
			ds[9] = bd
		} else if bd&(127^sBD) == (127^sBD) && bd^sBD != sBD {
			ds[0] = bd
		} else if bd&(127^sCF) == (127^sCF) && bd^sCF != sCF {
			ds[6] = bd
		}
	}

	sE := 127 ^ ds[9]
	sG := sEG ^ sE
	sD := 127 ^ ds[0]
	sB := sBD ^ sD
	sC := 127 ^ ds[6]
	sF := sCF ^ sC
	ds[3] = ds[7] | sD | sG
	ds[2] = 127 - sB - sF
	ds[5] = ds[6] - sE

	sum := 0
	for _, digit := range display.result {
		bitDigit := toBits(digit)
		for n, b := range ds {
			if b == bitDigit {
				sum = sum*10 + n
			}
		}
	}

	return sum
}

func getDisplaysSum(displays []DisplayData) int {
	sum := 0
	for _, d := range displays {
		sum += decodeDisplayNumber(d)
	}
	return sum
}

func main8() {
	data, err := ReadInputFrom("8.inp")

	if err != nil {
		log.Fatal(err)
		return
	}

	displays := parseAllDisplayData(data)

	log.Println(getNumUniqueDigits(displays))
	log.Println(getDisplaysSum(displays))
}
