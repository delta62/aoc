(ns aoc.day02
  (:require [aoc.util :refer [sum]]
            [clojure.string :refer [split-lines]]))

(def shapes {:rock {:beats :scissors
                    :loses-to :paper
                    :value 1}
             :paper {:beats :rock
                     :loses-to :scissors
                     :value 2}
             :scissors {:beats :paper
                        :loses-to :rock
                        :value 3}})

(defn- parse-pair [mine s]
  (case s
    "A" :rock
    "B" :paper
    "C" :scissors
    (mine s)))

(defn- parse-mine [mine]
  (case mine
    "X" :rock
    "Y" :paper
    "Z" :scissors))

(defn- parse-outcome [mine]
  (case mine
    "X" :loss
    "Y" :tie
    "Z" :win))

(defn- parse-line [line f]
  (->> line
       (re-matches #"(.) (.)")
       (rest)
       (map #(parse-pair f %))))

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

(defn parse-part1 [s]
  (map #(parse-line % parse-mine) (split-lines s)))

(defn parse-part2 [s]
  (map #(parse-line % parse-outcome) (split-lines s)))

(defn part1 [input]
  (sum (map score-game input)))

(defn part2 [input]
  (sum (map score-needed input)))
