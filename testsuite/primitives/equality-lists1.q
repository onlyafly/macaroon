; This tests that equality functions even where node metadata (like line number) are different

(def a '(1))
(def b '(1))
(def c '(1))(def d '(1)) ; These are intentionally on the same line

(list
    (= a b)
    (= c d))

