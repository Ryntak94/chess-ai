package main

import "fmt"
import "github.com/notnil/chess"
import "math/rand"
// import "time"

func main() {
    game := chess.NewGame()
	// generate moves until game is over
	for game.Outcome() == chess.NoOutcome {
		// select a random move
		moves := game.ValidMoves()
		move := moves[rand.Intn(len(moves))]
        game.Move(move)
        // fmt.Println(game.Position().Board().Draw())
        // time.Sleep(1 * time.Second)
	}
	// print outcome and game PGN
	fmt.Println(game.Position().Board().Draw())
	fmt.Printf("Game completed. %s by %s.\n", game.Outcome(), game.Method())
	fmt.Println(game.String())    
}
