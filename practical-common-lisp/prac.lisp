(defun primep (number)
  (when (> number 1)
    (loop for fac from 2 to (isqrt number)
       never (zerop (mod number fac)))))

(defun next-prime (number)
  (loop for n from number when (primep n) return n))

(defun collect-primes (num-start num-end)
  (do ((n num-start (1+ n)) primes)
      ((> n num-end) (reverse primes))
    (when (primep n)
      (setf primes (cons n primes)))))

#||
(defmacro do-primes (var-and-range &rest body)
  (let ((var (first var-and-range))
	(start (second var-and-range))
	(end (third var-and-range)))
    `(do ((,var (next-prime ,start) (next-prime (1+ ,var))))
	 ((> ,var ,end))
       ,@body)))
||#

#||
(defmacro do-primes ((var start end) &body body)
  `(do ((,var (next-prime ,start) (next-prime (1+ ,var))))
       ((> ,var ,end))
     ,@body))
||#

#||
(defmacro do-primes ((var start end) &body body)
  `(do ((,var (next-prime ,start) (next-prime (1+ ,var)))
	(ending-value ,end))
       ((> ,var ending-value))
     ,@body))
||#

(defmacro do-primes ((var start end) &body body)
  (let ((ending-value (gensym)))
    `(do ((,var (next-prime ,start) (next-prime (1+ ,var)))
	  (,ending-value ,end))
	 ((> ,var ,ending-value))
       ,@body)))