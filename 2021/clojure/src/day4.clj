; jack in ^alt c ^alt j
; eval file ^alt c enter

(ns day4
  (:require [clojure.java.io :as io]))

(defn run [opts]
  (println "Hello, world!"))

(comment
  (slurp (io/resource "day4.txt")))
  
  