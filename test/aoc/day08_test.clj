(ns aoc.day08-test
  (:require [clojure.test :refer [deftest is testing]]
            [clojure.string :as string]
            [aoc.day08 :refer [expanded-value line-value parse part1 part2]]))

(defn- dquote [s] (format "\"%s\"" s))

(def examples (map dquote [""
                           "abc"
                           "aaa\\\"aaa"
                           "\\x27"]))

(deftest part1-tests
  (testing "example 1"
    (is (= 2 (line-value (first (parse (first examples)))))))
  (testing "example 2"
    (is (= 2 (line-value (first (parse (second examples)))))))
  (testing "example 3"
    (is (= 3 (line-value (first (parse (nth examples 2)))))))
  (testing "example 4"
    (is (= 5 (line-value (first (parse (nth examples 3)))))))
  (testing "cumulative example"
    (is (= 12 (part1 (parse (string/join "\n" examples)))))))

(deftest part2-tests
  (testing "example 1"
    (is (= 4 (expanded-value (first (parse (first examples)))))))
  (testing "example 2"
    (is (= 4 (expanded-value (first (parse (second examples)))))))
  (testing "example 3"
    (is (= 6 (expanded-value (first (parse (nth examples 2)))))))
  (testing "example 4"
    (is (= 5 (expanded-value (first (parse (nth examples 3)))))))
  (testing "cumulative example"
    (is (= 19 (part2 (parse (string/join "\n" examples)))))))
