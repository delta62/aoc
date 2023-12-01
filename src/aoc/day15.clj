(ns aoc.day15
  (:require [aoc.util :refer [diff product sum]]
            [clojure.string :as string]))

(defn parse-line [s]
  (->> (re-seq #"-?\d+" s)
       (map parse-long)
       (partition 2)))

(defn parse [s]
  (map parse-line (string/split-lines s)))

(defn manhattan
  "Returns the Manhattan distance between two points"
  [[x1 y1] [x2 y2]]
  (+ (diff x1 x2) (diff y1 y2)))

(def beacon-x
  "Gives the x position of a beacon given an input line"
  (comp second second))

(defn impossible-spaces
  "Given a y coordinate, a point, and the closest beacon to that point, a range
   of x coordinates which could not possibly have a beacon on them"
  [y [[bx by :as beacon] point]]
  (let [radius (manhattan point beacon)
        dy (diff by y)
        xlen (- (inc (* 2 radius)) (* 2 dy))
        startx (- bx (quot xlen 2))
        endx (+ startx xlen)]
    (if (pos-int? xlen) [startx endx] nil)))

(defn beacon-x-positions [input line]
  (->> (map beacon-x input)
       (filter (partial = line))
       (set)))

(defn update-last [vec f]
  (let [last-idx (dec (count vec))
        last (last vec)]
    (assoc vec last-idx (f last))))

(defn collapse-ranges [ranges]
  (reduce (fn [state [range-start range-end :as range]]
            (let [[_prev-start prev-end] (last state)]
              (cond
                (empty? state) [range]
                (<= range-start prev-end) (update-last state (fn [last] (update-last last #(max % range-end))))
                :else (conj state range))))
          []
          ranges))

(defn range-contains? [[range-start range-end] val]
  (and (< val range-end) (>= val range-start)))

(defn split-range [[range-start range-end] val]
  [[range-start val] [(inc val) range-end]])

(defn remove-from-ranges [ranges val]
  (reduce (fn [state range]
            (if (range-contains? range val)
              (apply conj state (split-range range val))
              (conj state range))) [] ranges))

(defn remove-all-from-ranges [xs dead-range]
  (if (empty? xs)
    dead-range
    (recur (rest xs) (remove-from-ranges dead-range (first xs)))))

(defn range-count [ranges]
  (sum (map (fn [[range-start range-end]] (- range-end range-start)) ranges)))

(defn clamp-ranges [ranges start end]
  (reduce (fn [state [range-start range-end :as range]]
            (cond
              (< range-end start) state
              (> range-start end) state
              (and (< range-start start) (> range-end end)) (conj state [start end])
              (< range-start start) (conj state [start range-end])
              (> range-end end) (conj state [range-start end])
              :else (conj state range)))
          []
          ranges))

(defn all-impossible-on-line [input line]
  (let [beacon-xs (beacon-x-positions input line)]
    (->> (map (partial impossible-spaces line) input)
         (filter some?)
         (sort-by first)
         (collapse-ranges))))
        ;;  (remove-all-from-ranges beacon-xs))))

(defn find-possible-coordinate [input max-range]
  (some (fn [y]
          (when (zero? (rem y 100000)) (println "y =" y))
          (let [impossible (all-impossible-on-line input y)
                impossible (clamp-ranges impossible 0 (inc max-range))]
            (when (<= (range-count impossible) max-range)
              (let [x (if (zero? (first (first impossible))) (last (first impossible)) 0)]
                (println "x,y =" x y)
                [x y])))) (range max-range)))

(defn part1 [input]
  (range-count (all-impossible-on-line input 2000000)))

(defn part2 [input]
  (product (find-possible-coordinate input 4000000)))

(comment
  (find-possible-coordinate (parse "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3") 20)
  )