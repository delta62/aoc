(ns aoc.util)

(defn sum 
  "Returns the sum of all numbers in a collection"
  [nums] 
  (apply + nums))

(defn product
  "Returns the product of all numbers in a collection"
  [nums]
  (apply *' nums))

(defn upper-case-ascii-char
  "Return the given character if it's an uppercase ascii letter, else nil"
  [c]
  (let [char-code (int c)]
    (if (and (>= char-code 65) (<= char-code 90)) c nil)))

(def diff
  "Gives the difference between the two numbers as an absolute value"
  (comp abs -))
