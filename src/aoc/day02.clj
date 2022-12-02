(ns aoc.day02
  (:require [clojure.string :as string]
            [aoc.util :refer [sum]]))

(defn- parse-theirs [s]
  (case s
    "A" :rock
    "B" :paper
    "C" :scissors
    (keyword (string/lower-case s))))

(defn- parse-mine [[theirs mine]]
  (case mine
    :x [theirs :rock]
    :y [theirs :paper]
    :z [theirs :scissors]))

(defn- parse-outcome [[theirs mine]]
  (case mine
    :x [theirs :loss]
    :y [theirs :tie]
    :z [theirs :win]))

(defn- parse-line [line]
  (->> line
       (re-matches #"(.) (.)")
       (rest)
       (map parse-theirs)))

(defn- score-shape [shape]
  (case shape
    :rock 1
    :paper 2
    :scissors 3))

(defn- is-tie? [[theirs mine]]
  (= theirs mine))

(defn- is-win? [game]
  (case game
    ([:paper :scissors] [:rock :paper] [:scissors :rock]) true
    false))

(defn- outcome [game]
  (cond
    (is-tie? game) :tie
    (is-win? game) :win
    :else :loss))

(defn- score-outcome [game]
  (case (outcome game)
    :win 6
    :tie 3
    0))

(defn- score-game [[_ mine :as game]]
  (+ (score-shape mine) (score-outcome game)))

(defn- find-needed [[theirs needed-outcome]]
  (case [needed-outcome theirs]
    [:loss :rock]     :scissors
    [:loss :paper]    :rock
    [:loss :scissors] :paper
    [:win  :rock]     :paper
    [:win  :paper]    :scissors
    [:win  :scissors] :rock
    theirs))

(defn- score-needed [[theirs :as game]]
  (let [needed-shape (find-needed game)]
    (score-game [theirs needed-shape])))

(defn parse [s]
  (map parse-line (string/split-lines s)))

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
