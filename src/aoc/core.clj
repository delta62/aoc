(ns aoc.core
  (:gen-class)
  (:require [aoc.day1]
            [aoc.day2]
            [aoc.day3]
            [aoc.day4]
            [aoc.day5]
            [aoc.day6]
            [aoc.runner :refer [aoc-days]]))

(defn -main
  [& _args]
  (aoc-days ['aoc.day1
             'aoc.day2
             'aoc.day3
             'aoc.day4
             'aoc.day5
             'aoc.day6]))
