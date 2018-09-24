(def f (fn (n)
    (if (< n 10)
        (f (+ n 1))
        n)))

(f 0)