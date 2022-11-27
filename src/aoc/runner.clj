(ns aoc.runner
  (:require [clj-http.client :as client]
            [clojure.java.io :as io :refer [writer]]
            [clojure.string :as string]
            [clojure.edn :as edn])
  (:import [java.io FileNotFoundException]))

(defn- day-number [day]
  (edn/read-string (re-find #"[1-9]\d*" (str day))))

(defn- try-slurp [path]
  (try
    (slurp path)
    (catch FileNotFoundException _ nil)))

(defn- load-session [] (string/trim (try-slurp ".aoc-session")))

(defn- download-input [year day session]
  (try
    (get (client/get
          (str "https://adventofcode.com/" year "/day/" day "/input")
          {:cookies {"session" {:value session}}})
         :body)
    (catch Exception _ nil)))

(defn- input-path [day]
  (str "resources/input/day" (format "%02d" day) ".txt"))

(defn- write-input [day input]
  (with-open [writer (writer (input-path day))]
    (.write writer input)))

(defn- have-input? [day]
  (.exists (io/file (input-path day))))

(defn- load-input [day mod]
  (let [parser (ns-resolve mod 'parse)]
    (if (some? parser)
      (do
        (print "Parsing: ")
        (time (parser (slurp (input-path day)))))
      (slurp (input-path day)))))

(defn- load-or-download-input [year day mod]
  (when (not (have-input? day))
    (write-input day (download-input year day (load-session))))
  (load-input day mod))

(defn- latest-day [days]
  (last (sort-by day-number days)))

(defn run-day [year day]
  (println "Day" (day-number day))
  (let [part1 (ns-resolve day 'part1)
        part2 (ns-resolve day 'part2)
        day-num (day-number day)
        input (load-or-download-input year day-num day)]
    (when (some? part1)
      (print "Part 1: ")
      (println (time (part1 input))))
    (when (some? part2)
      (print "Part 2: ")
      (println (time (part2 input))))))

(defn run-latest [year days]
  (run-day year (latest-day days)))
