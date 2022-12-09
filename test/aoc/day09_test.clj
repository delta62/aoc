(ns aoc.day09-test
  (:require [clojure.test :refer [deftest is testing]]
            [aoc.day09 :refer [parse part1 part2]]))

(def sample-input1 "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2")

(def sample-input2 "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20")

(deftest day9-part1-test
  (testing "Example 1"
    (is (= 13 (part1 (parse sample-input1))))))

(deftest day9-part2-test
  (testing "Example 1"
    (is (= 1 (part2 (parse sample-input1)))))
  (testing "Example 2"
    (is (= 36 (part2 (parse sample-input2))))))
