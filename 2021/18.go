package main

import (
	"fmt"
	"log"
)

type SnailfishNumber struct {
	left  *SnailfishNumber
	right *SnailfishNumber
	value int
}

type SnailfishParse struct {
	data string
	pos  int
}

func (parse *SnailfishParse) next() byte {
	parse.pos += 1
	return parse.data[parse.pos-1]
}

func (parse *SnailfishParse) expect(c byte) {
	if found := parse.next(); found != c {
		err := fmt.Sprintf("Expected '%c', found '%c' at pos %v in %v", c, found, parse.pos, parse.data)
		log.Fatalln(err)
	}
}

func (parse *SnailfishParse) number() SnailfishNumber {
	ch := parse.next()
	if ch == '[' {
		left := parse.number()
		parse.expect(',')
		right := parse.number()
		parse.expect(']')

		return SnailfishNumber{&left, &right, 0}
	} else if ch >= '0' && ch <= '9' {
		return SnailfishNumber{nil, nil, int(ch - '0')}
	}

	log.Fatalln("Cant parse")
	return SnailfishNumber{}
}

func parseSnailfish(data string) *SnailfishNumber {
	parse := SnailfishParse{data, 0}
	n := parse.number()
	if parse.pos != len(data) {
		log.Fatalln("not reached end")
	}

	return &n
}

func (n SnailfishNumber) isRegular() bool {
	return n.left == nil && n.right == nil
}

func (n SnailfishNumber) str() string {
	if n.isRegular() {
		return fmt.Sprint(n.value)
	} else {
		return "[" + n.left.str() + "," + n.right.str() + "]"
	}
}

func (n SnailfishNumber) regulars() []*SnailfishNumber {
	lst := []*SnailfishNumber{}

	if n.left.isRegular() {
		lst = append(lst, n.left)
	} else {
		lst = append(lst, n.left.regulars()...)
	}

	if n.right.isRegular() {
		lst = append(lst, n.right)
	} else {
		lst = append(lst, n.right.regulars()...)
	}

	return lst
}

func (number *SnailfishNumber) explode(parents []*SnailfishNumber) bool {
	if number.isRegular() {
		return false
	}

	if len(parents) == 4 {
		if !number.left.isRegular() || !number.right.isRegular() {
			log.Fatal("explode left/right not regular")
			return true
		}

		regulars := parents[0].regulars()

		i := 0
		for regulars[i] != number.left {
			i++
		}

		if i > 0 {
			regulars[i-1].value += number.left.value
		}
		// +1 is number.right
		if i < len(regulars)-2 {
			regulars[i+2].value += number.right.value
		}

		number.left = nil
		number.right = nil
		number.value = 0

		return true
	}

	stack := make([]*SnailfishNumber, len(parents))
	copy(stack, parents)
	stack = append(stack, number)
	return number.left.explode(stack) || number.right.explode(stack)
}

func (n *SnailfishNumber) split() bool {
	for _, n := range n.regulars() {
		if n.value > 9 {
			n.left = &SnailfishNumber{nil, nil, n.value / 2}

			var r int
			if n.value%2 == 0 {
				r = n.value / 2
			} else {
				r = (n.value + 1) / 2
			}
			n.right = &SnailfishNumber{nil, nil, r}

			return true
		}
	}

	return false
}

func (n *SnailfishNumber) reduce() {
	for {
		if n.explode([]*SnailfishNumber{}) {
			continue
		}

		if n.split() {
			continue
		}

		break
	}
}

func (n *SnailfishNumber) magnitude() int {
	if n.isRegular() {
		return n.value
	}

	return n.left.magnitude()*3 + n.right.magnitude()*2
}

func (n SnailfishNumber) copy() SnailfishNumber {
	if n.isRegular() {
		return SnailfishNumber{nil, nil, n.value}
	}

	left := n.left.copy()
	right := n.right.copy()
	return SnailfishNumber{&left, &right, 0}
}

func parseNumbers(data []string) []*SnailfishNumber {
	res := []*SnailfishNumber{}

	for _, line := range data {
		res = append(res, parseSnailfish(line))
	}

	return res
}

func addNumbers(numbers []*SnailfishNumber) *SnailfishNumber {
	result := numbers[0]

	for _, n := range numbers[1:] {
		result = &SnailfishNumber{result, n, 0}
		result.reduce()
	}

	return result
}

func largestMagnitude(numbers []*SnailfishNumber) int {
	largest := 0

	for _, left := range numbers {
		for _, right := range numbers {
			if left == right {
				continue
			}

			leftC := left.copy()
			rightC := right.copy()
			n := addNumbers([]*SnailfishNumber{&leftC, &rightC}).magnitude()
			if n > largest {
				largest = n
			}
		}
	}

	return largest
}

func main18() {
	data, err := ReadInputFrom("18.inp")

	if err != nil {
		log.Fatal(err)
		return
	}

	numbers := parseNumbers(data)

	n := addNumbers(numbers)
	log.Println(n.magnitude())

	numbers = parseNumbers(data)
	log.Println(largestMagnitude(numbers))
}
