(ns aoc.day07
  (:require [clojure.string :refer [split split-lines]]))

; 16-bit binary ops - built-in bitwise ops are 32 bit
(def ^:private & bit-and)
(def ^:private | bit-or)
(defn- ! [x] (bit-and 65535 (bit-not x)))
(defn- << [x y] (bit-and 65535 (bit-shift-left x y)))
(def ^:private >> bit-shift-right)

(defn- parse-instr [s]
  (let [[input output] (split s #" -> ")]
    (condp re-matches input
      #"\d+"                :>> (fn [x]       [[:literal (parse-long x)] output])
      #"\w+"                :>> (fn [x]       [[:alias x] output])
      #"NOT (\w+)"          :>> (fn [[_ x]]   [[:not x] output])
      #"(\w+) AND (\w+)"    :>> (fn [[_ x y]] [[:and x y] output])
      #"(\w+) OR (\w+)"     :>> (fn [[_ x y]] [[:or x y] output])
      #"(\w+) LSHIFT (\d+)" :>> (fn [[_ x y]] [[:lshift x (parse-long y)] output])
      #"(\w+) RSHIFT (\d+)" :>> (fn [[_ x y]] [[:rshift x (parse-long y)] output]))))

(def solve
  (memoize (fn [key state]
             (or (parse-long key)
                 (let [[instr x y] (get state key)]
                   (case instr
                     :literal x
                     :alias  (solve x state)
                     :not    (!  (solve x state))
                     :and    (&  (solve x state) (solve y state))
                     :or     (|  (solve x state) (solve y state))
                     :lshift (<< (solve x state) y)
                     :rshift (>> (solve x state) y)))))))

(defn parse [s]
  (reduce
   (fn [state [input output]] (assoc state output input))
   {}
   (map parse-instr (split-lines s))))

#_{:clj-kondo/ignore [:clojure-lsp/unused-public-var]}
(defn part1 [instructions]
  (solve "a" instructions))

#_{:clj-kondo/ignore [:clojure-lsp/unused-public-var]}
(defn part2 [instructions]
  (let [new-b (solve "a" instructions)
        state-b (assoc instructions "b" [:literal new-b])]
    (solve "a" state-b)))
