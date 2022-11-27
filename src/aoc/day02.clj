(ns aoc.day02
  (:require [clojure.string :as string]
            [clojure.edn :as edn]))

(defn- parse-box [str]
  (map edn/read-string (string/split str #"x")))

(defn- box-surface [box]
  (let [l (first box)
        w (second box)
        h (nth box 2)]
    (+ (* 2 l w) (* 2 w h) (* 2 h l))
    ))

(defn- smallest-side [box]
  (let [l (first box)
        w (second box)
        h (nth box 2)]
    (min (* l w) (* w h) (* h l))))

(defn- ribbon-sides [box]
  (let [l (first box)
        w (second box)
        h (nth box 2)]
    (apply + (map #(* 2 %1) (take 2 (sort [l w h]))))))

(defn- box-volume [box] (apply * box))

(defn- ribbon-needed [line]
  (let [box (parse-box line)]
    (+ (ribbon-sides box) (box-volume box))))

(defn- paper-needed [line]
  (let [box (parse-box line)]
    (+ (box-surface box) (smallest-side box))))

(defn part1 [input] 
  (apply + (map paper-needed (string/split-lines input))))

(defn part2 [input]
  (apply + (map ribbon-needed (string/split-lines input))))
