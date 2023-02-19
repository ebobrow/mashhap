#!/usr/bin/racket

#lang racket

(require "mashhap.rkt" benchmark plot)

(define results
  (run-benchmarks
   (list 'bench)
   (list (range 10 1000 10))
   (lambda (_bench len)
     (define map (mashhap-new 5))
     (for [(i (in-range len))]
       (set!-map map (make-string i) i)))
   #:extract-time 'delta-time))

(plot-new-window? #t)

(parameterize ([plot-x-ticks no-ticks])
  (plot
   #:title "bench"
   #:x-label "size"
   #:y-label "normalized time"
   (render-benchmark-alts
    ; default options
    (list 10)
    results)))
