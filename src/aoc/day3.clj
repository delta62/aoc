(ns aoc.day3)

(def ^:private x first)
(def ^:private y second)

(defn- next-pos [cur-pos dir]
  (case dir
    \< [(dec (x cur-pos)) (y cur-pos)]
    \> [(inc (x cur-pos)) (y cur-pos)]
    \^ [(x cur-pos) (dec (y cur-pos))]
    \v [(x cur-pos) (inc (y cur-pos))]))

(defn- alternate
  ([coll] (alternate coll [] [] 0))
  ([coll lft rgt i]
   (if (empty? coll)
     [lft rgt]
     (if (even? i)
       (recur (rest coll) (conj lft (first coll)) rgt (inc i))
       (recur (rest coll) lft (conj rgt (first coll)) (inc i))))))

(defn- locations
  ([input] (locations input [0 0] #{}))
  ([input pos seen]
   (if (empty? input)
     (conj seen pos)
     (let [next (next-pos pos (first input))]
       (recur (rest input) next (conj seen pos))))))

(defn- dual-locations [input]
  (apply into (map locations (alternate input))))

(defn part1 [input] (count (locations input)))

(defn part2 [input] (count (dual-locations input)))
