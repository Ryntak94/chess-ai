package main

// https://towardsdatascience.com/neural-network-from-scratch-in-go-language-b98e2abcced3

import (
	"math/rand"
	// "time"
	// "math"
	"fmt"
	"github.com/notnil/chess"
)

func main() {

	game := chess.NewGame()
	// generate moves until game is over

	var record [][]*chess.Move
	for game.Outcome() == chess.NoOutcome {
		// select a random move
		moves := game.ValidMoves()
		move := moves[rand.Intn(len(moves))]
		game.Move(move)
		record = append(record, moves)
	}
	
	fmt.Println(record)
	fmt.Println(game.Position().Board().Draw())	
}
