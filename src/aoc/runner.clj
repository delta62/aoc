(ns aoc.runner
  (:require [clojure.string :as string]))

(defn- day-number [day]
  (re-find #"\d+" (str day)))

(defn- input-path [day]
  (str "resources/input/" (string/replace (str day) "aoc." "") ".txt"))

(defn- load-input [day]
  (slurp (input-path day)))

(defn- latest-day [days]
  (last (sort-by day-number days)))

(defn aoc-days [days]
  (let [day (latest-day days)
        input (load-input day)
        part1 (ns-resolve day 'part1)
        part2 (ns-resolve day 'part2)]
    (println "Day" (day-number day))
    (if (some? part1)
      (do
        (print "Part 1: ")
        (println (part1 input)))
      ())
    (if (some? part2)
      (do
        (print "Part 2: ")
        (println (part2 input)))
      ())))
