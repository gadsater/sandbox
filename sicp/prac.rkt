 
#lang racket

#|
;; factorial recursive definition
(define (factorial n)
  (if (= n 0)
      1
      (* n (factorial (- n 1)))))

;; factorial iterative definition

(define (factorial n)

  (define (fact-iter product counter)
    (if (> counter n)
        product
        (fact-iter (* counter product)
                   (+ counter 1))))
                   
  (fact-iter 1 1))

(define (plus a b)

  (define (dec r)
    (- r 1))

  (define (inc r)
    (+ r 1))
  
  (if (= a 0) b (inc (plus (dec a) b))))

;; my fib iter procedure
(define (fib n)
  (define (fib-iter prev curr iter)
    (cond
      [(= n 0) prev]
      [(= n 1) curr]
      [else (if (= (+ iter 1) n)
            curr
            (fib-iter curr
                      (+ prev curr)
                      (+ iter 1)))]))
  (fib-iter 0 1 0))

;;fib iter described in the book
(define (fib n)
  (define (fib-iter a b c)
    (if (= c 0)
        b
        (fib-iter (+ a b) a (- c 1))))
  (fib-iter 1 0 n))

;; counting change problem

(define (count-change amount) (cc amount 5))

(define (cc amount kinds-of-coins)
  (cond
    [(= amount 0) 1]
    [(or (< amount 0) (= kinds-of-coins 0)) 0]
    [else (+ (cc amount (- kinds-of-coins 1))
             (cc (- amount (first-denomination kinds-of-coins))
                kinds-of-coins))]))

(define (first-denomination kinds-of-coins)
  (cond
    [(= kinds-of-coins 1) 1]
    [(= kinds-of-coins 2) 2]
    [(= kinds-of-coins 3) 5]
    [(= kinds-of-coins 4) 10]
    [(= kinds-of-coins 5) 20]))

(define (test n)
  (when (< n 1) (format t "~a~%" "hello"))
  (when (< n 2) (format t "~a~%" "world"))
  (when (< n 3) (format t "~a~%" "hola")))
|#

