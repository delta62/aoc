(ns aoc.day04-test
  (:require [clojure.test :refer [deftest is testing]]
            [aoc.day04 :refer [parse part1 part2]]))

(def sample-input "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8")

(deftest day4-part1-test
  (testing "Example 1"
    (is (= 2 (part1 (parse sample-input))))))

(deftest day4-part2-test
  (testing "Example 1"
    (is (= 4 (part2 (parse sample-input))))))
