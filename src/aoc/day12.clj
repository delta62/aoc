(ns aoc.day12
  (:require [aoc.util :refer [lowercase-ascii-to-decimal key-of map-values]]
            [clojure.string :as string]))

(defn point-to-str
  "Serialize a point such as [3 2] to a string such as \"3:2\""
  [[x y]]
  (str x \: y))

(defn str-to-point
  "Deserialize a point such as \"3:2\" to a point such as [3 2]"
  [s]
  (->> (re-find #"(\d+):(\d+)" s)
       (rest)
       (map parse-long)
       (vec)))

(defn string-to-map
  "Parse a string of characters into a hash with keys set to serialized points
   and values of the letters in the input string"
  [s]
  (let [lines (string/split-lines s)
        rows (count lines)
        cols (count (first lines))
        coords (for [y (range rows), x (range cols)] [x y])]
    (->> (string/replace s #"\n" "")
         (map vector coords)
         (reduce (fn [state [p c]] (assoc state (point-to-str p) c)) {}))))

(defn parse [s]
  (let [m (string-to-map s)
        start (key-of m #(= \S %))
        end (key-of m #(= \E %))
        map (-> m
                (assoc start \a)
                (assoc end \z)
                (map-values lowercase-ascii-to-decimal))]
    {:start start, :end end, :map map}))

(defn coords-from
  "Returns possible coordinates from a given point by moving left, right, up,
   and down. No bounds checking is performed."
  [[x y]]
  [[(dec x) y]
   [(inc x) y]
   [x (inc y)]
   [x (dec y)]])

(defn find-coord
  "Find a point coordinate in a given map, else `fallback`"
  ([m point] (find-coord m point nil))
  ([m point fallback] (get m (point-to-str point) fallback)))

(defn can-visit?
  "Returns `true` if it is possible to visit `to` via `from`. Only the shortest
   path to `to` is considered; all others will return `false`."
  [m paths from to num-steps]
  (let [from-height (find-coord m from)
        to-height (find-coord m to)
        shortest-so-far (find-coord paths from ##Inf)]
    (and (some? from-height)
         (< num-steps shortest-so-far)
         (>= (inc from-height) to-height))))

(defn find-paths
  "Builds a map of distances to `to` from every location in `m`. Starting points
   that are unreachable are omitted from the map. O(n)."
  ([to m] (find-paths to m 0 {}))
  ([to m n paths]
   (reduce (fn [paths coord]
             (if (can-visit? m paths coord to (inc n))
               (find-paths coord m (inc n) paths)
               paths))
           (assoc paths (point-to-str to) n)
           (coords-from to))))

(defn part1 [{:keys [start end map]}]
  (-> (str-to-point end)
      (find-paths map)
      (get start)))

(defn part2 [input]
  (let [paths (find-paths (str-to-point (:end input)) (:map input))]
    (->> (:map input)
         (filter (fn [[_ v]] (= (lowercase-ascii-to-decimal \a) v)))
         (keys)
         (map #(get paths % ##Inf))
         (apply min))))
