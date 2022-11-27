(ns aoc.day05-test
  (:require [clojure.test :refer [deftest is testing]]
            [aoc.day05 :refer [part1 part2]]))

(deftest part1-tests
  (testing "example 1"
    (is (= 1 (part1 "ugknbfddgicrmopn"))))
  (testing "example 2"
    (is (= 1 (part1 "aaa"))))
  (testing "example 3"
    (is (zero? (part1 "jchzalrnumimnmhp"))))
  (testing "example 4"
    (is (zero? (part1 "haegwjzuvuyypxyu"))))
  (testing "example 5"
    (is (zero? (part1 "dvszwmarrgswjxmb")))))

(deftest part2-tests
  (testing "example 1"
    (is (= 1 (part2 "qjhvhtzxzqqjkmpb"))))
  (testing "example 2"
    (is (= 1 (part2 "xxyxx"))))
  (testing "example 3"
    (is (zero? (part2 "uurcxstgmygtbstg"))))
  (testing "example 4"
    (is (zero? (part2 "ieodomkazucvgmuy")))))
