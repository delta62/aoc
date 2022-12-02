(ns aoc.day01
  (:require [clojure.string :refer [split split-lines]]))

(defn- sum [nums]
  (apply + nums))

(defn parse
  "Parses a newline-separated string into nested lists of numbers"
  [s]
  (let [groups (split s #"\n\n")]
    (map #(->> %1
               (split-lines)
               (map parse-long))
         groups)))

(defn part1 [input]
  (apply max (map sum input)))

(defn part2 [input]
  (->> input
       (map sum)
       (sort >)
       (take 3)
       (sum)))
