(ns aoc.day05-test
  (:require [clojure.test :refer [deftest is testing]]
            [aoc.day05 :refer [parse part1 part2]]))

(def sample-input "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2")

(deftest day5-part1-test
  (testing "Example 1"
    (is (= "CMZ" (part1 (parse sample-input))))))

(deftest day5-part2-test
  (testing "Example 1"
    (is (= "MCD" (part2 (parse sample-input))))))
