(ns aoc.day07-test
  (:require [clojure.test :refer [deftest is testing]]
            [aoc.day07 :refer [parse part1 part2]]))

(def sample-input "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k")

(deftest day7-part1-test
  (testing "Example 1"
    (is (= 95437 (part1 (parse sample-input))))))

(deftest day7-part1-test
  (testing "Example 1"
    (is (= 24933642 (part2 (parse sample-input))))))
