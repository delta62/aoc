(ns aoc.day11
  (:require [aoc.util :refer [product]]
            [clojure.string :as string]))

(defn discard-label [s]
  (last (string/split s #": " 2)))

(defn parse-items [s]
  (vec (map parse-long (string/split (discard-label s) #", "))))

(defn create-op-fn [f amount-str]
  (let [old-str? (= "old" amount-str)]
    (cond
      (and old-str? (= +' f)) #(*' 2 %)
      (and old-str? (= *' f)) #(*' % %)
      :else #(f (parse-long amount-str) %))))

(defn parse-operation [s]
  (let [[_ op amount] (re-find #"([+*]) (\d+|old)" s)]
    (case op
      "+" (create-op-fn +' amount)
      "*" (create-op-fn *' amount))))

(defn divisible-by? [num denom]
  (= 0 (rem num denom)))

(defn parse-test [s]
  #(divisible-by? % (parse-long (re-find #"\d+" s))))

(defn parse-tests [s]
  (map parse-long (map #(re-find #"\d+" %) s)))

(defn parse-monkey [s]
  (let [[_ starting op test & tests] (string/split-lines s)
        items (parse-items starting)
        op (parse-operation op)
        test (parse-test test)
        tests (parse-tests tests)]
    {:items items
     :op op
     :inspect-count 0
     :test test
     :tests tests}))

(defn process-item [state monkey item index divide?]
  (let [worry-level ((:op monkey) item)
        new-worry-level (if divide? (quot worry-level 3) worry-level)
        test-ok? ((:test monkey) new-worry-level)
        destination (if test-ok? (first (:tests monkey)) (second (:tests monkey)))]
    (-> state
        (update-in [destination :items] #(conj % new-worry-level))
        (update-in [index :inspect-count] inc))))

(defn turn [divide? state index]
  (let [monkey (get state index)]
    (update-in 
     (reduce #(process-item %1 monkey %2 index divide?) state (:items monkey))
     [index :items]
     (constantly []))))

(defn round [monkeys divide?]
  (reduce (partial turn divide?) monkeys (keys monkeys)))

(defn parse [s]
  (->> (string/split s #"\n\n")
       (map parse-monkey)
       (map-indexed vector)
       (reduce (fn [state [i x]] (assoc state i x)) {})))

(defn part1 [input]
  (->> (range 20)
       (reduce (fn [state _] (round state true)) input)
       (vals)
       (map :inspect-count)
       (sort >)
       (take 2)
       (product)))

(defn part2 [input]
  (->> (range 10000)
       (reduce (fn [state _] (round state false)) input)
       (vals)
       (map :inspect-count)
       (sort >)
       (take 2)
       (product)))
