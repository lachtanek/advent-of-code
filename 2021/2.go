package main

import (
	"log"
	"strconv"
	"strings"
)

const (
	up      = iota
	down    = iota
	forward = iota
)

type Course struct {
	direction byte
	value     int64
}

func getDirection(dir string) byte {
	switch dir {
	case "up":
		return up
	case "down":
		return down
	case "forward":
		return forward
	}

	return 100
}

func parseCourses(data []string) []Course {
	courses := []Course{}
	var value int64

	for _, line := range data {
		res := strings.Split(line, " ")
		value, _ = strconv.ParseInt(res[1], 10, 64)
		course := Course{direction: getDirection(res[0]), value: value}
		courses = append(courses, course)
	}

	return courses
}

func getFinalValue(courses []Course) int64 {
	var depth int64 = 0
	var pos int64 = 0

	for _, course := range courses {
		switch course.direction {
		case forward:
			pos += course.value
		case up:
			depth -= course.value
		case down:
			depth += course.value
		}
	}

	return depth * pos
}

func getFinalValueBetter(courses []Course) int64 {
	var depth int64 = 0
	var pos int64 = 0
	var aim int64 = 0

	for _, course := range courses {
		switch course.direction {
		case forward:
			pos += course.value
			depth += aim * course.value
		case up:
			aim -= course.value
		case down:
			aim += course.value
		}
	}

	return depth * pos
}

func main2() {
	data, err := ReadInputFrom("2.inp")

	if err != nil {
		log.Fatal(err)
		return
	}

	courses := parseCourses(data)

	log.Println(getFinalValue(courses))

	log.Println(getFinalValueBetter(courses))
}
