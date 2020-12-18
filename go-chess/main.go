package main

// https://towardsdatascience.com/neural-network-from-scratch-in-go-language-b98e2abcced3

import (
	// "math/rand"
	// "time"
	"math"
	"fmt"
	"github.com/notnil/chess"
	"encoding/binary"
	"strings"
	"io/ioutil"
	"regexp"
)

func main() {
	data, err := ioutil.ReadFile("../twicPGN/twic920.pgn")
    if err != nil {
        fmt.Println("File reading error", err)
        return
	}
	
	gamePattern := regexp.MustCompile(`(?Us)1\..*(?-U)\d?/?\d-\d?/?\d`) 
	games := gamePattern.FindAllString(string(data), -1)

	var possibleStates [][]float64 
	var actualStates []float64

	for game := 0; game < 10; game++ {
		pgn, err := chess.PGN(strings.NewReader(games[game]))
		if err != nil {
			fmt.Println(err)
		}

		game := chess.NewGame(pgn)	
		actualMoves := game.Moves()

		game = chess.NewGame()
		// fmt.Println(len(actualMoves))
		for actualMove := 0; actualMove < len(actualMoves); actualMove++ {
			moves := game.ValidMoves()

			freeze_pos := game

			var possibleStateList []float64
			for move := 0; move < len(moves); move++ {
				game.Move(moves[move])
				pos, err := game.Position().Board().MarshalBinary()
				if err != nil {
					fmt.Println(err)
				}
				possibleStateList = append(possibleStateList, Float64bits(pos)))
				// reset position
				game = freeze_pos
			}
			game.Move(actualMoves[actualMove])
			pos, err := game.Position().Board().MarshalBinary()
			if err != nil {
				fmt.Println(err)
			}
			actualStates = append(actualStates, Float64bits(pos)))
			possibleStates = append(possibleStates, possibleStateList)
		}	
	}

	fmt.Println(possibleStates)
	fmt.Println(actualStates)
}

func Float64bits(f uint64) float64 {
	return *(*float64)(unsafe.Pointer(&f))
}
