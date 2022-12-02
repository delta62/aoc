(ns aoc.day02-test
  (:require [clojure.test :refer [deftest is testing]]
            [aoc.day02 :refer [parse part1 part2]]))

(def sample-input "A Y
B X
C Z")

(deftest day2-part1-test
  (testing "Example 1"
    (is (= 15 (part1 (parse sample-input))))))

(deftest day2-part2-test
  (testing "Example 1"
    (is (= 12 (part2 (parse sample-input))))))
