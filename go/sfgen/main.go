package main

import (
	"flag"
	"fmt"
	"log"
	"strconv"
	"time"

	"github.com/kkrypt0nn/spaceflake"
)

// Just some code to generate a Spaceflake more easily in the terminal, kind of like `uuidgen`

func main() {
	shouldDecompose := flag.Bool("decompose", false, "whether to decompose a Spacflake or just generate one")
	node := flag.Int("node", 0, "the node id to use")
	worker := flag.Int("worker", 0, "the worker id to use")
	flag.Parse()
	toDecompose := flag.Arg(0)

	if *shouldDecompose {
		id, err := strconv.ParseUint(toDecompose, 10, 64)
		if err != nil {
			log.Fatalf("Failed parsing the Spaceflake: %s", err)
			return
		}
		decomposed := spaceflake.Decompose(id, spaceflake.EPOCH)
		fmt.Println("Node:", decomposed["nodeID"])
		fmt.Println("Worker:", decomposed["workerID"])
		fmt.Println("Sequence:", decomposed["sequence"])
		timestamp := decomposed["time"]
		date := time.UnixMilli(int64(timestamp)).Format("Mon, 02 Jan 2006 15:04:05 MST")
		fmt.Println("Time:", timestamp, "("+date+")")
		return
	}

	genSettings := spaceflake.NewGeneratorSettings()
	genSettings.NodeID = uint64(*node)
	genSettings.WorkerID = uint64(*worker)
	sf, err := spaceflake.Generate(genSettings)
	if err != nil {
		log.Fatalf("Failed generating a Spaceflake: %s", err)
		return
	}
	fmt.Println(sf.StringID())
}
