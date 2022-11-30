(ns aoc.day08
  (:require [clojure.string :as string]))

(defn parse [input]
  (string/split-lines input))

(defn- str-len [s]
  (- (count (string/replace s #"(?i)\\\\|\\\"|\\x[0-9a-f]{2}" "x")) 2))

(defn line-value [s]
  (- (count s) (str-len s)))

(defn part1 [lines]
  (reduce #(+ %1 (line-value %2)) 0 lines))

(comment
  (str-len "\\x4F")
  (parse "\"\"\nabc")
  (line-value (first (parse "\"\"")))
  )
