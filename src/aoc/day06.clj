(ns aoc.day06
  (:require [clojure.string :as string])
  (:require [clojure.edn :as edn]))

(def ^:private re-line #"(on|off|toggle) (\d+),(\d+) through (\d+),(\d+)")

(defn- instruction [s]
  (let [matches (rest (re-find re-line s))]
    {:op (keyword (first matches))
     :min-x (edn/read-string (second matches))
     :min-y (edn/read-string (nth matches 2))
     :max-x (edn/read-string (nth matches 3))
     :max-y (edn/read-string (nth matches 4))}))

(defn parse [s] (map instruction (string/split-lines s)))

(defn- inclusive-range [min max] (range min (inc max)))

(defn- block-range [instr]
  (for [y (inclusive-range (:min-y instr) (:max-y instr))
        x (inclusive-range (:min-x instr) (:max-x instr))]
    [x y]))

(defn- toggle [state coord]
  (if (contains? state coord)
    (disj state coord)
    (conj state coord)))

(defn- run-instr [state instr]
  (let [coords (block-range instr)]
    (case (:op instr)
      :on (into state coords)
      :off (apply disj state coords)
      (reduce toggle state coords))))

(defn- inc-all
  ([state coords] (inc-all state coords 1))
  ([state coords n]
   (reduce (fn [state coord] (update state coord #(max 0 (+ n %1))))
           state
           coords)))

(defn- dec-all [state coords] (inc-all state coords -1))

(defn- run-brightness [state instr]
  (let [coords (block-range instr)]
    (case (:op instr)
      :on (inc-all state coords)
      :off (dec-all state coords)
      (inc-all state coords 2))))

(defn- sum [m] (apply + (vals m)))

(defn part1 [instrs]
  (count (reduce run-instr #{} instrs)))

(defn part2 [instrs]
  (let [init (reduce 
              #(assoc %1 %2 0) 
              {} 
              (block-range {:min-x 0 :max-x 999 :min-y 0 :max-y 999}))]
    (sum (reduce run-brightness init instrs))))
