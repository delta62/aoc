(ns aoc.collections)

(defn slice [start len coll] (take len (drop start coll)))

(defn window [n coll]
  (cond
    (empty? coll) '()
    (< (count coll) n) [(take n coll)]
    :else (map #(slice %1 n coll) (range (- (count coll) n -1)))))
