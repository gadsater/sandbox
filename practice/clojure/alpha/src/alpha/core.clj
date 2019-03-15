(ns alpha.core
  (:require [clojure.string :as str]))

(seq "Hello World")

(frequencies (str/lower-case "An adult all about A's"))

(defn yelling? [s]
  (every? #(or (not (Character/isLetter %)) (Character/isUpperCase %)) s))

(yelling? "LOUD NOISES!")

(yelling? "Take a DEEP breath.")

(apply str (seq "Hello, World!"))

(int \ )

(map int "Hello, World!")

(map char (range 0x0410 0x042F))

(apply str (interpose " " [1 2.000 (/ 3 1) (/ 4 9)]))

(def mentions #"@\w+|#\w+")

(re-seq mentions "@broth How is your day? #DoomsDay.")

(str/split "HEADER1   HEADER2\n\nHEADER3\n" #"\s+")

(keyword 'split)

(str :hello)

(name :hello)

(Integer/toString 97 16)

(defn mean [coll]
  (let [sum (apply + coll)
        cnt (count coll)]
    (if (pos? cnt)
      (/ sum cnt)
      0)))

(defn median [coll]
  (let [sorted (into [] (sort coll))
        cnt (count sorted)
        mid (int (/ cnt 2))
        get-mid (partial get sorted)]
    (if (odd? cnt)
      (get-mid mid)
      (/ (+ (get-mid mid) (get-mid (dec mid))) 2))))

(defn mode [coll]
  (let [freqs (frequencies coll)
        occurences (group-by second freqs)
        modes (last (sort occurences))
        modes (->> modes
                   second
                   (map first))]
    modes))

(mode [:alan :bob :alan :greg])






