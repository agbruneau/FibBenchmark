package main

/*
#include <stdint.h>
*/
import "C"

// Matrix2x2 represents a 2x2 matrix for Fibonacci calculation
type Matrix2x2 struct {
	a, b, c, d uint64
}

// FibIterative calculates Fibonacci using iterative method - O(n)
//
//export FibIterative
func FibIterative(n C.uint64_t) C.uint64_t {
	if n <= 1 {
		return C.uint64_t(n)
	}

	var a, b uint64 = 0, 1
	for i := uint64(2); i <= uint64(n); i++ {
		a, b = b, a+b
	}
	return C.uint64_t(b)
}

// FibRecursive calculates Fibonacci using naive recursive method - O(2^n)
// WARNING: Very slow for n > 35
//
//export FibRecursive
func FibRecursive(n C.uint64_t) C.uint64_t {
	return C.uint64_t(fibRecursiveGo(uint64(n)))
}

func fibRecursiveGo(n uint64) uint64 {
	if n <= 1 {
		return n
	}
	return fibRecursiveGo(n-1) + fibRecursiveGo(n-2)
}

// FibMemo calculates Fibonacci with memoization - O(n)
//
//export FibMemo
func FibMemo(n C.uint64_t) C.uint64_t {
	memo := make(map[uint64]uint64)
	return C.uint64_t(fibMemoGo(uint64(n), memo))
}

func fibMemoGo(n uint64, memo map[uint64]uint64) uint64 {
	if n <= 1 {
		return n
	}
	if val, ok := memo[n]; ok {
		return val
	}
	result := fibMemoGo(n-1, memo) + fibMemoGo(n-2, memo)
	memo[n] = result
	return result
}

// matrixMultiply multiplies two 2x2 matrices
func matrixMultiply(m1, m2 Matrix2x2) Matrix2x2 {
	return Matrix2x2{
		a: m1.a*m2.a + m1.b*m2.c,
		b: m1.a*m2.b + m1.b*m2.d,
		c: m1.c*m2.a + m1.d*m2.c,
		d: m1.c*m2.b + m1.d*m2.d,
	}
}

// matrixPower calculates matrix power using fast exponentiation
func matrixPower(m Matrix2x2, n uint64) Matrix2x2 {
	if n == 0 {
		return Matrix2x2{a: 1, b: 0, c: 0, d: 1} // Identity matrix
	}
	if n == 1 {
		return m
	}

	result := Matrix2x2{a: 1, b: 0, c: 0, d: 1} // Identity
	base := m

	for n > 0 {
		if n%2 == 1 {
			result = matrixMultiply(result, base)
		}
		base = matrixMultiply(base, base)
		n /= 2
	}

	return result
}

// FibMatrix calculates Fibonacci using matrix exponentiation - O(log n)
//
//export FibMatrix
func FibMatrix(n C.uint64_t) C.uint64_t {
	if n == 0 {
		return 0
	}

	fibMatrix := Matrix2x2{a: 1, b: 1, c: 1, d: 0}
	result := matrixPower(fibMatrix, uint64(n))
	return C.uint64_t(result.b)
}

// FibDoubling uses the doubling method - O(log n)
// F(2k) = F(k) * (2*F(k+1) - F(k))
// F(2k+1) = F(k)^2 + F(k+1)^2
//
//export FibDoubling
func FibDoubling(n C.uint64_t) C.uint64_t {
	return C.uint64_t(fibDoublingGo(uint64(n)))
}

func fibDoublingGo(n uint64) uint64 {
	if n == 0 {
		return 0
	}
	return fibDoublingHelper(n)[0]
}

// Returns (F(n), F(n+1))
func fibDoublingHelper(n uint64) [2]uint64 {
	if n == 0 {
		return [2]uint64{0, 1}
	}

	pair := fibDoublingHelper(n / 2)
	fk := pair[0]
	fk1 := pair[1]

	// F(2k) = F(k) * (2*F(k+1) - F(k))
	f2k := fk * (2*fk1 - fk)
	// F(2k+1) = F(k)^2 + F(k+1)^2
	f2k1 := fk*fk + fk1*fk1

	if n%2 == 0 {
		return [2]uint64{f2k, f2k1}
	}
	return [2]uint64{f2k1, f2k + f2k1}
}

// GetGoVersion returns the Go version as a string
//
//export GetGoVersion
func GetGoVersion() *C.char {
	return C.CString("go1.25.5")
}

// main is required for CGO but won't be called
func main() {}
