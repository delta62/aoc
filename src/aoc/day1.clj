(ns aoc.day1)

(defn- str-chars
  "Convert a string into a list of chars"
  [str]
  (chars (char-array str)))

(defn- is-leftparen
  "True when `char` is (, false otherwise"
  [char]
  (= char \())

(defn- count-parens [acc, x]
  (if (is-leftparen x)
    (+ acc 1)
    (- acc 1)))

(defn- oob
  "true when the given index is out of bounds from the collection, or false otherwise"
  [coll idx]
  (>= idx (count coll)))

(defn part1 [input]
  (reduce count-parens 0 (str-chars input)))

(defn part2
  ([input] (part2 input 0 0))
  ([input depth idx]
   (if (oob input idx)
     0
     (let [next-depth (if (is-leftparen (get input idx)) (inc depth) (dec depth))]
       (if (= next-depth -1)
         (inc idx)
         (recur input next-depth (inc idx)))))))
