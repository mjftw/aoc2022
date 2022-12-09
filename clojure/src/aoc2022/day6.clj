(ns aoc2022.day6)

(defn unique-run-position [seq run-len]
  (let [indexed-windows (map-indexed list (partition run-len 1 seq))
        window-unique-sizes (map (fn [[idx items]] [idx (count (set items))]) indexed-windows)
        packet-starts (map (fn [[idx _]] idx) (filter (fn [[_ size]] (== size run-len)) window-unique-sizes))
        packet-ends (map #(+ % run-len) packet-starts)]
    (first packet-ends)))

(defn solve [input-path]
  (let [input (slurp input-path)]
    (println (str "Day 6 part 1 answer: " (unique-run-position input 4)))
    (println (str "Day 6 part 2 answer: " (unique-run-position input 14)))))
