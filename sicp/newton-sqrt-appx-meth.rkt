#lang racket
;;; Newton's square root approximation method

#|
(define (sqrt-iter num guess)
  "guesses square of num until good-enough guess is achieved"
  (if (good-enough? num guess)
      guess
      (sqrt-iter num (improve num guess))))

(define (improve num guess)
  "improves the value of guess"
  (average guess (/ num guess)))

(define (average x y)
  "average of two numbers x and y"
  (/ (+ x y) 2))

(define (good-enough? num guess)
  "good-enough predicate for the tolerance of error
 from actual value"
  (< (abs (- (square guess) num)) 0.001))

(define (square x)
  "calculates square of a number"
  (* x x))

(define (sqrt x)
  "Finds square root of x"
  (sqrt-iter x 1.0))
|#

;; same program following block structure
(define (sqrt x)
  (define (square n) (* n n))
  (define (average m n) (/ (+ m n) 2))
  (define (good-enough? guess)
    (< (abs (- (square guess) x)) 0.001))
  (define (improve guess) (average guess (/ x guess)))
  (define (sqrt-iter guess)
    (if (good-enough? guess)
        guess
        (sqrt-iter (improve guess))))
    (sqrt-iter 1.0))
  