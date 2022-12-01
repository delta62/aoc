(ns aoc.day09-test
  (:require [clojure.test :refer [deftest is testing]]
            [aoc.day09 :refer [parse part1]]))

(def example-input "London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141")

(deftest part1-test
  (testing "Example 1"
    (is (= 605 (part1 (parse example-input))))))
