(ns aoc.day01
  (:require [clojure.string :as string]))

(defn parse [s]
  (let [groups (string/split s #"\n\n")
        to-nums (partial map parse-long)]
    (map (comp to-nums string/split-lines) groups)))

(defn- sum [nums]
  (apply + nums))

(defn part1 [input]
  (apply max (map sum input)))

(defn part2 [input]
  (apply + (take 3 (sort > (map sum input)))))
