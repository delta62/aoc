(ns aoc.day10
  (:require [aoc.util :refer [sum]]
            [clojure.string :refer [split split-lines]]))

(def initial-state [{:tick 1 :x 1}])

(def cycles [20 60 100 140 180 220])

(def screen-width 40)

(defn parse-line [line]
  (let [[op n] (split line #" ")]
    (case op
      "noop" [{:op :noop}]
      "addx" [{:op :noop}, {:op :addx, :val (parse-long n)}])))

(defn parse [s]
  (mapcat parse-line (split-lines s)))

(defn tick [past-states {:keys [op val]}]
  (let [{:keys [tick x]} (last past-states)]
    (conj past-states
          (case op
            :noop {:tick (inc tick) :x x}
            :addx {:tick (inc tick) :x (+ x val)}))))

(defn sprite-pixel [cpu gpu]
  (let [x (:x cpu)
        in-range? (and (>= x (dec gpu)) (<= x (inc gpu)))]
    (if in-range? \# \.)))

(defn part1 [input]
  (->> (reduce tick initial-state input)
       (filter (fn [x] (some #(= (:tick x) %) cycles)))
       (map #(* (:tick %) (:x %)))
       (sum)))

(defn part2 [input]
  (let [cpu (reduce tick initial-state input)
        gpu (cycle (range screen-width))
        pixels (map sprite-pixel cpu gpu)]
    (->> (interleave (partition screen-width pixels) (repeat \newline))
         (flatten)
         (apply str))))
