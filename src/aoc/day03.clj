(ns aoc.day03
  (:require [aoc.util :refer [sum]]
            [clojure.set :refer [intersection]]
            [clojure.string :refer [split-lines]]))

(defn- compartments
  "Split a string into two sets of characters"
  [s]
  (let [len (count s)
        n (/ len 2)]
    (map set (partition n s))))

(defn- all-contain
  "Finds the first item common to all passed collections"
  [colls]
  (first (apply intersection colls)))

(defn- prioritize
  "Return the priority number for a given character"
  [c]
  (let [x (int c)]
    (if (and (>= x 97) (<= x 122))
      (- x 96)
      (- x 38))))

(defn parse [s]
  (split-lines s))

(defn part1 [input]
  (->> input
       (map compartments)
       (map all-contain)
       (map prioritize)
       (sum)))

(defn part2 [input]
  (->> input
       (partition 3)
       (map (partial map set))
       (map all-contain)
       (map prioritize)
       (sum)))
