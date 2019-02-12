(defun fibonacci-series (num)
  (do ((n 0 (1+ n))
       (cur 0 next)
       (next 1 (+ cur next)))
      ((= num n)
       (terpri))
    (print cur)))

  
