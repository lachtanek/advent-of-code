package main

import (
	"fmt"
	"log"
	"math"
)

func removeIdx(points []Point32, idx int) []Point32 {
	if idx == len(points)-1 {
		return points[:len(points)-1]
	} else if idx == 0 {
		return points[1:]
	} else {
		return append(points[:idx], points[idx+1:]...)
	}
}

func intMax(a, b int) int {
	if a > b {
		return a
	} else {
		return b
	}
}

func (graph HeightMap) fakeAstar(start Point32, end Point32) int {
	dist := make(map[Point32]int)
	queue := make(map[Point32]bool)

	for y := 0; y < graph.ysize(); y++ {
		for x := 0; x < graph.xsize(); x++ {
			p := Point32{x, y}
			dist[p] = math.MaxInt32
		}
	}

	dist[start] = 0
	queue[start] = true

	for len(queue) > 0 {
		minDist := math.MaxInt32
		var point Point32
		for p := range queue {
			if dist[p] < minDist {
				point = p
				minDist = dist[p]
			}
		}

		delete(queue, point)

		if point == end {
			break
		}

		for _, adj := range graph.getAdjacentPoints(point) {
			newDist := dist[point] + graph.get(adj)
			if newDist < dist[adj] {
				queue[adj] = true
				dist[adj] = newDist
			}
		}
	}

	return dist[end]
}

func getBigGraph(graph HeightMap) HeightMap {
	bigGraph := HeightMap{[][]int{}}

	for y := 0; y < 5*graph.ysize(); y++ {
		bigGraph.points = append(bigGraph.points, make([]int, graph.xsize()*5))
	}

	for y := 0; y < 5*graph.ysize(); y++ {
		for x := 0; x < 5*graph.xsize(); x++ {
			val := graph.get(Point32{x % graph.xsize(), y % graph.ysize()})
			add := x/graph.xsize() + y/graph.ysize()
			val = val + int(add)
			if val > 9 {
				val = val - 9
			}
			bigGraph.points[y][x] = val
		}
	}

	return bigGraph
}

func main15() {
	data, err := ReadInputFrom("15.inp")

	if err != nil {
		log.Fatal(err)
		return
	}

	graph := parseHeightMap(data)
	fmt.Println(graph.fakeAstar(Point32{0, 0}, Point32{graph.xsize() - 1, graph.ysize() - 1}))

	bigGraph := getBigGraph(graph)
	fmt.Println(bigGraph.fakeAstar(Point32{0, 0}, Point32{bigGraph.xsize() - 1, bigGraph.ysize() - 1}))
}
