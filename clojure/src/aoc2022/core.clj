(ns aoc2022.core
  (:require [aoc2022.day6 :as day6]))

(defn -main
  [& args]
  (let [day6-input (first args)]
    (day6/solve day6-input)))
