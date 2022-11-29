(ns aoc.runner
  (:require [clj-http.client :as client]
            [clojure.java.io :as io]
            [clojure.string :as string]
            [clojure.edn :as edn]
            [clojure.tools.namespace.find :as nsf]
            [clojure.java.classpath :refer [classpath]]
            [clojure.term.colors :as color])
  (:import [java.io FileNotFoundException]))

(defn- load-namespaces []
  (let [namespaces (filter #(re-matches #"aoc.day\d{2}" (str %1)) (nsf/find-namespaces (classpath)))]
    (doseq [namespace namespaces]
      (require namespace))
    namespaces))

(defn- load-namespace [day]
  (let [is-day #(= (format "aoc.day%02d" day) (str %1))]
    (some #(when (is-day %1) %1) (load-namespaces))))

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
  (with-open [writer (io/writer (input-path day))]
    (.write writer input)))

(defn- have-input? [day]
  (.exists (io/file (input-path day))))

(defn- load-input [day mod]
  (let [parser (ns-resolve mod 'parse)]
    (if (some? parser)
      (parser (slurp (input-path day)))
      (slurp (input-path day)))))

(defn- load-or-download-input [year day mod]
  (when (not (have-input? day))
    (write-input day (download-input year day (load-session))))
  (load-input day mod))

(defn- latest-day [days]
  (last (sort-by day-number days)))

(defn time-ms [f & args]
  (let [start-time (. System (currentTimeMillis))
        result (apply f args)
        end-time (. System (currentTimeMillis))]
    [(- end-time start-time) result]))

(defn run-day [year day]
  (let [namespace (if (int? day) (load-namespace day) day)
        part1 (ns-resolve namespace 'part1)
        part2 (ns-resolve namespace 'part2)
        day-num (day-number day)
        [input-time input] (time-ms load-or-download-input year day-num namespace)]
    (when (some? part1)
      (let [[solve-time solution] (time-ms part1 input)]
        (println (format "Day %2d - Part 1:   " day-num) (color/green (str solution)))
        (println (str "         generator: " (color/blue (format "%dms" input-time))))
        (println (str "         solution:  " (color/blue (format "%dms" solve-time))))
        (println)))
    (when (some? part2)
      (let [[solve-time solution] (time-ms part2 input)]
        (println (format "Day %2d - Part 2:   " day-num) (color/green (str solution)))
        (println (str "         generator: " (color/blue (format "%dms" input-time))))
        (println (str "         solution:  " (color/blue (format "%dms" solve-time))))
        (println)))))

(defn run-latest [year]
  (run-day year (latest-day (load-namespaces))))

(defn run-all [year]
   (doseq [namespace (load-namespaces)]
     (run-day year namespace)))
