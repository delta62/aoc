(ns aoc.day14-test
  (:require [aoc.day14 :refer [parse part1 part2]]
            [clojure.test :refer [deftest is testing]]))

(def sample-input "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9")

(deftest day14-part1-test
  (testing "Example 1"
    (is (= 24 (part1 (parse sample-input))))))

(deftest day14-part2-test
  (testing "Example 1"
    (is (= 93 (part2 (parse sample-input))))))
