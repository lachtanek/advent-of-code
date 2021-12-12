package main

import (
	"bufio"
	"os"
	"strconv"
	"strings"
)

func ReadNumbersInputFrom(filename string) ([]int, error) {
	file, err := os.Open(filename)
	if err != nil {
		return nil, err
	}

	defer file.Close()

	numbers := []int{}
	scanner := bufio.NewScanner(file)

	for scanner.Scan() {
		number, err := strconv.Atoi(scanner.Text())
		if err != nil {
			return nil, err
		}

		numbers = append(numbers, number)
	}

	err = scanner.Err()
	if err != nil {
		return nil, err
	}

	return numbers, nil
}

func ReadInputFrom(filename string) ([]string, error) {
	file, err := os.Open(filename)
	if err != nil {
		return nil, err
	}

	defer file.Close()

	data := []string{}
	scanner := bufio.NewScanner(file)

	for scanner.Scan() {
		line := scanner.Text()
		data = append(data, line)
	}

	err = scanner.Err()
	if err != nil {
		return nil, err
	}

	return data, nil
}

func Sum64(nums []int64) int64 {
	var s int64 = 0

	for _, n := range nums {
		s += n
	}

	return s
}

func ReadIntegersOnLine(line string) []int {
	values := []int{}
	var value int64

	for _, number_string := range strings.Split(line, ",") {
		value, _ = strconv.ParseInt(number_string, 10, 64)
		values = append(values, int(value))
	}

	return values
}

func copyAndAppend(data []string, e string) []string {
	newData := make([]string, len(data))
	copy(newData, data)
	newData = append(newData, e)
	return newData
}
