(ns aoc.day06)

(defn sequence-index
  "Returns `index` when `coll` is a sequence of `n` unique items, or false
   otherwise"
  [n index coll]
  (and
   (-> (set coll) (count) (= n))
   index))

(defn find-uniques
  "Find the first index in `coll` with `n` unique contiguous items"
  [n coll]
  (->> (partition n 1 coll)
       (map-indexed (partial sequence-index n))
       (some identity)
       (+ n)))

(defn part1 [s] (find-uniques 4 s))

(defn part2 [s] (find-uniques 14 s))
