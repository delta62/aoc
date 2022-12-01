(ns aoc.day09
  (:require [clojure.string :as string]))

(defn- parse-line [state line]
  (let [[_ from to dist] (re-matches #"(\w+) to (\w+) = (\d+)" line)
        dist (parse-long dist)]
    (update state to #(assoc %1 from dist))))

(defn parse [input]
  (reduce parse-line {} (string/split-lines input)))


(defn part1 [state]
  42)

(comment
  (parse "London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141")
  )
