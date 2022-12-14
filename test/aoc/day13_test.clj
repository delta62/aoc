(ns aoc.day13-test
  (:require [aoc.day13 :refer [parse-part1 parse-part2 part1 part2]]
            [clojure.test :refer [deftest is testing]]))

(def sample-input "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]")

(deftest day13-part1-test
  (testing "Example 1"
    (is (= 13 (part1 (parse-part1 sample-input))))))

(deftest day13-part2-test
  (testing "Example 1"
    (is (= 140 (part2 (parse-part2 sample-input))))))
