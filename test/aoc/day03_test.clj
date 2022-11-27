(ns aoc.day03-test
  (:require [clojure.test :refer [deftest is testing]]
            [aoc.day03 :refer [part1 part2]]))

(deftest part1-tests
  (testing "example 1"
    (is (= 2 (part1 ">")))
    (is (= 4 (part1 "^>v<")))
    (is (= 2 (part1 "^v^v^v^v^v")))))

(deftest part2-tests
  (is (= 3 (part2 "^v")))
  (is (= 3 (part2 "^>v<")))
  (is (= 11 (part2 "^v^v^v^v^v"))))
