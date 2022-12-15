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
     :lowest-point lowest-point
     :floor (+ 2 lowest-point)}))

(defn available? [space floor [_x y :as coord]]
  (and (< y floor) (not (contains? space coord))))

(defn below [[x y]]
  [x (inc y)])

(defn below-left [[x y]]
  [(dec x) (inc y)])

(defn below-right [[x y]]
  [(inc x) (inc y)])

(defn past-lowest-point? [_space [_x y] lowest-point]
  (>= y lowest-point))

(defn sand-origin-blocked? [space _coord _lowest-point]
  (contains? space sand-origin))

(defn add-sand [state coord]
  (update state :space #(assoc % coord :sand)))

(defn drop-sand
  ([state f] (drop-sand state f sand-origin))
  ([{:keys [floor lowest-point space] :as state} f coord]
   (let [down (below coord)
         down-left (below-left coord)
         down-right (below-right coord)
         avail? (partial available? space floor)]
     (cond
       (f space coord lowest-point) state
       (avail? down) (recur state f down)
       (avail? down-left) (recur state f down-left)
       (avail? down-right) (recur state f down-right)
       :else (add-sand state coord)))))

(defn sand-until [f input]
  (->> (repeat nil)
       (reduce
        (fn [state _]
          (let [new-state (drop-sand state f)]
            (if (identical? state new-state) (reduced state) new-state)))
        input)
       (:space)
       (vals)
       (filter #{:sand})
       (count)))

(defn part1 [input]
  (sand-until past-lowest-point? input))

(defn part2 [input]
  (sand-until sand-origin-blocked? input))
