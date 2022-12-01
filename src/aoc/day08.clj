(ns aoc.day08
  (:require [clojure.string :as string]))

(defn parse [input]
  (string/split-lines input))

(defn- str-len [s]
  (- (count (string/replace s #"(?i)\\\\|\\\"|\\x[0-9a-f]{2}" "x")) 2))

(defn line-value [s]
  (- (count s) (str-len s)))

(defn- expanded-len [s]
  (+ 2 (count s) (count (re-seq #"\"|\\" s))))

(defn expanded-value [s]
  (- (expanded-len s) (count s)))

(defn part1 [lines]
  (reduce #(+ %1 (line-value %2)) 0 lines))

(defn part2 [lines]
  (reduce #(+ %1 (expanded-value %2)) 0 lines))
