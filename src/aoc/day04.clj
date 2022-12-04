(ns aoc.day04
  (:require [clojure.string :as string]))

(defn parse-line
  "Split a line of text into two pairs of two numbers"
  [line]
  (->> (string/split line #"[^\d]")
       (map parse-long)
       (partition 2)))

(defn parse [s]
  (->> s
       (string/split-lines)
       (map parse-line)))

(defn subset?
  "Returns true when either passed range is a subset of the other"
  [[[min1 max1] [min2 max2]]]
  (or
   (and (<= min1 min2) (>= max1 max2))
   (and (<= min2 min1) (>= max2 max1))))

(defn overlap?
  "Returns true when the passed ranges overlap one another"
  [[[min1 max1] [min2 max2] :as pairs]]
  (or
   (and (<= min1 max2) (>= min1 min2))
   (and (<= max1 max2) (>= max1 min2))
   (subset? pairs)))

(defn part1 [pairs]
  (count (filter subset? pairs)))

(defn part2 [pairs]
  (count (filter overlap? pairs)))
