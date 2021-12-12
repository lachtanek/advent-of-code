package main

import (
	"log"
	"strings"
)

type CaveNode struct {
	id          string
	connections []string
}

type CaveGraph struct {
	caves map[string]*CaveNode
}

func (graph CaveGraph) addConnection(from, to string) {
	from_node, ok := graph.caves[from]
	if !ok {
		graph.caves[from] = &CaveNode{from, []string{}}
		from_node = graph.caves[from]
	}

	to_node, ok := graph.caves[to]
	if !ok {
		graph.caves[to] = &CaveNode{to, []string{}}
		to_node = graph.caves[to]
	}

	from_node.connections = append(from_node.connections, to)
	to_node.connections = append(to_node.connections, from)
}

func parseCaveMap(data []string) CaveGraph {
	graph := CaveGraph{map[string]*CaveNode{}}

	for _, line := range data {
		caves := strings.Split(line, "-")
		graph.addConnection(caves[0], caves[1])
	}

	return graph
}

func isSmall(id string) bool {
	if id == "end" || id == "start" {
		return false
	}

	return id == strings.ToLower(id)
}

func pathContains(path []string, id string) bool {
	for _, n := range path {
		if n == id {
			return true
		}
	}

	return false
}

func dfsPath(graph CaveGraph, node string, path []string) [][]string {
	if node == "end" {
		return [][]string{append(path, "end")}
	}

	paths := [][]string{}

	for _, adj := range graph.caves[node].connections {
		if isSmall(adj) && pathContains(path, adj) {
			continue
		}

		if adj == "start" {
			continue
		}

		newPaths := dfsPath(graph, adj, copyAndAppend(path, node))
		paths = append(paths, newPaths...)
	}

	return paths
}

func canVisitCave(path []string, id string) bool {
	if id == "start" {
		return false
	}

	if !isSmall(id) {
		return true
	}

	visited := false
	smallTwice := false
	smallEntered := map[string]bool{}
	for _, n := range path {
		if isSmall(n) && !smallTwice {
			if _, ok := smallEntered[n]; ok {
				smallTwice = true
			} else {
				smallEntered[n] = true
			}
		}

		if n == id {
			visited = true
		}
	}

	if visited && smallTwice {
		return false
	}

	return true
}

func dfsPath2(graph CaveGraph, node string, path []string) [][]string {
	if node == "end" {
		return [][]string{path}
	}

	paths := [][]string{}

	for _, adj := range graph.caves[node].connections {
		if canVisitCave(path, adj) {
			newPaths := dfsPath2(graph, adj, copyAndAppend(path, adj))
			paths = append(paths, newPaths...)
		}
	}

	return paths
}

func main() {
	data, err := ReadInputFrom("12.inp")

	if err != nil {
		log.Fatal(err)
		return
	}

	graph := parseCaveMap(data)

	log.Println(len(dfsPath(graph, "start", []string{})))
	log.Println(len(dfsPath2(graph, "start", []string{"start"})))
}
