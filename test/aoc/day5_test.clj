(ns aoc.day5-test
  (:require [clojure.test :refer [deftest is testing]]
            [aoc.day5 :refer [part1]]))

(deftest part1-tests
  (testing "example 1"
    (is (= 1 (part1 "ugknbfddgicrmopn"))))
  (testing "example 2"
    (is (= 1 (part1 "aaa"))))
  (testing "example 3"
    (is (= 0 (part1 "jchzalrnumimnmhp"))))
  (testing "example 4"
    (is (= 0 (part1 "haegwjzuvuyypxyu"))))
  (testing "example 5"
    (is (= 0 (part1 "dvszwmarrgswjxmb")))))
