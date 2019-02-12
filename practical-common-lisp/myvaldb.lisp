(defvar *store-db* nil)
(defvar *curr-db-ind* nil)

(defun create-db (dbname)
  (vector-push (list :dbname dbname :table nil) *store-db*))

(defun use-db (dbname)
  (setf *curr-db-ind* (position dbname *store-db* :key #'first)))

(defun create-table (tbname &rest fields)
  (push (list :tbname tbname :fields fields :tbdata nil) (getf (elt *store-db* *curr-db-ind*) :table)))




  
