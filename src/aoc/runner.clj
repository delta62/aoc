(ns aoc.runner
  (:require [clj-http.client :as client]
            [clojure.java.io :as io]
            [clojure.string :as string]
            [clojure.edn :as edn]
            [clojure.tools.namespace.find :as nsf]
            [clojure.java.classpath :refer [classpath]])
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
  (let [namespace (if (int? day) (load-namespace day) day)
        part1 (ns-resolve namespace 'part1)
        part2 (ns-resolve namespace 'part2)
        day-num (day-number day)
        input (load-or-download-input year day-num namespace)]
    (println (str "Day " day-num))
    (when (some? part1)
      (print "Part 1: ")
      (println (time (part1 input))))
    (when (some? part2)
      (print "Part 2: ")
      (println (time (part2 input))))))

(defn run-latest [year]
  (run-day year (latest-day (load-namespaces))))

(defn run-all [year]
   (doseq [namespace (load-namespaces)]
     (run-day year namespace)))
