package main

import "fmt"

func main() {
    for i := 1000;  i > 0; i-=7{
        fmt.Println(i, "  - 7 =", i - 7)
    }
}