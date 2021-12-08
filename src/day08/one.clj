(require '[clojure.pprint :refer [pprint]])
(require '[clojure.string :as str])

(defn split 
 [sep str]
 (str/split str sep))

(defn contains
 [coll val]
 (some (fn [colval] (= val colval)) coll))

(->>
 (line-seq (java.io.BufferedReader. *in*))
 (map (partial split #" \| "))
 (map last)
 (map (partial split #" "))
 (flatten)
 (map count)
 (filter (partial contains [2,3,4,7]))
 (count)
 (pprint)
)