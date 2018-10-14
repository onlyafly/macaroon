(load "examples/prelude.q")

(defn evaluate (exp env)
    (if (atom? exp)
        (if (symbol? exp)
            (lookup exp env)
            exp)
        (case (first exp)
            TODO
            (else TODO))))