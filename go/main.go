package main

import (
	"bufio"
	"fmt"
	"os"
)

func main() {
	file, err := os.Open("lines")
	if err != nil {
		fmt.Println("Error opening file")
		return
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)

	for scanner.Scan() {
		line := scanner.Text()
		fmt.Println(line)
	}

	if err := scanner.Err(); err != nil {
		fmt.Println("Error reading file: ", err)
		return
	}

}
