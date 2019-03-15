(ns alpha.data-struct)

(defn cons-btree-list
  ([tree-vec]
   (cons-btree-list tree-vec 0))
  ([tree-vec index]
   (let [l-ind (inc (* 2 index))
         r-ind (inc l-ind)
         elem (get tree-vec index)]
     (if (nil? elem)
       '()
       (list elem (cons-btree-list tree-vec l-ind)
             (cons-btree-list tree-vec r-ind))))))

(defn cons-btree-vec
  ([tree-list]
   (cons-btree-vec tree-list 0))
  ([tree-list index]
   ))
(cons-btree-list [1 2 3 4 5 6 7 8 9])


