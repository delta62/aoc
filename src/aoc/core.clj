(ns aoc.core
  (:gen-class)
  (:require [aoc.day1]
            [aoc.day2]
            [aoc.day3]
            [aoc.day4]
            [aoc.day5]
            [aoc.day6]
            [aoc.runner :refer [run-latest]]))

(defn -main
  [& _args]
  (run-latest 2015 ['aoc.day1
             'aoc.day2
             'aoc.day3
             'aoc.day4
             'aoc.day5
             'aoc.day6]))
