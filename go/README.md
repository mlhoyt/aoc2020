# Advent-of-Code 2020 | Golang

## Setup

### Golang module -- one time

```
go mod init github.com/mlhoyt/aoc2020/go
```

### pkg/utils/loadInputFile.go -- one time

```
mkdir -p pkg/utils
cp ... pkg/utils/loadInputFile.go
```

### cmd/dayXXpartY/main.go -- each part of each day

```
mkdir -p cmd/day01part1
cat > cmd/day01part1/main.go <<EOHI
package main

import (
	"github.com/mlhoyt/aoc2020/go/pkg/utils"
)

func main() {
	input, err := utils.LoadInputFile("day01.txt")
	if err != nil {
		panic(err)
	}
}
EOHI
```
