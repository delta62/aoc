(ns aoc.day5
  (:require [clojure.string :as string]))

(defn- has-vowels? [n s]
  (>= (count (re-seq #"[aeiou]" s)) n))

(defn- has-double?
  ([s] (has-double? nil s))
  ([last s]
   (let [next (first s)]
     (cond
       (= last next) true
       (nil? next) false
       :else (recur next (rest s))))))

(defn- restricted? [s]
  (some? (re-find #"ab|cd|pq|xy" s)))

(defn- is-nice? [s]
  (and (has-vowels? 3 s) (has-double? s) (not (restricted? s))))

(defn part1 [input]
  (count (filter is-nice? (string/split-lines input))))
