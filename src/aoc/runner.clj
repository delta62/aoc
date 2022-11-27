(ns aoc.runner
  (:require [clojure.string :as string]))

(defn- day-number [day]
  (re-find #"\d+" (str day)))

(defn- input-path [day]
  (str "resources/input/" (string/replace (str day) "aoc." "") ".txt"))

(defn- load-input [day]
  (let [parser (or (ns-resolve day 'parse) identity)]
    (parser (slurp (input-path day)))))

(defn- latest-day [days]
  (last (sort-by day-number days)))

(defn aoc-days [days]
  (let [day (latest-day days)
        part1 (ns-resolve day 'part1)
        part2 (ns-resolve day 'part2)
        input (load-input day)]
    (println "Day" (day-number day))
    (if (some? part1)
      (do
        (print "Part 1: ")
        (println (time (part1 input))))
      ())
    (if (some? part2)
      (do
        (print "Part 2: ")
        (println (time (part2 input))))
      ())))
