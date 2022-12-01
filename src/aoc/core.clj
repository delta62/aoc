(ns aoc.core
  (:gen-class)
  (:require [clojure.tools.cli :refer [parse-opts]]
            [aoc-runner.core :refer [run-all run-day run-latest]]))

(def cli-options
  [["-d" "--day DAY" "Run a specific day"
    :parse-fn #(Integer/parseInt %)
    :validate [#(and (pos-int? %1) (<= %1 25))]]
   ["-a" "--all" "Run all days"]
   ["-y" "--year YEAR" "Run a specific year"
    :parse-fn #(Integer/parseInt %)
    :default 2015]
   ["-h" "--help" "Show help"]])

(defn -main
  [& args]
  (let [{:keys [options errors summary]} (parse-opts args cli-options)
        day (:day options)
        year (:year options)]
    (println "AOC" year)
    (cond
      (some? errors) (println (first errors))
      (:help options) (println summary)
      (:all options) (run-all year)
      (some? day) (run-day year day)
      :else (run-latest year))))
