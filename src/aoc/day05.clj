(ns aoc.day05
  (:require [clojure.string :refer [join split split-lines]]
            [aoc.util :refer [upper-case-ascii-char]]))

(defn parse-init-line [line]
  (->> line
       (partition 3 4)
       (map #(some upper-case-ascii-char %))))

(defn parse-init
  "Parses an ASCII block diagram into a vector of collections. The first item
   in each collection corresponds to the top block in the diagram, and the last
   to the bottom. Note that diagrams are 1-indexed whereas this generates a
   0-indexed vector."
  [init]
  (->> init
       (split-lines)
       (map parse-init-line)
       (drop-last)
       (apply map vector)
       (map #(filter some? %))
       (vec)))

(defn parse-move-line
  "Parse a human-readable move instruction to a map with :count and 0-indexed
   :from and :two values"
  [line]
  (let [[count from to] (map parse-long (rest (split line #"\D+")))]
    {:count count :from (dec from) :to (dec to)}))

(defn parse-moves [moves]
  (map parse-move-line (split-lines moves)))

(defn parse[s]
  (map #(%1 %2) [parse-init parse-moves] (split s #"\n\n")))

(defn move
  "Repeatedly move `count` items from `from` to `to` one by one"
  [state {:keys [count from to]}]
  (if (zero? count)
    state
    (let [[item & newfrom] (nth state from)
          newfrom (or newfrom [])
          newto (cons item (nth state to))
          state (assoc state from newfrom)
          state (assoc state to newto)]
      (recur state {:count (dec count) :from from :to to}))))

(defn move-stack
  "Move `count` items from `from` to `to` in a single pass (order is
   maintained)"
  [state {:keys [count from to]}]
  (let [[moving newfrom] (split-at count (nth state from))
        newto (concat moving (nth state to))
        state (assoc state from newfrom)]
    (assoc state to newto)))

(defn part1 [[init moves]]
  (join (map first (reduce move init moves))))

(defn part2 [[init moves]]
  (join (map first (reduce move-stack init moves))))
