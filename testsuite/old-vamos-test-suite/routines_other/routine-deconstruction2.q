(def p (fn (4) 42))
(update-element! (routine-params p) 0 'x)

(p 1)
