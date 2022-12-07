(ns aoc.day07
  (:require [clojure.string :as string]
            [aoc.util :refer [sum]]))

(def total-space 70000000)
(def needed-space 30000000)

(defn parse-cd [path] {:command :cd, :path path})

(defn parse-ls-size [size]
  (if (= "dir" size) :dir (parse-long size)))

(defn parse-entry [state entry]
  (let [[size name] (string/split entry #"\s+")
        size (parse-ls-size size)]
    (assoc state name size)))

(defn parse-ls [output]
  (let [entries (string/split-lines output)
        files (reduce parse-entry {} entries)]
    {:command :ls, :entries files}))

(defn parse-cmd [line]
  (let [[cmd output] (string/split line #"\s+" 2)]
    (case cmd
      "cd" (parse-cd output)
      "ls" (parse-ls output))))

(defn parse [s]
  (->> (string/split s #"\$")
       (rest)
       (map string/trim)
       (map parse-cmd)))

(defn update-path [path new-path]
  (case new-path
    "/" "/"
    ".." (string/replace path #"\/[^\/]+\/[^\/]*$" "/")
    (str (string/join "" [path new-path]) "/")))

(defn path-sequence [path]
  (->> (string/split path #"\/")
       (rest)
       (cons "/")))

(defn update-tree [tree path entries]
  (update-in tree (path-sequence path) (constantly entries)))

(defn build-tree [[path tree] command]
  (case (:command command)
    :cd (let [new-path (update-path path (:path command))]
          [new-path tree])
    :ls [path (update-tree tree path (:entries command))]))

(defn get-size [tree]
  (if (number? tree) tree (:size tree)))

(defn tree-size [tree]
  (if (number? tree)
    tree
    (let [children (map (fn [[k v]] [k (tree-size v)]) tree)
          new-tree (into {} children)
          size (sum (map get-size (vals new-tree)))]
      (assoc new-tree :size size))))

(defn find-directories [tree]
  (let [size (:size tree)]
    (if (nil? size)
      []
      (cons size (mapcat find-directories (vals tree))))))

(defn part1 [input]
  (let [tree (last (reduce build-tree ["/" {}] input))
        tree (tree-size tree)]
    (sum (filter #(< %1 100000) (find-directories (get tree "/"))))))

(defn part2 [input]
  (let [tree (last (reduce build-tree ["/" {}] input))
        tree (tree-size tree)
        dir-sizes (find-directories (get tree "/"))
        used-space (:size tree)
        remaining-space (- total-space used-space)
        must-delete (- needed-space remaining-space)]
    (->> dir-sizes
         (sort)
         (filter #(>= %1 must-delete))
         (first))))
