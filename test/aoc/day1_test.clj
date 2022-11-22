(ns aoc.day1-test
  (:require [clojure.test :refer [deftest testing is]]
            [aoc.day1 :refer [part1 part2]]))

(deftest part1-tests
  (testing "zero output"
    (is (= 0 (part1 "(())")))
    (is (= 0 (part1 "()()"))))
  
  (testing "three output"
    (is (= 3 (part1 "(((")))
    (is (= 3 (part1 "(()(()(")))
    (is (= 3 (part1 "))((((("))))
  
  (testing "negative floor"
    (is (= -1 (part1 "())")))
    (is (= -1 (part1 "))("))))
  
  (testing "-3 floor"
    (is (= -3 (part1 ")))")))
    (is (= -3 (part1 ")())())")))))

(deftest part2-tests
  (testing "example 1"
    (is (= 1 (part2 ")"))))
  
  (testing "example 2"
    (is (= 5 (part2 "()())")))))
