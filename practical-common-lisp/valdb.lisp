(defvar *db* nil)

(defun make-val (serial-no name weight benefit broth)
  (list :serial-no serial-no
	:name name
	:weight weight
	:benefit benefit
	:broth broth))

(defun add-record (val) (push val *db*))

#||
(defun dump-db ()
  (dolist (val *db*)
    (format t "~{~a:~11t~a~%~}~%" val)))
||#

(defun dump-db ()
  (format t "~{~{~a:~11t~a~%~}~%~}" *db*))

(defun prompt-read (prompt)
  (format *query-io* "~a: " prompt)
  (force-output *query-io*)
  (read-line *query-io*))

(defun prompt-for-val ()
  (make-val
   (or (parse-integer (prompt-read "Serial No") :junk-allowed t) 0)
   (prompt-read "Name")
   (or (parse-integer (prompt-read "Weight") :junk-allowed t) 0) 
   (or (parse-integer (prompt-read "Benefit") :junk-allowed t) 0)
   (y-or-n-p "Broth [y/n]: ")))

(defun add-records ()
  (loop (add-record (prompt-for-val))
     (if (not (y-or-n-p "Another? [y/n]: ")) (return))))

(defun save-db (filename)
  (with-open-file (out filename
		       :direction :output
		       :if-exists :supersede)
    (with-standard-io-syntax
      (print *db* out))))

(defun load-db (filename)
  (with-open-file (in filename)
    (with-standard-io-syntax
      (setf *db* (read in)))))

(defun select-by-name (name)
  (remove-if-not
   #'(lambda (val) (equal (getf val :name) name))
   *db*))

(defun select (selector-fn)
  (remove-if-not selector-fn *db*))

#||
(defun where (&key serial-no name weight benefit (broth nil broth-p))
  #'(lambda (val)
      (and
       (if serial-no (equal (getf val :serial-no) serial-no) t)
       (if name (equal (getf val :name) name) t)
       (if weight (equal (getf val :weight) weight) t)
       (if benefit (equal (getf val :benefit) benefit) t)
       (if broth-p (equal (getf val :broth) broth) t))))
||#

(defun update (selector-fn &key serial-no name weight benefit (broth nil broth-p))
  (setf *db*
	(mapcar
	 #'(lambda (row)
	     (when (funcall selector-fn row)
	       (if serial-no (setf (getf row :serial-no) serial-no))
	       (if name (setf (getf row :name) name))
	       (if weight (setf (getf row :weight) weight))
	       (if benefit (setf (getf row :benefit) benefit))
	       (if broth-p (setf (getf row :broth) broth)))
	     row) *db*)))

(defun delete-rows (selector-fn)
  (setf *db* (remove-if selector-fn *db*)))

#||
(defun make-comparison-expr (field value)
  (list 'equal (list 'getf 'val field) value))
||#

(defun make-comparison-expr (field value)
  `(equal (getf val ,field) ,value))

(defun make-comparisons-list (fields)
  (loop while fields
       collecting (make-comparison-expr (pop fields) (pop fields))))

(defmacro where (&rest clauses)
  `#'(lambda (val) (and ,@(make-comparisons-list clauses))))

