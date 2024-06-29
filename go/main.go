package main

import "fmt"

func main() {
	fmt.Printf("Hello, World!\n")

	a := uint8(255)
	b := uint8(1)
	c := false
	value := uint16(a) + uint16(b)
	if c {
		value++
	}
	fmt.Printf("a = %08b = %v\n", a, a)
	fmt.Printf("b = %08b = %v\n", b, b)
	fmt.Printf("c = %v\n", c)
	fmt.Printf("value = %016b = %v\n", value, value)
	byteValue := uint8(value)
	fmt.Printf("value = %08b = %v\n", byteValue, byteValue)
}
