(ns aoc.day02
  (:require [clojure.string :refer [lower-case split-lines]]
            [aoc.util :refer [sum]]))

(def shapes {:rock {:beats :scissors
                    :loses-to :paper
                    :value 1}
             :paper {:beats :rock
                     :loses-to :scissors
                     :value 2}
             :scissors {:beats :paper
                        :loses-to :rock
                        :value 3}})

(defn- parse-theirs [s]
  (case s
    "A" :rock
    "B" :paper
    "C" :scissors
    (keyword (lower-case s))))

(defn- parse-mine [[theirs mine]]
  [theirs (case mine
            :x :rock
            :y :paper
            :z :scissors)])

(defn- parse-outcome [[theirs mine]]
  [theirs (case mine
            :x :loss
            :y :tie
            :z :win)])

(defn- parse-line [line]
  (->> line
       (re-matches #"(.) (.)")
       (rest)
       (map parse-theirs)))

(defn- is-tie? [game]
  (apply = game))

(defn- is-win? [[theirs mine]]
  (= theirs (-> shapes (get mine) :beats)))

(defn- score-outcome [game]
  (cond
    (is-win? game) 6
    (is-tie? game) 3
    :else 0))

(defn- score-game [[_ mine :as game]]
  (+
   (-> shapes (get mine) :value)
   (score-outcome game)))

(defn- find-needed [[theirs needed-outcome]]
  (case needed-outcome
    :loss (-> shapes (get theirs) :beats)
    :win (-> shapes (get theirs) :loses-to)
    theirs))

(defn- score-needed [[theirs :as game]]
  (let [needed-shape (find-needed game)]
    (score-game [theirs needed-shape])))

(defn parse [s]
  (map parse-line (split-lines s)))

(defn part1 [input]
  (->> input
       (map parse-mine)
       (map score-game)
       (sum)))

(defn part2 [input]
  (->> input
       (map parse-outcome)
       (map score-needed)
       (sum)))
