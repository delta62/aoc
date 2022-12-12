(ns aoc.day12-test
  (:require [aoc.day12 :refer [parse part1 part2]]
            [clojure.test :refer [deftest is testing]]))

(def sample-input "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi")

(deftest day12-part1-test
  (testing "Example 1"
    (is (= 31 (part1 (parse sample-input))))))

(deftest day12-part2-test
  (testing "Example 1"
    (is (= 29 (part2 (parse sample-input))))))
