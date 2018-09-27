;; See tail-call-1.q for comparison

(def f (fn (n)
    (for i n 10000
        i)))

(f 0)