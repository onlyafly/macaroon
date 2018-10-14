(load "examples/prelude.q")
(load "examples/qtest.q")

(defn evaluate (e env)
    (if (atom? e)
        (cond
            (symbol? e) (lookup e env)
            (or (number? e) (string? e) (char? e) (boolean? e)) e
            else (panic (concat "Cannot eval: " (str e))))
        (case (first exp)
            TODO
            (else TODO))))

(defqtest "Evaluate atom"
  (qt=
    (evaluate 1 ())
    1))

(qtest-start)