(require '[clojure.pprint :refer [pprint]])
(require '[clojure.string :as str])
(require '[clojure.set :as set])

(defn split 
 [sep str]
 (str/split str sep))

(defn contains
 [coll val]
 (some (fn [colval] (= val colval)) coll))

(defn count-is [c col] (= c (count col)))

(defn on-off-blinking-segments
 [segments on off blinking]
 (let [off-segments (set/difference #{ "a", "b", "c", "d", "e", "f", "g" }, segments)]
  (let [new-blinking 
   (set/union 
    blinking
    (set/intersection segments off)
    (set/intersection off-segments on)
   )]
   [ (set/difference (set/union segments on) new-blinking) (set/difference (set/union off-segments off) new-blinking) new-blinking ]
  )
 )
)
(defn on-off-blinking-displays
 [displays ofb]
 (if (= (count displays) 0)
  ofb
  (on-off-blinking-displays 
   (pop displays)
   (on-off-blinking-segments
    (set (str/split (peek displays) #""))
    (get ofb 0)
    (get ofb 1)
    (get ofb 2)
   )
  )
 )
)
(defn blinken-code 
 [needle ofb]
 (if (= (count ofb) 0) 
  ""
  (str/join "" [
   (blinken-code needle (pop ofb))
   (if (contains (get (peek ofb) 0) needle)
    "1"
    (if (contains (get (peek ofb) 1) needle)
     "0"
     "?"
    )
   )
  ])
 )
)
(defn correction-map
 [displays]
 (let [ofb [
   (on-off-blinking-displays (filterv (partial count-is 2) displays) [ #{} #{} #{} ])
   (on-off-blinking-displays (filterv (partial count-is 3) displays) [ #{} #{} #{} ])
   (on-off-blinking-displays (filterv (partial count-is 4) displays) [ #{} #{} #{} ])
   (on-off-blinking-displays (filterv (partial count-is 5) displays) [ #{} #{} #{} ])
   (on-off-blinking-displays (filterv (partial count-is 6) displays) [ #{} #{} #{} ])
  ]]
  (let [blinken-rosetta (hash-map 
    "01011" "a"
    "001?1" "b"
    "111??" "c"
    "0011?" "d"
    "000??" "e"
    "111?1" "f"
    "00011" "g"
   )]
   (hash-map 
    "a" (get blinken-rosetta (blinken-code "a" ofb))
    "b" (get blinken-rosetta (blinken-code "b" ofb))
    "c" (get blinken-rosetta (blinken-code "c" ofb))
    "d" (get blinken-rosetta (blinken-code "d" ofb))
    "e" (get blinken-rosetta (blinken-code "e" ofb))
    "f" (get blinken-rosetta (blinken-code "f" ofb))
    "g" (get blinken-rosetta (blinken-code "g" ofb))
   )
  )
 )
)
(defn correct-display
 [correction-map display]
 (str/join "" (sort (map (partial get correction-map) display)))
)
(defn read-displays
 [observations-string displays-string]
 (let [
   observations (split #" " observations-string)
   displays (split #" " displays-string)
 ]
  (let [
    using-map (correction-map observations)
    digit-map (hash-map
     "abcefg" "0"
     "cf" "1"
     "acdeg" "2"
     "acdfg" "3"
     "bcdf" "4"
     "abdfg" "5"
     "abdefg" "6"
     "acf" "7"
     "abcdefg" "8"
     "abcdfg" "9"
    )]
   (Integer. (str/join "" (map (partial get digit-map) (map (partial correct-display using-map) (map (partial split #"") displays)))))
  )
 )
)
(->>
 (line-seq (java.io.BufferedReader. *in*))
 (map (partial split #" \| "))
 (map (partial apply read-displays))
 (reduce +)
 (pprint)
)