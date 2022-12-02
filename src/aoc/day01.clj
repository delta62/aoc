(ns aoc.day01
  (:require [clojure.string :as string]))

(defn- sum [nums]
  (apply + nums))

(defn parse
  "Parses a newline-separated string into nested vectors of numbers"
  [s]
  (let [groups (string/split s #"\n\n")
        to-nums (partial map parse-long)]
    (map (comp to-nums string/split-lines) groups)))

(defn part1 [input]
  (apply max (map sum input)))

(defn part2 [input]
  (->> input
       (map sum)
       (sort >)
       (take 3)
       (sum)))
