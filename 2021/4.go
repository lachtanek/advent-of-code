package main

import (
	"log"
	"strconv"
	"strings"
)

type Board = []int64

func getDrawnNumbers(line string) []int64 {
	values := []int64{}
	var value int64

	for _, number_string := range strings.Split(line, ",") {
		value, _ = strconv.ParseInt(number_string, 10, 64)
		values = append(values, value)
	}

	return values
}

func getBoardLine(line string) []int64 {
	data := strings.Split(strings.ReplaceAll(strings.TrimSpace(line), "  ", " "), " ")
	numbers := []int64{}

	for _, e := range data {
		n, _ := strconv.ParseInt(e, 10, 64)
		numbers = append(numbers, n)
	}

	return numbers
}

func getBoards(data []string) []Board {
	board := Board{}
	boards := []Board{}

	for _, line := range data {
		if line == "" {
			boards = append(boards, board)
			board = Board{}
			continue
		}

		board = append(board, getBoardLine(line)...)
	}

	return boards
}

func getBoardMask(board Board, drawn int64) int64 {
	var mask int64 = 0

	for i, n := range board {
		if n == drawn {
			mask |= 1 << i
		}
	}

	return mask
}

func isWinning(mask int64) bool {
	winMasks := []int64{32505856, 1015808, 31744, 992, 31, 17318416, 8659208, 4329604, 2164802, 1082401}

	for _, m := range winMasks {
		if mask&m == m {
			return true
		}
	}

	return false
}

func getUnmarkedSum(board Board, mask int64) int64 {
	var sum int64 = 0

	for i := range board {
		if mask&(1<<i) == 0 {
			sum += board[i]
		}
	}

	return sum
}

func getWinScore(boards []Board, draws []int64) int64 {
	masks := make([]int64, len(boards))

	for _, draw := range draws {
		for i, board := range boards {
			masks[i] |= getBoardMask(board, draw)
			if isWinning(masks[i]) {
				return getUnmarkedSum(board, masks[i]) * draw
			}
		}
	}

	return -1
}

type Win struct {
	mask int64
	draw int64
}

func getLastWinScore(boards []Board, draws []int64) int64 {
	lastWins := make(map[int]Win)
	masks := make([]int64, len(boards))

	for _, draw := range draws {
		for i, board := range boards {
			masks[i] |= getBoardMask(board, draw)
			if isWinning(masks[i]) {
				lastWins[i] = Win{masks[i], draw}

				if len(lastWins) == len(boards) {
					win := lastWins[i]
					return getUnmarkedSum(board, win.mask) * win.draw
				}
			}
		}
	}

	return -1
}

func main() {
	data, err := ReadInputFrom("4.inp")

	if err != nil {
		log.Fatal(err)
		return
	}

	draws := getDrawnNumbers(data[0])
	boards := getBoards(data[2:])

	log.Println(getWinScore(boards, draws))
	log.Println(getLastWinScore(boards, draws))
}
