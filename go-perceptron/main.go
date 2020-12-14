package main

// https://towardsdatascience.com/neural-network-from-scratch-in-go-language-b98e2abcced3

import (
	"math/rand"
	"time"
	"math"
	"fmt"
	// "github.com/notnil/chess"
)

func main() {

	// game := chess.NewGame()
	// // generate moves until game is over

	// var record []string
	// for game.Outcome() == chess.NoOutcome {
	// 	// select a random move
	// 	moves := game.ValidMoves()
	// 	move := moves[rand.Intn(len(moves))]
	// 	game.Move(move)
	// 	record = append(record, make([]string, moves))
	// }
	
	// fmt.Println(record)

	goPerceptron := Perceptron{
		input:        [][]float64{{0, 0, 1}, {1, 1, 1}, {1, 0, 1}, {0, 1, 0}}, //Input Data
		actualOutput: []float64{0, 1, 1, 0},                                   //Actual Output
		epochs:       1000,                                                     //Number of Epoch
	}
	goPerceptron.initialize()
	goPerceptron.train()
	// print(goPerceptron.forwardPass([]float64{0, 1, 0}), "\n") //Make Predictions
	// print(goPerceptron.forwardPass([]float64{1, 0, 1}), "\n")
}

type Perceptron struct {
	input        [][]float64
	actualOutput []float64
	weights      []float64
	bias         float64
	epochs       int
}

func dotProduct(v1, v2 []float64) float64 { //Dot Product of Two Vectors of same size
	dot := 0.0
	for i := 0; i < len(v1); i++ {
		dot += v1[i] * v2[i]
	}
	return dot
}

func vecAdd(v1, v2 []float64) []float64 { //Addition of Two Vectors of same size
	add := make([]float64, len(v1))
	for i := 0; i < len(v1); i++ {
		add[i] = v1[i] + v2[i]
	}
	return add
}

func scalarMatMul(s float64, mat []float64) []float64 { //Multiplication of a Vector & Matrix
	result := make([]float64, len(mat))
	for i := 0; i < len(mat); i++ {
		result[i] += s * mat[i]
	}
	return result
}

func (a *Perceptron) initialize() { //Random Initialization
	rand.Seed(time.Now().UnixNano())
	a.bias = 0.0
	a.weights = make([]float64, len(a.input[0]))
	for i := 0; i < len(a.input[0]); i++ {
		a.weights[i] = rand.Float64()
	}
}

func (a *Perceptron) sigmoid(x float64) float64 { //Sigmoid Activation
	return 1.0 / (1.0 + math.Exp(-x))
}

func (a *Perceptron) forwardPass(x []float64) (sum float64) { //Forward Propagation
	return a.sigmoid(dotProduct(a.weights, x) + a.bias)
}

func (a *Perceptron) gradW(x []float64, y float64) []float64 { //Calculate Gradients of Weights
	pred := a.forwardPass(x)
	return  scalarMatMul(-(pred-y)*pred*(1-pred), x)
}

func (a *Perceptron) gradB(x []float64, y float64) float64 { //Calculate Gradients of Bias
	pred := a.forwardPass(x)
	return -(pred - y) * pred * (1 - pred)
}


func (a *Perceptron) train() { //Train the Perceptron for n epochs
	start := time.Now()
	for i := 0; i < a.epochs; i++ {
		dw := make([]float64, len(a.input[0]))
		db := 0.0
		for length, val := range a.input {
			dw = vecAdd(dw, a.gradW(val, a.actualOutput[length]))
			db += a.gradB(val, a.actualOutput[length])
		}
		dw = scalarMatMul(2 / float64(len(a.actualOutput)), dw)
		a.weights = vecAdd(a.weights, dw)
		a.bias += db * 2 / float64(len(a.actualOutput))
	}
	stop := time.Now()
	elapsed := stop.Sub(start)
	fmt.Println(elapsed)
}

