//go:build !docker

package internal

import "fmt"

func DockerPs() {
	fmt.Println("Tag was not set")
}
