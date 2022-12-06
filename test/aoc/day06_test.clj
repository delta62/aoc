(ns aoc.day06-test
  (:require [clojure.test :refer [deftest is testing]]
            [aoc.day06 :refer [part1 part2]]))

(def samples ["mjqjpqmgbljsphdztnvjfqwrcgsmlb"
              "bvwbjplbgvbhsrlpgdmjqwftvncz"
              "nppdvjthqldpwncqszvftbrmjlhg"
              "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"
              "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"])

(deftest day6-part1-test
  (testing "Example 1"
    (is (= 7 (part1 (nth samples 0)))))
  (testing "Example 2"
    (is (= 5 (part1 (nth samples 1)))))
  (testing "Example 3"
    (is (= 6 (part1 (nth samples 2)))))
  (testing "Example 4"
    (is (= 10 (part1 (nth samples 3)))))
  (testing "Example 5"
    (is (= 11 (part1 (nth samples 4))))))

(deftest day6-part2-test
  (testing "Example 1"
    (is (= 19 (part2 (nth samples 0)))))
  (testing "Example 2"
    (is (= 23 (part2 (nth samples 1)))))
  (testing "Example 3"
    (is (= 23 (part2 (nth samples 2)))))
  (testing "Example 4"
    (is (= 29 (part2 (nth samples 3)))))
  (testing "Example 5"
    (is (= 26 (part2 (nth samples 4))))))
