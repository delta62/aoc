(ns aoc.core
  (:gen-class)
  (:require [aoc.day01]
            [aoc.day02]
            [aoc.day03]
            [aoc.day04]
            [aoc.day05]
            [aoc.day06]
            [aoc.runner :refer [run-latest]]))

(defn -main
  [& _args]
  (run-latest 2015 ['aoc.day01
             'aoc.day02
             'aoc.day03
             'aoc.day04
             'aoc.day05
             'aoc.day06]))
