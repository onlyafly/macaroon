;; This tests a recursive function with a tail call. This would
;; likely cause a stack overflow without tail call optimization.

(def f (fn (n)
    (if (< n 10000)
        (f (+ n 1))
        n)))

(f 0)