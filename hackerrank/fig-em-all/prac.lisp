(format t "~a~%" (let ((a (read)) (b (read)))
		   (when (< b a)
		     (setf a (+ a b))
		     (setf b (- a b))
		     (setf a (- a b)))
		   (loop for i from a to b
		    summing i into total
		    finally (return total))))
