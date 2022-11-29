(ns aoc.day07-test
  (:require [clojure.test :refer [deftest is testing]]
            [aoc.day07 :refer [parse solve]]))

(def input "123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i")

(deftest part-1
  (testing "no it doesnt"
    (let [parsed (parse input)]
      (is (= 72 (solve "d" parsed)))
      (is (= 507 (solve "e" parsed)))
      (is (= 492 (solve "f" parsed)))
      (is (= 114 (solve "g" parsed)))
      (is (= 65412 (solve "h" parsed)))
      (is (= 65079 (solve "i" parsed)))
      (is (= 123 (solve "x" parsed)))
      (is (= 456 (solve "y" parsed))))))
