package main

import (
	"fmt"
	"log"
	"strconv"
	"strings"
)

type PaperFold struct {
	axis  byte
	coord int
}

type Paper struct {
	dots  []Point32
	folds []PaperFold
}

func parsePaper(data []string) Paper {
	paper := Paper{}
	readingFolds := false

	for _, line := range data {
		if line == "" {
			readingFolds = true
			continue
		}

		if readingFolds {
			splits := strings.Split(line, "=")
			axis := splits[0][len(splits[0])-1]
			coord, _ := strconv.ParseInt(splits[1], 10, 64)
			paper.folds = append(paper.folds, PaperFold{axis, int(coord)})
		} else {
			splits := strings.Split(line, ",")

			x, _ := strconv.ParseInt(splits[0], 10, 64)
			y, _ := strconv.ParseInt(splits[1], 10, 64)
			paper.dots = append(paper.dots, Point32{int(x), int(y)})
		}
	}

	return paper
}

func transformPoint(fold PaperFold, point Point32) Point32 {
	if fold.axis == 'x' {
		if point.x < fold.coord {
			return point
		} else {
			return Point32{fold.coord - (point.x - fold.coord), point.y}
		}
	} else {
		if point.y < fold.coord {
			return point
		} else {
			return Point32{point.x, fold.coord - (point.y - fold.coord)}
		}
	}
}

func findPoint(points []Point32, point Point32) bool {
	for _, p := range points {
		if p == point {
			return true
		}
	}

	return false
}

func foldPaper(dots []Point32, fold PaperFold) []Point32 {
	newDots := []Point32{}

	for _, p := range dots {
		newPoint := transformPoint(fold, p)
		if !findPoint(newDots, newPoint) {
			newDots = append(newDots, newPoint)
		}
	}

	return newDots
}

func visualize(points []Point32, mx, my int) {
	for y := 0; y < my; y++ {
		for x := 0; x < mx; x++ {
			// inefficient, but who cares
			if findPoint(points, Point32{x, y}) {
				fmt.Print("#")
			} else {
				fmt.Print(" ")
			}
		}

		fmt.Println()
	}
}

func main13() {
	data, err := ReadInputFrom("13.inp")

	if err != nil {
		log.Fatal(err)
		return
	}

	paper := parsePaper(data)

	fmt.Println("Number of dots", len(foldPaper(paper.dots, paper.folds[0])))

	newDots := paper.dots
	for _, fold := range paper.folds {
		newDots = foldPaper(newDots, fold)
	}

	fmt.Println("---- Code is ----")
	visualize(newDots, 39, 6)
}
