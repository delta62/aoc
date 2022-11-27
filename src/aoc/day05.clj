(ns aoc.day05
  (:require [clojure.string :as string])
  (:require [aoc.collections :refer [window]]))

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

(defn- contains-double-pair?
  ([s] (contains-double-pair? s nil 0 {}))
  ([s last idx found]
   (let [next (first s)
         rest (rest s)]
     (cond
       (nil? next) false
       (nil? last) (recur rest next (inc idx) found)
       :else (let [pair (str last next)]
               (if (contains? found pair)
                 (if (< (get found pair) (dec idx))
                   true
                   (recur rest next (inc idx) found))
                 (recur rest next (inc idx) (assoc found pair idx))))))))

(defn- first-equals-last? [coll] (= (first coll) (last coll)))

(defn- has-alternating? [s]
  (some? (some first-equals-last? (window 3 s))))

(defn- is-nice'? [s]
  (and (contains-double-pair? s) (has-alternating? s)))

(defn part1 [input]
  (count (filter is-nice? (string/split-lines input))))

(defn part2 [input]
  (count (filter is-nice'? (string/split-lines input))))
