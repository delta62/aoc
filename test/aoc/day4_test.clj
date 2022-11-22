(ns aoc.day4-test
  (:require [clojure.test :refer [deftest is testing]]
            [aoc.day4 :refer [part1]]))

(deftest part1-tests
  (testing "example 1"
    (is (= 609043 (part1 "abcdef"))))
  (testing "example 2"
    (is (= 1048970 (part1 "pqrstuv")))))
