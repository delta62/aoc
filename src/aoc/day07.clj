(ns aoc.day07
  (:require [aoc.util :refer [sum]]
            [clojure.string :as string]))

(def total-space 70000000)
(def needed-space 30000000)

(defn parse-cd
  "Parses a path into a :cd command object"
  [path]
  {:command :cd, :path path})

(defn parse-ls-size
  "Given an entry's `ls` output, return the entry's size. Gives 0 for
   directories."
  [size]
  (if (= "dir" size) 0 (parse-long size)))

(defn parse-entry
  "Adds an `ls` entry into a hash of name/size pairs"
  [state entry]
  (let [[size name] (string/split entry #"\s+")
        size (parse-ls-size size)]
    (assoc state name size)))

(defn parse-ls
  "Parses output of an `ls` command into a :ls command object containing a hash
   of name/size pairs"
  [output]
  (let [entries (string/split-lines output)
        files (reduce parse-entry {} entries)]
    {:command :ls, :entries files}))

(defn parse-cmd
  "Parse a command's input & output into a command object"
  [line]
  (let [[cmd output] (string/split line #"\s+" 2)]
    (case cmd
      "cd" (parse-cd output)
      "ls" (parse-ls output))))

(defn parse [s]
  (->> (string/split s #"\$")
       (rest)
       (map string/trim)
       (map parse-cmd)))

(defn update-path
  "Given an existing path, calculates what the new path should be after running
   `cd`"
  [path new-path]
  (case new-path
    "/" "/"
    ".." (string/replace path #"\/[^\/]+\/[^\/]*$" "/")
    (str (string/join "" [path new-path]) "/")))

(defn path-sequence
  "Splits a path into a sequence of components"
  [path]
  (->> (string/split path #"\/")
       (rest)
       (cons "/")))

(defn update-tree
  "Return a new tree with the item deeply nested at `path` replaced with
   `value`"
  [tree path value]
  (update-in tree (path-sequence path) (constantly value)))

(defn add-to-tree
  "Returns a new `[path, tree]` pair with the path reflecting any cwd changes,
   and the tree reflecting any `ls` output from the given command"
  [[path tree] command]
  (case (:command command)
    :cd (let [new-path (update-path path (:path command))]
          [new-path tree])
    :ls [path (update-tree tree path (:entries command))]))

(defn get-size
  "Return the cumulative size of a tree"
  [tree]
  (if (number? tree) tree (:size tree)))

(defn tree-size
  "Calculate the cumulative size of a tree. When the tree is a number, that
   number is returned. Otherwise, a new tree is returned with a `:size`
   property added."
  [tree]
  (if (number? tree)
    tree
    (let [children (map (fn [[k v]] [k (tree-size v)]) tree)
          new-tree (into {} children)
          size (sum (map get-size (vals new-tree)))]
      (assoc new-tree :size size))))

(defn find-directories
  "Returns a sequence of all the directory sizes contained in a tree"
  [tree]
  (let [size (:size tree)]
    (if (nil? size)
      []
      (->> (vals tree) (mapcat find-directories) (cons size)))))

(defn commands-to-tree
  "Converts a series of command objects into a tree data structure, and adds a
   `:size` field to each directory with its cumulative size"
  [commands]
  (->> (reduce add-to-tree ["/" {}] commands)
       (last)
       (tree-size)))

(defn part1 [input]
  (->> (commands-to-tree input)
       (find-directories)
       (filter #(< %1 100000))
       (sum)))

(defn part2 [input]
  (let [tree (commands-to-tree input)
        dir-sizes (find-directories tree)
        used-space (:size tree)
        remaining-space (- total-space used-space)
        must-delete (- needed-space remaining-space)]
    (->> dir-sizes
         (sort)
         (filter #(>= %1 must-delete))
         (first))))
