(ns aoc.runner
  (:require [clj-http.client :as client]
            [clojure.java.io :as io]
            [clojure.string :refer [trim]]
            [clojure.tools.namespace.find :refer [find-namespaces]]
            [clojure.java.classpath :refer [classpath]]
            [clojure.term.colors :as color])
  (:import [java.io FileNotFoundException]))

(defn time-ms [f & args]
  (let [start-time (. System (currentTimeMillis))
        result (apply f args)
        end-time (. System (currentTimeMillis))]
    [(- end-time start-time) result]))

(defn- load-namespaces []
  (let [namespaces (filter #(re-matches #"aoc.day\d{2}" (str %1)) (find-namespaces (classpath)))]
    (doseq [namespace namespaces] (require namespace))
    namespaces))

(defn- load-namespace [day]
  (let [is-day #(= (format "aoc.day%02d" day) (str %1))]
    (some #(when (is-day %1) %1) (load-namespaces))))

(defn- day-number [day]
  (parse-long (re-find #"[1-9]\d*" (str day))))

(defn- load-session []
  (trim (try (slurp ".aoc-session")
             (catch FileNotFoundException _ (throw (Exception. "Couldn't find session file. Copy your session cookie into a file named .aoc-session."))))))

(defn- download-input [year day session]
  (try
    (get (client/get
          (str "https://adventofcode.com/" year "/day/" day "/input")
          {:cookies {"session" {:value session}}})
         :body)
    (catch Exception _
      (throw (Exception. "Couldn't download input. Is your session token in .aoc-session valid?")))))

(defn- input-path [day]
  (format "resources/input/day%02d.txt" day))

(defn- write-input [day input]
  (spit (input-path day) input))

(defn- have-input? [day]
  (.exists (io/file (input-path day))))

(defn- load-input [day mod]
  (let [parser (ns-resolve mod 'parse)
        input (slurp (input-path day))]
    (if (some? parser)
      (time-ms parser input)
      [nil input])))

(defn- load-or-download-input [year day mod]
  (when (not (have-input? day))
    (write-input day (download-input year day (load-session))))
  (load-input day mod))

(defn- latest-day [days]
  (last (sort-by day-number days)))

(defn- fmt-ms [ms]
  (if (nil? ms)
    "n/a"
    (color/blue (format "%dms" ms))))

(defn run-day [year day]
  (let [namespace (if (int? day) (load-namespace day) day)
        part1 (ns-resolve namespace 'part1)
        part2 (ns-resolve namespace 'part2)
        day-num (day-number day)]
    (when (some? part1)
      (let [[input-time input] (load-or-download-input year day-num namespace)
            [solve-time solution] (time-ms part1 input)]
        (println (format "Day %2d - Part 1:   " day-num) (color/green (str solution)))
        (println (str "         generator: " (fmt-ms input-time)))
        (println (str "         solution:  " (fmt-ms solve-time)))
        (println)))
    (when (some? part2)
      (let [[input-time input] (load-or-download-input year day-num namespace)
            [solve-time solution] (time-ms part2 input)]
        (println (format "Day %2d - Part 2:   " day-num) (color/green (str solution)))
        (println (str "         generator: " (fmt-ms input-time)))
        (println (str "         solution:  " (fmt-ms solve-time)))
        (println)))))

(defn run-latest [year]
  (run-day year (latest-day (load-namespaces))))

(defn run-all [year]
   (doseq [namespace (load-namespaces)]
     (run-day year namespace)))
