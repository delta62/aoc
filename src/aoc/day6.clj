(ns aoc.day6
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
  (for [y (inclusive-range (get instr :min-y) (get instr :max-y))
        x (inclusive-range (get instr :min-x) (get instr :max-x))]
    [x y]))

(defn- toggle [state coord]
  (if (contains? state coord)
    (disj state coord)
    (conj state coord)))

(defn- run-instr [state instr]
  (let [coords (block-range instr)]
    (case (get instr :op)
      :on (into state coords)
      :off (apply disj state coords)
      (reduce toggle state coords))))

(defn part1 [instrs]
  (count (reduce run-instr #{} instrs)))
