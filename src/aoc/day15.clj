(ns aoc.day15
  (:require [aoc.util :refer [diff]]
            [clojure.string :as string]))

(defn parse-line [s]
  (->> (re-seq #"-?\d+" s)
       (map parse-long)
       (partition 2)))

(defn parse [s]
  (map parse-line (string/split-lines s)))

(defn manhattan
  "Returns the Manhattan distance between two points"
  [[x1 y1] [x2 y2]]
  (+ (diff x1 x2) (diff y1 y2)))

(def beacon-x
  "Gives the x position of a beacon given an input line"
  (comp second second))

(defn impossible-spaces
  "Given a y coordinate, a point, and the closest beacon to that point, returns
   all of the x coordinates which could not possibly have a beacon on them"
  [y [[bx by :as beacon] point]]
  (let [radius (manhattan point beacon)
        dy (diff by y)
        xlen (- (inc (* 2 radius)) (* 2 dy))
        startx (- bx (quot xlen 2))
        endx (+ startx xlen)]
    (if (pos-int? xlen) (set (range startx endx)) #{})))

(defn beacon-x-positions [input line]
  (->> (map beacon-x input)
       (filter (partial = line))
       (set)))

(defn all-impossible-on-line [input line]
  (let [beacon-xs (beacon-x-positions input line)
        dead-zone (reduce
                   #(into %1 (impossible-spaces line %2))
                   #{}
                   input)]
    (apply disj dead-zone beacon-xs)))

(defn part1 [input]
  (count (all-impossible-on-line input 2000000)))
