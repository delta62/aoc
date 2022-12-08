(ns aoc.day08-test
  (:require [clojure.test :refer [deftest is testing]]
            [aoc.day08 :refer [parse part1 part2]]))

(def sample-input "30373
25512
65332
33549
35390")

(deftest day8-part1-test
  (testing "Example 1"
    (is (= 21 (part1 (parse sample-input))))))

(deftest day8-part2-test
  (testing "Example 1"
    (is (= 8 (part2 (parse sample-input))))))
