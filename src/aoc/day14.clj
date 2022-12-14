(ns aoc.day14 
  (:require [clojure.string :as string]))

(def sand-origin [500 0])

(defn points-along [[x1 y1] [x2 y2]]
  (cond
    (< x1 x2) (map vector (range x1 (inc x2)) (repeat y1))
    (> x1 x2) (map vector (range x2 (inc x1)) (repeat y1))
    (< y1 y2) (map vector (repeat x1) (range y1 (inc y2)))
    (> y1 y2) (map vector (repeat x1) (range y2 (inc y1)))))

(defn add-rocks [space [p1 p2]]
  (let [p1 (map parse-long p1)
        p2 (map parse-long p2)
        points (points-along p1 p2)
        key-value-pairs (map #(vector %1 :rock) points)]
    (into space key-value-pairs)))

(defn draw-line [space s]
  (->> (string/split s #" -> ")
       (map #(string/split % #","))
       (partition 2 1)
       (reduce add-rocks space)))

(defn parse [s]
  (let [space (reduce draw-line {} (string/split-lines s))
        lowest-point (apply max (map last (keys space)))]
    {:space space
     :lowest-point lowest-point}))

(defn available? [space coord]
  (not (contains? space coord)))

(defn below [[x y]]
  [x (inc y)])

(defn below-left [[x y]]
  [(dec x) (inc y)])

(defn below-right [[x y]]
  [(inc x) (inc y)])

(defn add-sand [state coord]
  (update state :space #(assoc % coord :sand)))

(defn drop-sand
  ([state] (drop-sand state sand-origin))
  ([{:keys [space lowest-point] :as state} [_ y :as coord]]
    (let [down (below coord)
          down-left (below-left coord)
          down-right (below-right coord)]
      (cond
        (>= y lowest-point) state
        (available? space down) (recur state down)
        (available? space down-left) (recur state down-left)
        (available? space down-right) (recur state down-right)
        :else (add-sand state coord))))
  )

(defn part1 [input]
  (->> (repeat nil)
       (reduce
        (fn [state _]
          (let [new-state (drop-sand state)]
            (if (identical? state new-state) (reduced state) new-state)))
        input)
       (:space)
       (vals)
       (filter #{:sand})
       (count)))
  