package main

import (
	"log"
	"math"
	"strconv"
)

type Result struct {
	gamma   int64
	epsilon int64
}

func intPow(x int64, y int) int64 {
	return int64(math.Pow(float64(x), float64(y)))
}

func parseValues(data []string) []int64 {
	values := []int64{}
	var value int64

	for _, line := range data {
		value, _ = strconv.ParseInt(line, 2, 64)
		values = append(values, value)
	}

	return values
}

func getGammaEpsilon(data []int64) Result {
	oneCnt := 0
	result := Result{}

	for i := 11; i >= 0; i-- {
		for _, n := range data {
			if n&intPow(2, i) > 0 {
				oneCnt++
			}
		}

		if oneCnt > len(data)/2 {
			result.gamma += intPow(2, i)
		} else {
			result.epsilon += intPow(2, i)
		}

		oneCnt = 0
	}

	return result
}

func findOxygenCO2Ratings(data []int64, bit int64, rating bool) int64 {
	if len(data) == 1 {
		return data[0]
	}

	// log.Println(data, bit)

	var values []int64
	oneCnt, zeroCnt := 0, 0

	for _, n := range data {
		if n&bit > 0 {
			oneCnt++
		} else {
			zeroCnt++
		}
	}

	moreOnes := oneCnt >= zeroCnt

	for _, n := range data {
		if n&bit > 0 {
			if moreOnes == rating {
				values = append(values, n)
			}
		} else {
			if moreOnes != rating {
				values = append(values, n)
			}
		}
	}

	return findOxygenCO2Ratings(values, bit/2, rating)
}

func main() {
	data, err := ReadInputFrom("3.inp")

	if err != nil {
		log.Fatal(err)
		return
	}

	values := parseValues(data)

	log.Println(getGammaEpsilon(values))

	log.Println(findOxygenCO2Ratings(values, 2048, true) * findOxygenCO2Ratings(values, 2048, false))
}
