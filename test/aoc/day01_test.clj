(ns aoc.day01-test
  (:require [clojure.test :refer [deftest is]]
            [aoc.day01 :refer [parse part1 part2]]))

(def sample-input "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000")

(deftest day1-part1-test
  (is (= 24000 (part1 (parse sample-input)))))

(deftest day1-part2-test
  (is (= 45000 (part2 (parse sample-input)))))
