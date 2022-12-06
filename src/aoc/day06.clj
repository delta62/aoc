(ns aoc.day06)

(defn unique-sequence-index
  "Returns a pair where the first item is the passed `index`, and the second
   is `true` when `coll` contains at least `n` unique items, or `false`
   otherwise"
  [n index coll]
  [(+ n index) (= n (count (set coll)))])

(defn index-matches?
  "Returns `index` if `is-match` is truthy, or `false` otherwise"
  [[index is-match]]
  (if is-match index false))

(defn find-prefix-index
  "Find the first index in `coll` with `n` unique contiguous items"
  [n coll]
  (->> (partition n 1 coll)
       (map-indexed (partial unique-sequence-index n))
       (some index-matches?)))

(defn part1 [s] (find-prefix-index 4 s))

(defn part2 [s] (find-prefix-index 14 s))
