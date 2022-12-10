(ns aoc.day09
  (:require [aoc.util :refer [diff]]
            [clojure.string :as string]))

(defn parse-direction [s]
  (case s
    "U" :up
    "D" :down
    "L" :left
    "R" :right))

(defn parse-line [line]
  (let [[direction magnitude] (string/split line #" ")
        direction (parse-direction direction)
        magnitude (parse-long magnitude)]
    (repeat magnitude direction)))

(defn parse [s]
  (mapcat parse-line (string/split-lines s)))

(defn move-head [[x y] direction]
  (case direction
    :up    [x (inc y)]
    :down  [x (dec y)]
    :left  [(dec x) y]
    :right [(inc x) y]))

(defn move-tail-diagonally [[head-x head-y] [tail-x tail-y]]
  (let [x (if (> head-x tail-x) (inc tail-x) (dec tail-x))
        y (if (> head-y tail-y) (inc tail-y) (dec tail-y))]
    [x y]))

(defn move-tail-along-axis [[head-x head-y] [tail-x tail-y]]
  (cond
    (< head-x tail-x) [(dec tail-x) tail-y]
    (> head-x tail-x) [(inc tail-x) tail-y]
    (< head-y tail-y) [tail-x (dec tail-y)]
    (> head-y tail-y) [tail-x (inc tail-y)]))

(defn move-tail [[head-x head-y :as head] [tail-x tail-y :as tail]]
  (let [dx (diff head-x tail-x)
        dy (diff head-y tail-y)
        must-move? (or (> dx 1) (> dy 1))
        move-diagonal? (and (pos? dx) (pos? dy))]
    (cond
      (not must-move?) tail
      move-diagonal? (move-tail-diagonally head tail)
      :else (move-tail-along-axis head tail))))

(defn move-each-tail [head tails]
  (reduce
   (fn [moved tail] (conj moved (move-tail (last moved) tail)))
   [head]
   tails))

(defn move [[visited head & tail] direction]
  (let [head (move-head head direction)
        path (move-each-tail head tail)
        visited (conj visited (last path))]
    (cons visited path)))

(defn init-path [n]
  (cons #{[0 0]} (vec (repeat n [0 0]))))

(defn part1 [input]
  (let [init (init-path 2)]
    (-> (reduce move init input) (first) (count))))

(defn part2 [input]
  (let [init (init-path 10)]
    (-> (reduce move init input) (first) (count))))
