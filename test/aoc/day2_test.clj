(ns aoc.day2-test
  (:require [clojure.test :refer [deftest is testing]]
            [aoc.day2 :refer [part1 part2]]))

(deftest part1-tests
  (testing "example 1"
    (is (= 58 (part1 "2x3x4"))))
  (testing "example 2"
    (is(= 43 (part1 "1x1x10")))))

(deftest part2-tests
  (testing "example 1"
    (is (= 34 (part2 "2x3x4"))))
  (testing "example 2"
    (is (= 14 (part2 "1x1x10")))))
