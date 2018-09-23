(def len2
  (fn (xs)
    (if (= xs '())
      0
      (+ 1 (len2 (rest xs))))))

(def n (now))

(len2 n)
