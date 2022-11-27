(ns aoc.day04
  (:require [clojure.string :as string])
  (:import java.security.MessageDigest java.math.BigInteger))

(defn- md5 [s]
  (let [algo (MessageDigest/getInstance "MD5")
        raw (.digest algo (.getBytes s))]
    (format "%032x" (BigInteger. 1 raw))))

(defn- starts-with-zeroes [n input]
  (string/starts-with? (md5 input) (string/join (repeat n \0))))

(defn- find-idx [f coll]
  (first (keep-indexed #(if (f %2) %1 nil) coll)))

(defn part1 [input]
  (find-idx #(starts-with-zeroes 5 %1) (map #(str input %1) (range))))

(defn part2 [input]
  (find-idx #(starts-with-zeroes 6 %1) (map #(str input %1) (range))))
