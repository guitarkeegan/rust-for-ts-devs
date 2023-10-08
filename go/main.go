package main

import "fmt"

func main() {

	var col = Color{"red", "blue", "green"}
	fmt.Println(col.isGreen())
	newCol := newColor()
	fmt.Println(newCol.isGreen())
}

type Color struct {
	Red   string
	Blue  string
	Green string
}

func newColor() Color {
	return Color{"hi", "yo", "hey"}
}

func (c Color) isGreen() bool {
	if c.Green == "green" {
		return true
	}
	return false
}
