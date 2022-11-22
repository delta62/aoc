(ns aoc.day4
  (:require [clojure.string :as string]))

(import 'java.security.MessageDigest
        'java.math.BigInteger)

(defn- md5
  "Calculate the md5 hash of some string
   From https://gist.github.com/jizhang/4325757"
  [input]
  (let [algo (MessageDigest/getInstance "MD5")
        raw (.digest algo (.getBytes input))]
   (format "%032x" (BigInteger. 1 raw))))

(defn- starts-with-five-zeroes [input]
  (string/starts-with? (md5 input) "00000"))

(defn- starts-with-six-zeroes [input]
  (string/starts-with? (md5 input) "000000"))

(defn- find-idx [f coll]
  (first (keep-indexed #(if (f %2) %1) coll)))

(defn part1 [input]
  (find-idx starts-with-five-zeroes (map #(str input %1) (range))))

(defn part2 [input]
  (find-idx starts-with-six-zeroes (map #(str input %1) (range))))
