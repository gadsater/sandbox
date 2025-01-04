package main

import (
	"fmt"
	"math/rand"
	"strings"
	"time"
)

type Difficulty uint8

const (
	novice  Difficulty = 0
	easy               = 1
	medium             = 2
	hard               = 3
	extreme            = 4
)

type Sudoku struct {
	board [9][9]uint8
}

func (sudoku *Sudoku) PrintSudoku() {
	for i := range sudoku.board {
		if i%3 == 0 {
			fmt.Printf("%s\n", strings.Repeat("-", 25))
		}
		for j := range sudoku.board[i] {
			if j%3 == 0 {
				fmt.Printf("| ")
			}
			if sudoku.board[i][j] == 0 {
				fmt.Print(". ")
			} else {
				fmt.Printf("%d ", sudoku.board[i][j])
			}
		}
		fmt.Printf("|")
		fmt.Printf("\n")
	}
	fmt.Printf("%s\n", strings.Repeat("-", 25))
}

func (sudoku *Sudoku) CreateSudoku(difficulty Difficulty) {
	seed := generateRandomNumber()

	sudoku.board = [9][9]uint8{}

	row := rand.Intn(9)
	col := rand.Intn(9)

	sudoku.board[row][col] = seed
	sudoku.populateBoard(0, 0)

	sudoku.addBlanks(difficulty)
}

func (sudoku *Sudoku) addBlanks(difficulty Difficulty) {
	var count uint8
	switch difficulty {
	case easy:
		count = 15
		break
	case medium:
		count = 20
		break
	case hard:
		count = 25
		break
	case extreme:
		count = 30
		break
	case novice:
	default:
		count = 10
		break
	}

	populatedQuadrant := [3][3]uint8{}

	numInserted := uint8(0)
	for numInserted != count {

		// randomly pick a quadrant which has not been populated before
		quadR, quadC := rand.Intn(3), rand.Intn(3)

		for !(populatedQuadrant[quadR][quadC] == numInserted/9) {
			quadR, quadC = rand.Intn(3), rand.Intn(3)
		}

		row, col := uint8(rand.Intn(3)+(quadR*3)), uint8(rand.Intn(3)+(quadC*3))

		for sudoku.board[row][col] == 0 {
			row, col = uint8(rand.Intn(3)+(quadR*3)), uint8(rand.Intn(3)+(quadC*3))
		}

		populatedQuadrant[row/3][col/3] += 1

		sudoku.board[row][col] = 0
		numInserted += 1
	}
}

func (sudoku *Sudoku) populateBoard(row, col int) bool {
	// If we've reached the last cell, the board is filled
	if row == 9 {
		return true
	}

	// Move to the next row if we've reached the end of a column
	nextRow, nextCol := row, col+1
	if nextCol == 9 {
		nextRow, nextCol = row+1, 0
	}

	// If the current cell is already filled, skip it
	if sudoku.board[row][col] != 0 {
		return sudoku.populateBoard(nextRow, nextCol)
	}

	check := map[uint8]bool{}
	num := generateUnseenRandomNumber(check)
	for !isAllNumbersGenerated(check) {
		if sudoku.isValidPlacement(row, col, num) {
			// Place the number
			sudoku.board[row][col] = num

			// Recurse to the next cell
			if sudoku.populateBoard(nextRow, nextCol) {
				return true
			}

			// Backtrack if placing the number doesn't lead to a solution
			sudoku.board[row][col] = 0
		}

		num = generateUnseenRandomNumber(check)
	}

	// Return false if no number can be placed in the current cell
	return false
}

func generateUnseenRandomNumber(check map[uint8]bool) uint8 {
	randNum := generateRandomNumber()

	for check[randNum] == true {
		randNum = generateRandomNumber()
	}

	check[randNum] = true
	return randNum
}

func generateRandomNumber() uint8 {
	return uint8(rand.Intn(9) + 1)
}

func isAllNumbersGenerated(check map[uint8]bool) bool {
	for i := uint8(1); i < 10; i++ {
		if check[i] == false {
			return false
		}
	}

	return true
}

func (sudoku *Sudoku) isValidPlacement(row, col int, num uint8) bool {
	// Check the row and column
	for i := 0; i < 9; i++ {
		if sudoku.board[row][i] == num || sudoku.board[i][col] == num {
			return false
		}
	}

	// Check the 3x3 subgrid
	startRow, startCol := (row/3)*3, (col/3)*3
	for i := 0; i < 3; i++ {
		for j := 0; j < 3; j++ {
			if sudoku.board[startRow+i][startCol+j] == num {
				return false
			}
		}
	}

	return true
}

func main() {
	rand.New(rand.NewSource(time.Now().Unix()))
	sudoku := &Sudoku{board: [9][9]uint8{}}

	sudoku.CreateSudoku(extreme)
	sudoku.PrintSudoku()
}


//
// -------------------------
// | 1 4 . | 6 9 3 | 7 2 8 |
// | 9 . 3 | 4 . 2 | . 1 . |
// | 6 . 7 | . . 1 | 4 . 9 |
// -------------------------
// | 5 1 . | 9 3 4 | 2 . 7 |
// | 7 9 . | 1 2 . | 8 . . |
// | . . 2 | . . 5 | . 9 4 |
// -------------------------
// | 4 5 1 | . . . | 9 7 2 |
// | . 3 . | 2 . 7 | . 4 5 |
// | . 7 6 | 5 4 9 | 3 . . |
// -------------------------
//
// -------------------------
// | 1 4 5 | 6 9 3 | 7 2 8 |
// | 9 8 3 | 4 7 2 | 5 1 6 |
// | 6 2 7 | 8 5 1 | 4 3 9 |
// -------------------------
// | 5 1 8 | 9 3 4 | 2 6 7 |
// | 7 9 4 | 1 2 6 | 8 5 3 |
// | 3 6 2 | 7 8 5 | 1 9 4 |
// -------------------------
// | 4 5 1 | 3 6 8 | 9 7 2 |
// | 8 3 9 | 2 1 7 | 6 4 5 |
// | 2 7 6 | 5 4 9 | 3 8 1 |
// -------------------------