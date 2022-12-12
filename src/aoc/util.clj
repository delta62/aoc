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

(defn lowercase-ascii-to-decimal [c]
  (- (int c) 97))

(def diff
  "Gives the difference between the two numbers as an absolute value"
  (comp abs -))

(defn map-values
  "Map over the values of a map"
  [m f]
  (reduce (fn [state [k v]] (assoc state k (f v))) {} m))

(defn key-of
  "Find the key in `m` which matches the given predicate, or `nil` if nothing
   matches"
  [m f]
  (some (fn [[k v]] (when (f v) k)) m))
