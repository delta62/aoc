(ns aoc.day06-test
  (:require [clojure.test :refer [deftest is testing]])
  (:require [aoc.day06 :refer [part1 part2 parse]]))

(deftest part-1
  (testing "example 1"
    (is (= 1000000 (part1 (parse "turn on 0,0 through 999,999")))))
  (testing "example 2"
    (is (= 1000 (part1 (parse "toggle 0,0 through 999,0")))))
  (testing "example 3"
    (is (= 0 (part1 (parse "turn off 499,499 through 500,500"))))))

(deftest part-2
  (testing "example 1"
    (is (= 1 (part2 (parse "turn on 0,0 through 0,0")))))
  (testing "example 2"
    (is (= 2000000 (part2 (parse "toggle 0,0 through 999,999"))))))
