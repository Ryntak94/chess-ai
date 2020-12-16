# chess-ai

## Path Forward
Begone, go-perceptron! rust-perceptron is fully implemented and runs a lil faster.
Yes, I'm still thinking about sticking with go for the chess move generation. After considering the options in rust, this library seems better suited.
Plus, it'll be fun to learn how to integrate the two :) 
Although I did learn about bit boards from one of the rust implementations, which made it worthwhile.

I've added a Dockerfile which installs both go and rust. So far it's good for development. 

## Weights and Conversions
Possibly the biggest challenge for us will be representing the state of the chessboard as a float. 
Not just this, but also how to judge whether an action is good or bad.
For example, in clasical scoring, taking a knight is 3 points.
However, if taking that knight means your own checkmate next turn, it should be considered a poor choice.
We'll need the model to consider not only each move, but also the overall game.