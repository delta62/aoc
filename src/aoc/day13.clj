(ns aoc.day13
  (:require [aoc.util :refer [ascii-digit? product sum]]
            [clojure.string :as string]))

(defn parse-num
  "Reads digits from `s`, returning a parsed number and the rest of the unused
   characters in `s`"
  [s]
  (let [cs (take-while ascii-digit? s)
        num-length (count cs)
        rest (drop num-length s)
        num (parse-long (string/join cs))]
    [num rest]))

(defn parse-list
  "Reads a list literal from `s`, returning a parsed list and the rest of the
   unused characters in `s`. Lists consist of either nested lists or numbers."
  ([s] (parse-list [] (rest s)))
  ([list [next & rest :as s]]
   (case next
     \[ (let [[l rest] (parse-list [] rest)]
          (recur (conj list l) rest))
     \] [list rest]
     \, (recur list rest)
     (let [[num rest] (parse-num s)]
       (recur (conj list num) rest)))))

(defn parse-pair [s]
  (map #(first (parse-list %)) (string/split-lines s)))

(defn parse-part1 [s]
  (map parse-pair (string/split s #"\n\n")))

(defn parse-part2 [s]
  (->> (string/split-lines s)
       (filter (comp not empty?))
       (map (comp first parse-list))))

(defn in-order? [[l & ls :as left] [r & rs :as right]]
  (cond
    (and (empty? left) (empty? right)) :continue
    (empty? left) true
    (empty? right) false
    (and (int? l) (int? r)) (if (= l r) (recur ls rs) (< l r))
    (and (coll? l) (coll? r)) (let [result (in-order? l r)]
                                (or (true? result) (and (= :continue result) (recur ls rs))))
    (and (coll? l) (int? r))  (let [result (in-order? l [r])]
                                (or (true? result) (and (= :continue result) (recur ls rs))))
    (and (int? l)  (coll? r)) (let [result (in-order? [l] r)]
                                (or (true? result) (and (= :continue result) (recur ls rs))))
    :else (throw (Exception. "Unexpected state"))))

(defn part1 [input]
  (->> (map #(apply in-order? %) input)
       (map-indexed vector)
       (filter second)
       (map (comp inc first))
       (sum)))

(defn is-marker? [x]
  (let [vals (set (flatten x))]
    (or (= vals #{2}) (= vals #{6}))))

(defn part2 [input]
  (->> (conj input [[2]] [[6]])
       (sort-by identity in-order?)
       (map-indexed #(vector (inc %1) %2))
       (filter (comp is-marker? second))
       (map first)
       (product)))
