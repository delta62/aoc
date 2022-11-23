(ns aoc.day1)

(def ^:private leftparen? (partial = \())

(defn- count-parens [acc, x]
  (if (leftparen? x) (inc acc) (dec acc)))

(defn part1 [input] (reduce count-parens 0 input))

(defn part2
  ([input] (part2 input 0 1))
  ([input depth i]
   (cond
     (leftparen? (first input)) (recur (rest input) (inc depth) (inc i))
     (zero? depth) i
     :else (recur (rest input) (dec depth) (inc i)))))
