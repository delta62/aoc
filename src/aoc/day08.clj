(ns aoc.day08
  (:require [clojure.string :as string]))

(defn parse-digit [c]
  (parse-long (str c)))

(defn parse [s]
  (map #(map parse-digit %) (string/split-lines s)))

(defn width [forest]
  (count (first forest)))

(defn height [forest]
  (count forest))

(defn visible [[tallest state] [height idx]]
  (if (> height tallest)
    [height (conj state idx)]
    [tallest state]))

(defn vis-from-left [forest]
  (apply concat
   (for [y (range (height forest))]
     (map #(vector % y)
      (last
       (reduce visible [-1 []]
        (map #(vector %2 %1) (range) (nth forest y))))))))

(defn vis-from-right [forest]
  (let [mirrored (map reverse forest)
        visible (vis-from-left mirrored)]
    (map (fn [[x y]] [(- (dec (width forest)) x) y]) visible)))

(defn vis-from-top [forest]
  (let [rotated (apply map vector forest)
        visible (vis-from-left rotated)]
    (map (fn [[y x]] [x y]) visible)))

(defn vis-from-bot [forest]
  (let [rotated (map reverse (apply map vector forest))
        visible (vis-from-left rotated)]
    (map (fn [[y x]] [x (- (dec (height forest)) y)]) visible)))

(defn each-coord [forest]
  (for [x (range (width forest))
        y (range (height forest))]
    [x y]))

(defn count-visible [forest [height-limit count] x y]
  (let [tree (nth (nth forest y) x)
        visible? (< tree height-limit)]
    (if visible?
      [height-limit (inc count)]
      (if (= -1 height-limit)
        [-1 count]
        [-1 (inc count)]))))

(defn scenic-score [[x y] forest]
  (let [height-limit (nth (nth forest y) x)
        height (count forest)
        width (count (first forest))
        up-score (last (reduce #(count-visible forest %1 x %2) [height-limit 0] (range (dec y) -1 -1)))
        down-score (last (reduce #(count-visible forest %1 x %2) [height-limit 0] (range (inc y) height)))
        left-score (last (reduce #(count-visible forest %1 %2 y) [height-limit 0] (range (dec x) -1 -1)))
        right-score (last (reduce #(count-visible forest %1 %2 y) [height-limit 0] (range (inc x) width)))]
    (* up-score left-score right-score down-score)))

(defn part1 [input]
  (-> (into #{} (vis-from-left input))
      (into (vis-from-right input))
      (into (vis-from-top input))
      (into (vis-from-bot input))
      (count)))

(defn part2 [input]
  (apply max (map #(scenic-score % input) (each-coord input))))
