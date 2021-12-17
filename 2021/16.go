package main

import (
	"fmt"
	"log"
	"math"
	"strconv"
)

type Packet struct {
	version    int64
	t          int64
	content    string
	subPackets []Packet
}

func parseBits(data string) string {
	bits := ""

	for _, c := range data {
		n, _ := strconv.ParseInt(string(c), 16, 64)
		for i := 3; i >= 0; i-- {
			if n&int64(1<<i) > 0 {
				bits += "1"
			} else {
				bits += "0"
			}
		}
	}

	return bits
}

func parsePacket(data string, ptr *int) Packet {
	version, _ := strconv.ParseInt(data[*ptr:*ptr+3], 2, 32)
	t, _ := strconv.ParseInt(data[*ptr+3:*ptr+6], 2, 32)
	*ptr += 6

	if t == 4 {
		n := ""
		for *ptr < len(data) {
			n += data[*ptr+1 : *ptr+5]
			p := *ptr
			*ptr += 5

			if data[p] == '0' {
				break
			}
		}

		return Packet{version, t, n, []Packet{}}
	} else {
		var lengthSize int
		var typeId bool
		// type id
		if data[*ptr] == '0' {
			// typeid = 0 -> length field represents length in bits of sub-packets
			lengthSize = 15
			typeId = false
		} else {
			// typeid = 1 -> length field represents number of sub-packets
			lengthSize = 11
			typeId = true
		}

		*ptr += 1
		offset := *ptr + lengthSize
		length, _ := strconv.ParseInt(data[*ptr:offset], 2, 64)

		if !typeId {
			*ptr += lengthSize
			endOffset := int(length) + *ptr
			subPackets := []Packet{}
			for *ptr < endOffset {
				packet := parsePacket(data, ptr)
				subPackets = append(subPackets, packet)
			}

			return Packet{version, t, "", subPackets}
		} else {
			*ptr += lengthSize
			subPackets := []Packet{}
			for n := 0; n < int(length); n++ {
				packet := parsePacket(data, ptr)
				subPackets = append(subPackets, packet)
			}

			return Packet{version, t, "", subPackets}
		}
	}
}

func addVersions(packet Packet) int {
	total := int(packet.version)

	for _, sub := range packet.subPackets {
		total += addVersions(sub)
	}

	return total
}

func addValues(packet Packet) int {
	switch packet.t {
	case 0:
		sum := 0
		for _, p := range packet.subPackets {
			sum += addValues(p)
		}
		return sum
	case 1:
		total := 1
		for _, p := range packet.subPackets {
			total *= addValues(p)
		}
		return total
	case 2:
		min := math.MaxInt32
		for _, p := range packet.subPackets {
			v := addValues(p)
			if v < min {
				min = v
			}
		}
		return min
	case 3:
		max := 0
		for _, p := range packet.subPackets {
			v := addValues(p)
			if v > max {
				max = v
			}
		}
		return max
	case 4:
		v, _ := strconv.ParseInt(packet.content, 2, 64)
		return int(v)
	case 5:
		v1 := addValues(packet.subPackets[0])
		v2 := addValues(packet.subPackets[1])
		if v1 > v2 {
			return 1
		} else {
			return 0
		}
	case 6:
		v1 := addValues(packet.subPackets[0])
		v2 := addValues(packet.subPackets[1])
		if v1 < v2 {
			return 1
		} else {
			return 0
		}
	case 7:
		v1 := addValues(packet.subPackets[0])
		v2 := addValues(packet.subPackets[1])
		if v1 == v2 {
			return 1
		} else {
			return 0
		}
	}

	log.Fatal("unk switch", packet.t)
	return 0
}

func main16() {
	data, err := ReadInputFrom("16.inp")

	if err != nil {
		log.Fatal(err)
		return
	}

	bits := parseBits(data[0])
	ptr := 0
	parsed := parsePacket(bits, &ptr)

	fmt.Println("versions: ", addVersions(parsed))
	fmt.Println("value: ", addValues(parsed))
}
