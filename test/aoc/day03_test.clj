(ns aoc.day03-test
  (:require [clojure.test :refer [deftest is testing]]
            [aoc.day03 :refer [parse part1 part2]]))

(def sample-input "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw")

(deftest day3-part1-test
  (testing "Example 1"
    (is (= 157 (part1 (parse sample-input))))))

(deftest day3-part2-test
  (testing "Example 1"
    (is (= 70 (part2 (parse sample-input))))))
